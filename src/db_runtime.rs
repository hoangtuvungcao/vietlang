//! Unified async database runtime for VietLang.
//! Supports: PostgreSQL, MySQL, SQLite, MongoDB, Redis, ClickHouse, Cassandra/ScyllaDB, Elasticsearch.

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex, OnceLock,
    },
    time::Duration,
};

use serde_json::Value as JsonValue;
use sqlx::{
    mysql::{MySqlPool, MySqlPoolOptions, MySqlRow},
    postgres::{PgPool, PgPoolOptions, PgRow},
    Column, Row,
};

use crate::{
    error::{VietError, VietResult},
    interpreter::value::Value,
};

// ---------------------------------------------------------------------------
// Connection handle registry
// ---------------------------------------------------------------------------

#[derive(Clone)]
enum Pool {
    Postgres(PgPool),
    MySql(MySqlPool),
    Mongo(mongodb::Client, String /* db_name */),
    Redis(redis::Client),
    ClickHouse(ClickHousePool),
    Cassandra(std::sync::Arc<scylla::Session>),
    Elastic(elasticsearch::Elasticsearch),
}

#[derive(Clone)]
struct ClickHousePool {
    url: String,
    database: String,
    user: String,
    password: String,
    client: reqwest::Client,
}

static POOLS: OnceLock<Mutex<HashMap<usize, Pool>>> = OnceLock::new();
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

fn runtime(line: usize, col: usize) -> VietResult<&'static tokio::runtime::Runtime> {
    if let Some(rt) = RUNTIME.get() {
        return Ok(rt);
    }
    let built = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("vietlang-db")
        .build()
        .map_err(|e| {
            VietError::runtime_error(format!("Cannot start async database runtime: {}", e), line, col)
        })?;
    let _ = RUNTIME.set(built);
    Ok(RUNTIME.get().expect("database runtime initialized"))
}

fn register_pool(pool: Pool) -> usize {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    POOLS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .expect("pool registry lock")
        .insert(id, pool);
    id
}

fn with_pool<T>(id: usize, line: usize, col: usize, op: impl FnOnce(&Pool) -> VietResult<T>) -> VietResult<T> {
    let pool = POOLS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .map_err(|_| VietError::runtime_error("Database pool registry lock is poisoned".into(), line, col))?
        .get(&id)
        .cloned()
        .ok_or_else(|| {
            VietError::runtime_error(format!("Database pool #{} is closed or unknown", id), line, col)
        })?;
    op(&pool)
}

fn make_handle(id: usize, driver: &str, extra: Vec<(&str, Value)>) -> Value {
    let mut fields = HashMap::new();
    fields.insert("id".into(), Value::Int(id as i64));
    fields.insert("driver".into(), Value::String(driver.into()));
    for (k, v) in extra {
        fields.insert(k.to_string(), v);
    }
    Value::Struct { type_name: "DbPool".into(), fields }
}

fn pool_id(value: Option<&Value>, line: usize, col: usize) -> VietResult<usize> {
    match value {
        Some(Value::Int(id)) => usize::try_from(*id)
            .map_err(|_| VietError::type_error("Invalid pool id".into(), line, col)),
        Some(Value::Struct { fields, .. }) => match fields.get("id") {
            Some(Value::Int(id)) => usize::try_from(*id)
                .map_err(|_| VietError::type_error("Invalid pool id".into(), line, col)),
            _ => Err(VietError::type_error("Pool handle has no id".into(), line, col)),
        },
        _ => Err(VietError::type_error("Expected database pool handle".into(), line, col)),
    }
}

// ---------------------------------------------------------------------------
// SQL helpers (PostgreSQL + MySQL)
// ---------------------------------------------------------------------------

pub fn connect(args: &[Value], driver: &str, line: usize, col: usize) -> VietResult<Value> {
    let dsn = match args.first() {
        Some(Value::String(v)) => v,
        _ => return Err(VietError::type_error(format!("{}_connect() expects a DSN string", driver), line, col)),
    };
    let (max, min, timeout) = pool_config(args.get(1));
    let async_runtime = runtime(line, col)?;
    let pool = match driver {
        "postgres" => Pool::Postgres(
            async_runtime
                .block_on(async {
                    PgPoolOptions::new()
                        .max_connections(max)
                        .min_connections(min)
                        .acquire_timeout(timeout)
                        .connect_lazy(dsn)
                })
                .map_err(|e| VietError::runtime_error(format!("Invalid PostgreSQL DSN: {}", e), line, col))?,
        ),
        "mysql" => Pool::MySql(
            async_runtime
                .block_on(async {
                    MySqlPoolOptions::new()
                        .max_connections(max)
                        .min_connections(min)
                        .acquire_timeout(timeout)
                        .connect_lazy(dsn)
                })
                .map_err(|e| VietError::runtime_error(format!("Invalid MySQL DSN: {}", e), line, col))?,
        ),
        _ => return Err(VietError::runtime_error("Unsupported SQL driver".into(), line, col)),
    };
    let id = register_pool(pool);
    Ok(make_handle(id, driver, vec![
        ("max_connections", Value::Int(max as i64)),
        ("async", Value::Bool(true)),
    ]))
}

pub fn execute(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let (id, sql, params) = operation_args(args, line, col)?;
    with_pool(id, line, col, |pool| match pool {
        Pool::Postgres(pool) => {
            let mut query = sqlx::query(sql);
            for v in params { query = bind_postgres(query, v); }
            runtime(line, col)?
                .block_on(query.execute(pool))
                .map(|r| Value::Int(r.rows_affected() as i64))
                .map_err(|e| db_error("PostgreSQL execute", e, line, col))
        }
        Pool::MySql(pool) => {
            let mut query = sqlx::query(sql);
            for v in params { query = bind_mysql(query, v); }
            runtime(line, col)?
                .block_on(query.execute(pool))
                .map(|r| Value::Int(r.rows_affected() as i64))
                .map_err(|e| db_error("MySQL execute", e, line, col))
        }
        _ => Err(VietError::runtime_error("execute() only available for SQL pools".into(), line, col)),
    })
}

pub fn query(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let (id, sql, params) = operation_args(args, line, col)?;
    with_pool(id, line, col, |pool| match pool {
        Pool::Postgres(pool) => {
            let mut query = sqlx::query(sql);
            for v in params { query = bind_postgres(query, v); }
            runtime(line, col)?
                .block_on(query.fetch_all(pool))
                .map(|rows| Value::Array(rows.iter().map(pg_row).collect()))
                .map_err(|e| db_error("PostgreSQL query", e, line, col))
        }
        Pool::MySql(pool) => {
            let mut query = sqlx::query(sql);
            for v in params { query = bind_mysql(query, v); }
            runtime(line, col)?
                .block_on(query.fetch_all(pool))
                .map(|rows| Value::Array(rows.iter().map(mysql_row).collect()))
                .map_err(|e| db_error("MySQL query", e, line, col))
        }
        _ => Err(VietError::runtime_error("query() only available for SQL pools".into(), line, col)),
    })
}

pub fn ping(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    with_pool(id, line, col, |pool| {
        let result = match pool {
            Pool::Postgres(pool) => runtime(line, col)?.block_on(sqlx::query("SELECT 1").execute(pool)).map(|_| ()),
            Pool::MySql(pool) => runtime(line, col)?.block_on(sqlx::query("SELECT 1").execute(pool)).map(|_| ()),
            Pool::Redis(client) => {
                runtime(line, col)?.block_on(async {
                    let mut conn = client.get_multiplexed_async_connection().await?;
                    redis::cmd("PING").query_async::<()>(&mut conn).await
                }).map_err(|e: redis::RedisError| sqlx::Error::Protocol(e.to_string()))
            }
            Pool::Mongo(client, _) => {
                runtime(line, col)?.block_on(async {
                    client.database("admin")
                        .run_command(mongodb::bson::doc! {"ping": 1})
                        .await
                }).map(|_| ()).map_err(|e| sqlx::Error::Protocol(e.to_string()))
            }
            _ => return Ok(Value::Bool(true)),
        };
        result.map(|_| Value::Bool(true)).map_err(|e| db_error("ping", e, line, col))
    })
}

pub fn close(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let pool = POOLS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .map_err(|_| VietError::runtime_error("Database pool registry lock is poisoned".into(), line, col))?
        .remove(&id);
    if let Some(pool) = pool {
        match pool {
            Pool::Postgres(p) => runtime(line, col)?.block_on(p.close()),
            Pool::MySql(p) => runtime(line, col)?.block_on(p.close()),
            _ => {} // Redis, Mongo, etc. are dropped implicitly
        }
        Ok(Value::Bool(true))
    } else {
        Ok(Value::Bool(false))
    }
}

pub fn migration_lock(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let key = match args.get(1) { Some(Value::Int(v)) => *v, _ => 8_642_091 };
    with_pool(id, line, col, |pool| match pool {
        Pool::Postgres(pool) => runtime(line, col)?
            .block_on(sqlx::query("SELECT pg_advisory_lock($1)").bind(key).execute(pool))
            .map(|_| Value::Bool(true))
            .map_err(|e| db_error("PostgreSQL migration lock", e, line, col)),
        Pool::MySql(pool) => runtime(line, col)?
            .block_on(sqlx::query("SELECT GET_LOCK(?, 30)").bind(format!("vietlang-migration-{key}")).execute(pool))
            .map(|_| Value::Bool(true))
            .map_err(|e| db_error("MySQL migration lock", e, line, col)),
        _ => Err(VietError::runtime_error("migration_lock not supported for this driver".into(), line, col)),
    })
}

// ---------------------------------------------------------------------------
// MongoDB
// ---------------------------------------------------------------------------

/// Connect to MongoDB: args = [uri_string, db_name]
pub fn mongo_connect(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let uri = match args.first() {
        Some(Value::String(s)) => s.clone(),
        _ => return Err(VietError::type_error("mongo_connect() expects uri string".into(), line, col)),
    };
    let db_name = match args.get(1) {
        Some(Value::String(s)) => s.clone(),
        _ => "vietlang".to_string(),
    };
    let rt = runtime(line, col)?;
    let client = rt.block_on(async {
        let options = mongodb::options::ClientOptions::parse(&uri).await?;
        mongodb::Client::with_options(options)
    }).map_err(|e| VietError::runtime_error(format!("MongoDB connect failed: {}", e), line, col))?;

    let id = register_pool(Pool::Mongo(client, db_name.clone()));
    Ok(make_handle(id, "mongodb", vec![("database", Value::String(db_name))]))
}

pub fn mongo_find(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    use mongodb::bson::Document;
    use futures_util::TryStreamExt;

    let id = pool_id(args.first(), line, col)?;
    let collection = str_arg(args, 1, "collection name", line, col)?;
    let filter_val = args.get(2).cloned().unwrap_or(Value::None);
    let options_val = args.get(3).cloned().unwrap_or(Value::None);

    with_pool(id, line, col, |pool| {
        let Pool::Mongo(client, db_name) = pool else {
            return Err(VietError::runtime_error("Not a MongoDB connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let filter = value_to_bson_doc(&filter_val);
        let mut find_options = mongodb::options::FindOptions::default();
        if let Value::Struct { fields, .. } = &options_val {
            if let Some(Value::Int(n)) = fields.get("limit") { find_options.limit = Some(*n); }
            if let Some(Value::Int(n)) = fields.get("skip") { find_options.skip = Some(*n as u64); }
        }
        let docs: Vec<Value> = rt.block_on(async {
            let coll: mongodb::Collection<Document> = client.database(db_name).collection(collection);
            let mut cursor = coll.find(filter).with_options(find_options).await?;
            let mut results = Vec::new();
            while let Some(doc) = cursor.try_next().await? {
                results.push(bson_doc_to_value(&doc));
            }
            Ok::<Vec<Value>, mongodb::error::Error>(results)
        }).map_err(|e| VietError::runtime_error(format!("MongoDB find failed: {}", e), line, col))?;
        Ok(Value::Array(docs))
    })
}

pub fn mongo_find_one(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    use mongodb::bson::Document;

    let id = pool_id(args.first(), line, col)?;
    let collection = str_arg(args, 1, "collection name", line, col)?;
    let filter_val = args.get(2).cloned().unwrap_or(Value::None);

    with_pool(id, line, col, |pool| {
        let Pool::Mongo(client, db_name) = pool else {
            return Err(VietError::runtime_error("Not a MongoDB connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let filter = value_to_bson_doc(&filter_val);
        let doc_opt: Option<Document> = rt.block_on(async {
            let coll: mongodb::Collection<Document> = client.database(db_name).collection(collection);
            coll.find_one(filter).await
        }).map_err(|e| VietError::runtime_error(format!("MongoDB find_one failed: {}", e), line, col))?;
        Ok(doc_opt.map(|d| bson_doc_to_value(&d)).unwrap_or(Value::None))
    })
}

pub fn mongo_insert_one(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    use mongodb::bson::Document;

    let id = pool_id(args.first(), line, col)?;
    let collection = str_arg(args, 1, "collection name", line, col)?;
    let doc_val = args.get(2).cloned().unwrap_or(Value::None);

    with_pool(id, line, col, |pool| {
        let Pool::Mongo(client, db_name) = pool else {
            return Err(VietError::runtime_error("Not a MongoDB connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let doc = value_to_bson_doc(&doc_val);
        let result = rt.block_on(async {
            let coll: mongodb::Collection<Document> = client.database(db_name).collection(collection);
            coll.insert_one(doc).await
        }).map_err(|e| VietError::runtime_error(format!("MongoDB insert_one failed: {}", e), line, col))?;
        let inserted_id = result.inserted_id.to_string();
        Ok(Value::String(inserted_id))
    })
}

pub fn mongo_insert_many(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    use mongodb::bson::Document;

    let id = pool_id(args.first(), line, col)?;
    let collection = str_arg(args, 1, "collection name", line, col)?;
    let docs_val = match args.get(2) {
        Some(Value::Array(arr)) => arr.clone(),
        _ => vec![],
    };

    with_pool(id, line, col, |pool| {
        let Pool::Mongo(client, db_name) = pool else {
            return Err(VietError::runtime_error("Not a MongoDB connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let docs: Vec<Document> = docs_val.iter().map(|v| value_to_bson_doc(v)).collect();
        let result = rt.block_on(async {
            let coll: mongodb::Collection<Document> = client.database(db_name).collection(collection);
            coll.insert_many(docs).await
        }).map_err(|e| VietError::runtime_error(format!("MongoDB insert_many failed: {}", e), line, col))?;
        Ok(Value::Int(result.inserted_ids.len() as i64))
    })
}

pub fn mongo_update_one(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    use mongodb::bson::Document;

    let id = pool_id(args.first(), line, col)?;
    let collection = str_arg(args, 1, "collection name", line, col)?;
    let filter_val = args.get(2).cloned().unwrap_or(Value::None);
    let update_val = args.get(3).cloned().unwrap_or(Value::None);

    with_pool(id, line, col, |pool| {
        let Pool::Mongo(client, db_name) = pool else {
            return Err(VietError::runtime_error("Not a MongoDB connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let filter = value_to_bson_doc(&filter_val);
        let update = value_to_bson_doc(&update_val);
        let result = rt.block_on(async {
            let coll: mongodb::Collection<Document> = client.database(db_name).collection(collection);
            coll.update_one(filter, update).await
        }).map_err(|e| VietError::runtime_error(format!("MongoDB update_one failed: {}", e), line, col))?;
        Ok(Value::Int(result.modified_count as i64))
    })
}

pub fn mongo_update_many(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    use mongodb::bson::Document;

    let id = pool_id(args.first(), line, col)?;
    let collection = str_arg(args, 1, "collection name", line, col)?;
    let filter_val = args.get(2).cloned().unwrap_or(Value::None);
    let update_val = args.get(3).cloned().unwrap_or(Value::None);

    with_pool(id, line, col, |pool| {
        let Pool::Mongo(client, db_name) = pool else {
            return Err(VietError::runtime_error("Not a MongoDB connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let filter = value_to_bson_doc(&filter_val);
        let update = value_to_bson_doc(&update_val);
        let result = rt.block_on(async {
            let coll: mongodb::Collection<Document> = client.database(db_name).collection(collection);
            coll.update_many(filter, update).await
        }).map_err(|e| VietError::runtime_error(format!("MongoDB update_many failed: {}", e), line, col))?;
        Ok(Value::Int(result.modified_count as i64))
    })
}

pub fn mongo_delete_one(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    use mongodb::bson::Document;

    let id = pool_id(args.first(), line, col)?;
    let collection = str_arg(args, 1, "collection name", line, col)?;
    let filter_val = args.get(2).cloned().unwrap_or(Value::None);

    with_pool(id, line, col, |pool| {
        let Pool::Mongo(client, db_name) = pool else {
            return Err(VietError::runtime_error("Not a MongoDB connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let filter = value_to_bson_doc(&filter_val);
        let result = rt.block_on(async {
            let coll: mongodb::Collection<Document> = client.database(db_name).collection(collection);
            coll.delete_one(filter).await
        }).map_err(|e| VietError::runtime_error(format!("MongoDB delete_one failed: {}", e), line, col))?;
        Ok(Value::Int(result.deleted_count as i64))
    })
}

pub fn mongo_delete_many(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    use mongodb::bson::Document;

    let id = pool_id(args.first(), line, col)?;
    let collection = str_arg(args, 1, "collection name", line, col)?;
    let filter_val = args.get(2).cloned().unwrap_or(Value::None);

    with_pool(id, line, col, |pool| {
        let Pool::Mongo(client, db_name) = pool else {
            return Err(VietError::runtime_error("Not a MongoDB connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let filter = value_to_bson_doc(&filter_val);
        let result = rt.block_on(async {
            let coll: mongodb::Collection<Document> = client.database(db_name).collection(collection);
            coll.delete_many(filter).await
        }).map_err(|e| VietError::runtime_error(format!("MongoDB delete_many failed: {}", e), line, col))?;
        Ok(Value::Int(result.deleted_count as i64))
    })
}

pub fn mongo_count(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    use mongodb::bson::Document;

    let id = pool_id(args.first(), line, col)?;
    let collection = str_arg(args, 1, "collection name", line, col)?;
    let filter_val = args.get(2).cloned().unwrap_or(Value::None);

    with_pool(id, line, col, |pool| {
        let Pool::Mongo(client, db_name) = pool else {
            return Err(VietError::runtime_error("Not a MongoDB connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let filter = value_to_bson_doc(&filter_val);
        let count = rt.block_on(async {
            let coll: mongodb::Collection<Document> = client.database(db_name).collection(collection);
            coll.count_documents(filter).await
        }).map_err(|e| VietError::runtime_error(format!("MongoDB count failed: {}", e), line, col))?;
        Ok(Value::Int(count as i64))
    })
}

pub fn mongo_aggregate(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    use mongodb::bson::Document;
    use futures_util::TryStreamExt;

    let id = pool_id(args.first(), line, col)?;
    let collection = str_arg(args, 1, "collection name", line, col)?;
    let pipeline_val = match args.get(2) {
        Some(Value::Array(arr)) => arr.clone(),
        _ => vec![],
    };

    with_pool(id, line, col, |pool| {
        let Pool::Mongo(client, db_name) = pool else {
            return Err(VietError::runtime_error("Not a MongoDB connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let pipeline: Vec<Document> = pipeline_val.iter().map(|v| value_to_bson_doc(v)).collect();
        let docs: Vec<Value> = rt.block_on(async {
            let coll: mongodb::Collection<Document> = client.database(db_name).collection(collection);
            let mut cursor = coll.aggregate(pipeline).await?;
            let mut results = Vec::new();
            while let Some(doc) = cursor.try_next().await? {
                results.push(bson_doc_to_value(&doc));
            }
            Ok::<Vec<Value>, mongodb::error::Error>(results)
        }).map_err(|e| VietError::runtime_error(format!("MongoDB aggregate failed: {}", e), line, col))?;
        Ok(Value::Array(docs))
    })
}

pub fn mongo_create_index(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    use mongodb::bson::Document;
    use mongodb::IndexModel;

    let id = pool_id(args.first(), line, col)?;
    let collection = str_arg(args, 1, "collection name", line, col)?;
    let keys_val = args.get(2).cloned().unwrap_or(Value::None);
    let unique = match args.get(3) { Some(Value::Bool(b)) => *b, _ => false };

    with_pool(id, line, col, |pool| {
        let Pool::Mongo(client, db_name) = pool else {
            return Err(VietError::runtime_error("Not a MongoDB connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let keys = value_to_bson_doc(&keys_val);
        let options = mongodb::options::IndexOptions::builder()
            .unique(unique)
            .build();
        let model = IndexModel::builder().keys(keys).options(options).build();
        let name = rt.block_on(async {
            let coll: mongodb::Collection<Document> = client.database(db_name).collection(collection);
            coll.create_index(model).await
        }).map_err(|e| VietError::runtime_error(format!("MongoDB create_index failed: {}", e), line, col))?;
        Ok(Value::String(name.index_name))
    })
}

pub fn mongo_list_collections(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    use futures_util::TryStreamExt;

    let id = pool_id(args.first(), line, col)?;
    with_pool(id, line, col, |pool| {
        let Pool::Mongo(client, db_name) = pool else {
            return Err(VietError::runtime_error("Not a MongoDB connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let names: Vec<Value> = rt.block_on(async {
            let mut cursor = client.database(db_name).list_collection_names().await?;
            Ok::<Vec<String>, mongodb::error::Error>(cursor)
        }).map_err(|e| VietError::runtime_error(format!("MongoDB list_collections failed: {}", e), line, col))?
          .into_iter().map(Value::String).collect();
        Ok(Value::Array(names))
    })
}

pub fn mongo_drop_collection(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    use mongodb::bson::Document;

    let id = pool_id(args.first(), line, col)?;
    let collection = str_arg(args, 1, "collection name", line, col)?;
    with_pool(id, line, col, |pool| {
        let Pool::Mongo(client, db_name) = pool else {
            return Err(VietError::runtime_error("Not a MongoDB connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        rt.block_on(async {
            let coll: mongodb::Collection<Document> = client.database(db_name).collection(collection);
            coll.drop().await
        }).map_err(|e| VietError::runtime_error(format!("MongoDB drop_collection failed: {}", e), line, col))?;
        Ok(Value::Bool(true))
    })
}

// ---------------------------------------------------------------------------
// Redis
// ---------------------------------------------------------------------------

pub fn redis_connect(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let url = match args.first() {
        Some(Value::String(s)) => s.clone(),
        _ => "redis://localhost:6379".to_string(),
    };
    let client = redis::Client::open(url.as_str())
        .map_err(|e| VietError::runtime_error(format!("Redis connect failed: {}", e), line, col))?;
    let id = register_pool(Pool::Redis(client));
    Ok(make_handle(id, "redis", vec![("url", Value::String(url))]))
}

pub fn redis_cmd(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let command = str_arg(args, 1, "redis command", line, col)?;
    let cmd_args = match args.get(2) {
        Some(Value::Array(arr)) => arr.clone(),
        _ => vec![],
    };

    with_pool(id, line, col, |pool| {
        let Pool::Redis(client) = pool else {
            return Err(VietError::runtime_error("Not a Redis connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let result: redis::Value = rt.block_on(async {
            let mut conn = client.get_multiplexed_async_connection().await?;
            let mut cmd = redis::cmd(command);
            for arg in &cmd_args {
                match arg {
                    Value::String(s) => { cmd.arg(s); }
                    Value::Int(n) => { cmd.arg(*n); }
                    Value::Float(f) => { cmd.arg(*f); }
                    Value::Bool(b) => { cmd.arg(if *b { "1" } else { "0" }); }
                    _ => { cmd.arg(arg.to_string()); }
                }
            }
            cmd.query_async(&mut conn).await
        }).map_err(|e: redis::RedisError| VietError::runtime_error(format!("Redis command failed: {}", e), line, col))?;
        Ok(redis_value_to_viet(result))
    })
}

pub fn redis_set(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let key = str_arg(args, 1, "key", line, col)?;
    let value = args.get(2).cloned().unwrap_or(Value::None);
    let ttl = match args.get(3) { Some(Value::Int(n)) => *n, _ => 0 };
    with_pool(id, line, col, |pool| {
        let Pool::Redis(client) = pool else {
            return Err(VietError::runtime_error("Not a Redis connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let val_str = value_to_redis_string(&value);
        rt.block_on(async {
            let mut conn = client.get_multiplexed_async_connection().await?;
            if ttl > 0 {
                redis::cmd("SETEX").arg(key).arg(ttl).arg(&val_str).query_async::<()>(&mut conn).await
            } else {
                redis::cmd("SET").arg(key).arg(&val_str).query_async::<()>(&mut conn).await
            }
        }).map_err(|e: redis::RedisError| VietError::runtime_error(format!("Redis SET failed: {}", e), line, col))?;
        Ok(Value::Bool(true))
    })
}

pub fn redis_get(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let key = str_arg(args, 1, "key", line, col)?;
    with_pool(id, line, col, |pool| {
        let Pool::Redis(client) = pool else {
            return Err(VietError::runtime_error("Not a Redis connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let result: Option<String> = rt.block_on(async {
            let mut conn = client.get_multiplexed_async_connection().await?;
            redis::cmd("GET").arg(key).query_async(&mut conn).await
        }).map_err(|e: redis::RedisError| VietError::runtime_error(format!("Redis GET failed: {}", e), line, col))?;
        Ok(result.map(Value::String).unwrap_or(Value::None))
    })
}

pub fn redis_del(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let keys_val = args.get(1).cloned().unwrap_or(Value::None);
    with_pool(id, line, col, |pool| {
        let Pool::Redis(client) = pool else {
            return Err(VietError::runtime_error("Not a Redis connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let count: i64 = rt.block_on(async {
            let mut conn = client.get_multiplexed_async_connection().await?;
            let mut cmd = redis::cmd("DEL");
            match &keys_val {
                Value::Array(arr) => { for k in arr { cmd.arg(k.to_string()); } }
                Value::String(k) => { cmd.arg(k); }
                _ => {}
            }
            cmd.query_async(&mut conn).await
        }).map_err(|e: redis::RedisError| VietError::runtime_error(format!("Redis DEL failed: {}", e), line, col))?;
        Ok(Value::Int(count))
    })
}

pub fn redis_incr(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let key = str_arg(args, 1, "key", line, col)?;
    with_pool(id, line, col, |pool| {
        let Pool::Redis(client) = pool else {
            return Err(VietError::runtime_error("Not a Redis connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let count: i64 = rt.block_on(async {
            let mut conn = client.get_multiplexed_async_connection().await?;
            redis::cmd("INCR").arg(key).query_async(&mut conn).await
        }).map_err(|e: redis::RedisError| VietError::runtime_error(format!("Redis INCR failed: {}", e), line, col))?;
        Ok(Value::Int(count))
    })
}

pub fn redis_expire(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let key = str_arg(args, 1, "key", line, col)?;
    let ttl = match args.get(2) { Some(Value::Int(n)) => *n, _ => 0 };
    with_pool(id, line, col, |pool| {
        let Pool::Redis(client) = pool else {
            return Err(VietError::runtime_error("Not a Redis connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let ok: bool = rt.block_on(async {
            let mut conn = client.get_multiplexed_async_connection().await?;
            redis::cmd("EXPIRE").arg(key).arg(ttl).query_async(&mut conn).await
        }).map_err(|e: redis::RedisError| VietError::runtime_error(format!("Redis EXPIRE failed: {}", e), line, col))?;
        Ok(Value::Bool(ok))
    })
}

pub fn redis_ttl(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let key = str_arg(args, 1, "key", line, col)?;
    with_pool(id, line, col, |pool| {
        let Pool::Redis(client) = pool else {
            return Err(VietError::runtime_error("Not a Redis connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let ttl: i64 = rt.block_on(async {
            let mut conn = client.get_multiplexed_async_connection().await?;
            redis::cmd("TTL").arg(key).query_async(&mut conn).await
        }).map_err(|e: redis::RedisError| VietError::runtime_error(format!("Redis TTL failed: {}", e), line, col))?;
        Ok(Value::Int(ttl))
    })
}

pub fn redis_exists(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let key = str_arg(args, 1, "key", line, col)?;
    with_pool(id, line, col, |pool| {
        let Pool::Redis(client) = pool else {
            return Err(VietError::runtime_error("Not a Redis connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let count: i64 = rt.block_on(async {
            let mut conn = client.get_multiplexed_async_connection().await?;
            redis::cmd("EXISTS").arg(key).query_async(&mut conn).await
        }).map_err(|e: redis::RedisError| VietError::runtime_error(format!("Redis EXISTS failed: {}", e), line, col))?;
        Ok(Value::Bool(count > 0))
    })
}

pub fn redis_hset(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let key = str_arg(args, 1, "hash key", line, col)?;
    let field = str_arg(args, 2, "field", line, col)?;
    let value = args.get(3).cloned().unwrap_or(Value::None);
    with_pool(id, line, col, |pool| {
        let Pool::Redis(client) = pool else {
            return Err(VietError::runtime_error("Not a Redis connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let val_str = value_to_redis_string(&value);
        let n: i64 = rt.block_on(async {
            let mut conn = client.get_multiplexed_async_connection().await?;
            redis::cmd("HSET").arg(key).arg(field).arg(&val_str).query_async(&mut conn).await
        }).map_err(|e: redis::RedisError| VietError::runtime_error(format!("Redis HSET failed: {}", e), line, col))?;
        Ok(Value::Int(n))
    })
}

pub fn redis_hget(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let key = str_arg(args, 1, "hash key", line, col)?;
    let field = str_arg(args, 2, "field", line, col)?;
    with_pool(id, line, col, |pool| {
        let Pool::Redis(client) = pool else {
            return Err(VietError::runtime_error("Not a Redis connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let result: Option<String> = rt.block_on(async {
            let mut conn = client.get_multiplexed_async_connection().await?;
            redis::cmd("HGET").arg(key).arg(field).query_async(&mut conn).await
        }).map_err(|e: redis::RedisError| VietError::runtime_error(format!("Redis HGET failed: {}", e), line, col))?;
        Ok(result.map(Value::String).unwrap_or(Value::None))
    })
}

pub fn redis_lpush(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let key = str_arg(args, 1, "list key", line, col)?;
    let values = match args.get(2) { Some(Value::Array(a)) => a.clone(), _ => vec![] };
    with_pool(id, line, col, |pool| {
        let Pool::Redis(client) = pool else {
            return Err(VietError::runtime_error("Not a Redis connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let count: i64 = rt.block_on(async {
            let mut conn = client.get_multiplexed_async_connection().await?;
            let mut cmd = redis::cmd("LPUSH");
            cmd.arg(key);
            for v in &values { cmd.arg(value_to_redis_string(v)); }
            cmd.query_async(&mut conn).await
        }).map_err(|e: redis::RedisError| VietError::runtime_error(format!("Redis LPUSH failed: {}", e), line, col))?;
        Ok(Value::Int(count))
    })
}

pub fn redis_rpush(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let key = str_arg(args, 1, "list key", line, col)?;
    let values = match args.get(2) { Some(Value::Array(a)) => a.clone(), _ => vec![] };
    with_pool(id, line, col, |pool| {
        let Pool::Redis(client) = pool else {
            return Err(VietError::runtime_error("Not a Redis connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let count: i64 = rt.block_on(async {
            let mut conn = client.get_multiplexed_async_connection().await?;
            let mut cmd = redis::cmd("RPUSH");
            cmd.arg(key);
            for v in &values { cmd.arg(value_to_redis_string(v)); }
            cmd.query_async(&mut conn).await
        }).map_err(|e: redis::RedisError| VietError::runtime_error(format!("Redis RPUSH failed: {}", e), line, col))?;
        Ok(Value::Int(count))
    })
}

pub fn redis_lrange(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let key = str_arg(args, 1, "list key", line, col)?;
    let start = match args.get(2) { Some(Value::Int(n)) => *n, _ => 0 };
    let stop = match args.get(3) { Some(Value::Int(n)) => *n, _ => -1 };
    with_pool(id, line, col, |pool| {
        let Pool::Redis(client) = pool else {
            return Err(VietError::runtime_error("Not a Redis connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let items: Vec<String> = rt.block_on(async {
            let mut conn = client.get_multiplexed_async_connection().await?;
            redis::cmd("LRANGE").arg(key).arg(start).arg(stop).query_async(&mut conn).await
        }).map_err(|e: redis::RedisError| VietError::runtime_error(format!("Redis LRANGE failed: {}", e), line, col))?;
        Ok(Value::Array(items.into_iter().map(Value::String).collect()))
    })
}

pub fn redis_publish(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let channel = str_arg(args, 1, "channel", line, col)?;
    let message = str_arg(args, 2, "message", line, col)?;
    with_pool(id, line, col, |pool| {
        let Pool::Redis(client) = pool else {
            return Err(VietError::runtime_error("Not a Redis connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let count: i64 = rt.block_on(async {
            let mut conn = client.get_multiplexed_async_connection().await?;
            redis::cmd("PUBLISH").arg(channel).arg(message).query_async(&mut conn).await
        }).map_err(|e: redis::RedisError| VietError::runtime_error(format!("Redis PUBLISH failed: {}", e), line, col))?;
        Ok(Value::Int(count))
    })
}

// ---------------------------------------------------------------------------
// ClickHouse
// ---------------------------------------------------------------------------

pub fn clickhouse_connect(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let host = match args.first() { Some(Value::String(s)) => s.clone(), _ => "localhost".to_string() };
    let port = match args.get(1) { Some(Value::Int(n)) => *n as u16, _ => 8123 };
    let database = match args.get(2) { Some(Value::String(s)) => s.clone(), _ => "default".to_string() };
    let user = match args.get(3) { Some(Value::String(s)) => s.clone(), _ => "default".to_string() };
    let password = match args.get(4) { Some(Value::String(s)) => s.clone(), _ => String::new() };

    let url = format!("http://{}:{}", host, port);
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|error| {
            VietError::runtime_error(
                format!("Cannot initialize ClickHouse HTTP client: {}", error),
                line,
                col,
            )
        })?;

    let id = register_pool(Pool::ClickHouse(ClickHousePool {
        url,
        database: database.clone(),
        user,
        password,
        client,
    }));
    Ok(make_handle(id, "clickhouse", vec![
        ("host", Value::String(host)),
        ("database", Value::String(database)),
    ]))
}

pub fn clickhouse_query(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let sql = str_arg(args, 1, "SQL query", line, col)?;
    with_pool(id, line, col, |pool| {
        let Pool::ClickHouse(pool) = pool else {
            return Err(VietError::runtime_error("Not a ClickHouse connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let body = format!("{}\nFORMAT JSONEachRow", sql.trim_end_matches(';'));
        let response = rt
            .block_on(
                pool.client
                    .post(&pool.url)
                    .basic_auth(&pool.user, Some(&pool.password))
                    .query(&[("database", &pool.database)])
                    .body(body)
                    .send(),
            )
            .and_then(reqwest::Response::error_for_status)
            .map_err(|error| {
                VietError::runtime_error(
                    format!("ClickHouse query failed: {}", error),
                    line,
                    col,
                )
            })?;
        let response_body = rt.block_on(response.text()).map_err(|error| {
            VietError::runtime_error(
                format!("Cannot read ClickHouse response: {}", error),
                line,
                col,
            )
        })?;
        let mut rows = Vec::new();
        for row in response_body.lines().filter(|row| !row.trim().is_empty()) {
            let json: JsonValue = serde_json::from_str(row).map_err(|error| {
                VietError::runtime_error(
                    format!("Invalid ClickHouse JSONEachRow response: {}", error),
                    line,
                    col,
                )
            })?;
            rows.push(json_to_value(&json));
        }
        Ok(Value::Array(rows))
    })
}

pub fn clickhouse_execute(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let sql = str_arg(args, 1, "SQL", line, col)?;
    with_pool(id, line, col, |pool| {
        let Pool::ClickHouse(pool) = pool else {
            return Err(VietError::runtime_error("Not a ClickHouse connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        rt.block_on(
            pool.client
                .post(&pool.url)
                .basic_auth(&pool.user, Some(&pool.password))
                .query(&[("database", &pool.database)])
                .body(sql.to_string())
                .send(),
        )
        .and_then(reqwest::Response::error_for_status)
        .map_err(|error| {
            VietError::runtime_error(
                format!("ClickHouse execute failed: {}", error),
                line,
                col,
            )
        })?;
        Ok(Value::Bool(true))
    })
}

// ---------------------------------------------------------------------------
// Cassandra/ScyllaDB
// ---------------------------------------------------------------------------

pub fn cassandra_connect(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let hosts: Vec<String> = match args.first() {
        Some(Value::Array(arr)) => arr.iter().filter_map(|v| {
            if let Value::String(s) = v { Some(s.clone()) } else { None }
        }).collect(),
        _ => vec!["127.0.0.1:9042".to_string()],
    };
    let keyspace = match args.get(1) { Some(Value::String(s)) => s.clone(), _ => String::new() };

    let rt = runtime(line, col)?;
    let session = rt.block_on(async {
        let mut builder = scylla::SessionBuilder::new();
        for host in &hosts {
            builder = builder.known_node(host.as_str());
        }
        if !keyspace.is_empty() {
            builder = builder.use_keyspace(&keyspace, false);
        }
        builder.build().await
    }).map_err(|e| VietError::runtime_error(format!("Cassandra connect failed: {}", e), line, col))?;

    let session = std::sync::Arc::new(session);
    let id = register_pool(Pool::Cassandra(session));
    Ok(make_handle(id, "cassandra", vec![
        ("hosts", Value::Array(hosts.into_iter().map(Value::String).collect())),
    ]))
}

pub fn cassandra_query(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let cql = str_arg(args, 1, "CQL query", line, col)?;
    let params: Vec<Value> = match args.get(2) {
        Some(Value::Array(a)) => a.clone(),
        _ => vec![],
    };
    if !params.is_empty() {
        return Err(VietError::runtime_error(
            "Cassandra bound parameters are not supported yet; refusing to interpolate values into CQL"
                .into(),
            line,
            col,
        ));
    }

    with_pool(id, line, col, |pool| {
        let Pool::Cassandra(session) = pool else {
            return Err(VietError::runtime_error("Not a Cassandra connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let session = session.clone();
        let rows = rt.block_on(async {
            let result = session
                .query_unpaged(cql, ())
                .await
                .map_err(|error| error.to_string())?;
            let column_specs = result.col_specs().to_vec();
            let result_rows = result.rows().map_err(|error| error.to_string())?;
            let mut rows_out = Vec::new();
            for row in result_rows {
                let mut map = HashMap::new();
                for (i, col_spec) in column_specs.iter().enumerate() {
                    let val = match row.columns.get(i) {
                        Some(Some(cv)) => cassandra_cql_to_value(cv),
                        _ => Value::None,
                    };
                    map.insert(col_spec.name.to_string(), val);
                }
                rows_out.push(Value::Struct { type_name: "Map".into(), fields: map });
            }
            Ok::<Vec<Value>, String>(rows_out)
        }).map_err(|e| VietError::runtime_error(format!("Cassandra query failed: {}", e), line, col))?;
        Ok(Value::Array(rows))
    })
}

pub fn cassandra_execute(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let cql = str_arg(args, 1, "CQL statement", line, col)?;

    with_pool(id, line, col, |pool| {
        let Pool::Cassandra(session) = pool else {
            return Err(VietError::runtime_error("Not a Cassandra connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let session = session.clone();
        rt.block_on(async { session.query_unpaged(cql, ()).await })
            .map_err(|e| VietError::runtime_error(format!("Cassandra execute failed: {}", e), line, col))?;
        Ok(Value::Bool(true))
    })
}

// ---------------------------------------------------------------------------
// Elasticsearch
// ---------------------------------------------------------------------------

pub fn elastic_connect(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let url = match args.first() { Some(Value::String(s)) => s.clone(), _ => "http://localhost:9200".to_string() };
    let api_key = match args.get(1) { Some(Value::String(s)) => s.clone(), _ => String::new() };

    let mut transport_builder = elasticsearch::http::transport::TransportBuilder::new(
        elasticsearch::http::transport::SingleNodeConnectionPool::new(
            url.parse().map_err(|e| VietError::runtime_error(format!("Invalid Elasticsearch URL: {}", e), line, col))?
        )
    );
    if !api_key.is_empty() {
        transport_builder = transport_builder.auth(elasticsearch::auth::Credentials::ApiKey(
            String::new(), api_key
        ));
    }
    let transport = transport_builder.build()
        .map_err(|e| VietError::runtime_error(format!("Elasticsearch transport failed: {}", e), line, col))?;
    let client = elasticsearch::Elasticsearch::new(transport);

    let id = register_pool(Pool::Elastic(client));
    Ok(make_handle(id, "elasticsearch", vec![("url", Value::String(url))]))
}

pub fn elastic_index_doc(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let index = str_arg(args, 1, "index name", line, col)?;
    let doc_id = str_arg(args, 2, "document id", line, col)?;
    let doc_val = args.get(3).cloned().unwrap_or(Value::None);

    with_pool(id, line, col, |pool| {
        let Pool::Elastic(client) = pool else {
            return Err(VietError::runtime_error("Not an Elasticsearch connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let body = value_to_json(&doc_val);
        let response = rt.block_on(async {
            client.index(elasticsearch::IndexParts::IndexId(index, doc_id))
                .body(body)
                .send()
                .await
        }).map_err(|e| VietError::runtime_error(format!("Elasticsearch index failed: {}", e), line, col))?;
        Ok(Value::Bool(response.status_code().is_success()))
    })
}

pub fn elastic_get_doc(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let index = str_arg(args, 1, "index name", line, col)?;
    let doc_id = str_arg(args, 2, "document id", line, col)?;

    with_pool(id, line, col, |pool| {
        let Pool::Elastic(client) = pool else {
            return Err(VietError::runtime_error("Not an Elasticsearch connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let response = rt.block_on(async {
            client.get(elasticsearch::GetParts::IndexId(index, doc_id))
                .send()
                .await
        }).map_err(|e| VietError::runtime_error(format!("Elasticsearch get failed: {}", e), line, col))?;
        if response.status_code().as_u16() == 404 {
            return Ok(Value::None);
        }
        let json: JsonValue = rt.block_on(response.json())
            .map_err(|e| VietError::runtime_error(format!("Elasticsearch response parse failed: {}", e), line, col))?;
        Ok(json_to_value(&json))
    })
}

pub fn elastic_search(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let index = str_arg(args, 1, "index name", line, col)?;
    let body_val = args.get(2).cloned().unwrap_or(Value::None);

    with_pool(id, line, col, |pool| {
        let Pool::Elastic(client) = pool else {
            return Err(VietError::runtime_error("Not an Elasticsearch connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let body = value_to_json(&body_val);
        let response = rt.block_on(async {
            client.search(elasticsearch::SearchParts::Index(&[index]))
                .body(body)
                .send()
                .await
        }).map_err(|e| VietError::runtime_error(format!("Elasticsearch search failed: {}", e), line, col))?;
        let json: JsonValue = rt.block_on(response.json())
            .map_err(|e| VietError::runtime_error(format!("Elasticsearch response parse failed: {}", e), line, col))?;
        Ok(json_to_value(&json))
    })
}

pub fn elastic_delete_doc(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let index = str_arg(args, 1, "index name", line, col)?;
    let doc_id = str_arg(args, 2, "document id", line, col)?;

    with_pool(id, line, col, |pool| {
        let Pool::Elastic(client) = pool else {
            return Err(VietError::runtime_error("Not an Elasticsearch connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let response = rt.block_on(async {
            client.delete(elasticsearch::DeleteParts::IndexId(index, doc_id))
                .send()
                .await
        }).map_err(|e| VietError::runtime_error(format!("Elasticsearch delete failed: {}", e), line, col))?;
        Ok(Value::Bool(response.status_code().is_success()))
    })
}

pub fn elastic_create_index(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let index = str_arg(args, 1, "index name", line, col)?;
    let body_val = args.get(2).cloned().unwrap_or(Value::None);

    with_pool(id, line, col, |pool| {
        let Pool::Elastic(client) = pool else {
            return Err(VietError::runtime_error("Not an Elasticsearch connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let body = value_to_json(&body_val);
        let response = rt.block_on(async {
            client.indices().create(elasticsearch::indices::IndicesCreateParts::Index(index))
                .body(body)
                .send()
                .await
        }).map_err(|e| VietError::runtime_error(format!("Elasticsearch create_index failed: {}", e), line, col))?;
        Ok(Value::Bool(response.status_code().is_success()))
    })
}

pub fn elastic_delete_index(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let id = pool_id(args.first(), line, col)?;
    let index = str_arg(args, 1, "index name", line, col)?;

    with_pool(id, line, col, |pool| {
        let Pool::Elastic(client) = pool else {
            return Err(VietError::runtime_error("Not an Elasticsearch connection".into(), line, col));
        };
        let rt = runtime(line, col)?;
        let response = rt.block_on(async {
            client.indices().delete(elasticsearch::indices::IndicesDeleteParts::Index(&[index]))
                .send()
                .await
        }).map_err(|e| VietError::runtime_error(format!("Elasticsearch delete_index failed: {}", e), line, col))?;
        Ok(Value::Bool(response.status_code().is_success()))
    })
}

// ---------------------------------------------------------------------------
// Helper utilities
// ---------------------------------------------------------------------------

fn pool_config(value: Option<&Value>) -> (u32, u32, Duration) {
    let Some(Value::Struct { fields, .. }) = value else {
        return (16, 0, Duration::from_secs(30));
    };
    let max = int_field(fields, "max_connections", 16).clamp(1, 1024) as u32;
    let min = int_field(fields, "min_connections", 0).clamp(0, max as i64) as u32;
    let timeout = int_field(fields, "acquire_timeout_ms", 30_000).clamp(1, 300_000) as u64;
    (max, min, Duration::from_millis(timeout))
}

fn int_field(fields: &HashMap<String, Value>, name: &str, default: i64) -> i64 {
    match fields.get(name) { Some(Value::Int(v)) => *v, _ => default }
}

fn str_arg<'a>(args: &'a [Value], idx: usize, name: &str, line: usize, col: usize) -> VietResult<&'a str> {
    match args.get(idx) {
        Some(Value::String(s)) => Ok(s.as_str()),
        _ => Err(VietError::type_error(format!("Expected string for {}", name), line, col)),
    }
}

fn operation_args(args: &[Value], line: usize, col: usize) -> VietResult<(usize, &str, &[Value])> {
    if args.len() < 2 {
        return Err(VietError::runtime_error("Database operation expects pool, SQL, and optional params".into(), line, col));
    }
    let id = pool_id(args.first(), line, col)?;
    let sql = match args.get(1) {
        Some(Value::String(v)) => v.as_str(),
        _ => return Err(VietError::type_error("SQL must be a string".into(), line, col)),
    };
    let params = match args.get(2) {
        Some(Value::Array(values)) => values.as_slice(),
        None => &[],
        _ => return Err(VietError::type_error("SQL params must be an array".into(), line, col)),
    };
    Ok((id, sql, params))
}

fn bind_postgres<'q>(
    query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>,
    value: &'q Value,
) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
    match value {
        Value::Int(v) => query.bind(*v),
        Value::Float(v) => query.bind(*v),
        Value::Bool(v) => query.bind(*v),
        Value::String(v) => query.bind(v),
        Value::None => query.bind(Option::<String>::None),
        other => query.bind(other.to_string()),
    }
}

fn bind_mysql<'q>(
    query: sqlx::query::Query<'q, sqlx::MySql, sqlx::mysql::MySqlArguments>,
    value: &'q Value,
) -> sqlx::query::Query<'q, sqlx::MySql, sqlx::mysql::MySqlArguments> {
    match value {
        Value::Int(v) => query.bind(*v),
        Value::Float(v) => query.bind(*v),
        Value::Bool(v) => query.bind(*v),
        Value::String(v) => query.bind(v),
        Value::None => query.bind(Option::<String>::None),
        other => query.bind(other.to_string()),
    }
}

fn pg_row(row: &PgRow) -> Value {
    row_value(row.columns().iter().enumerate().map(|(i, col)| (col.name(), read_pg(row, i))))
}

fn mysql_row(row: &MySqlRow) -> Value {
    row_value(row.columns().iter().enumerate().map(|(i, col)| (col.name(), read_mysql(row, i))))
}

fn row_value<'a>(values: impl Iterator<Item = (&'a str, Value)>) -> Value {
    Value::Struct {
        type_name: "Map".into(),
        fields: values.map(|(name, value)| (name.to_string(), value)).collect(),
    }
}

fn read_pg(row: &PgRow, index: usize) -> Value {
    if let Ok(value) = row.try_get::<Option<String>, _>(index) { return value.map(Value::String).unwrap_or(Value::None); }
    if let Ok(value) = row.try_get::<Option<i64>, _>(index) { return value.map(Value::Int).unwrap_or(Value::None); }
    if let Ok(value) = row.try_get::<Option<f64>, _>(index) { return value.map(Value::Float).unwrap_or(Value::None); }
    if let Ok(value) = row.try_get::<Option<bool>, _>(index) { return value.map(Value::Bool).unwrap_or(Value::None); }
    Value::String("<unsupported-sql-type>".into())
}

fn read_mysql(row: &MySqlRow, index: usize) -> Value {
    if let Ok(value) = row.try_get::<Option<String>, _>(index) { return value.map(Value::String).unwrap_or(Value::None); }
    if let Ok(value) = row.try_get::<Option<i64>, _>(index) { return value.map(Value::Int).unwrap_or(Value::None); }
    if let Ok(value) = row.try_get::<Option<f64>, _>(index) { return value.map(Value::Float).unwrap_or(Value::None); }
    if let Ok(value) = row.try_get::<Option<bool>, _>(index) { return value.map(Value::Bool).unwrap_or(Value::None); }
    Value::String("<unsupported-sql-type>".into())
}

fn db_error(context: &str, error: sqlx::Error, line: usize, col: usize) -> VietError {
    VietError::runtime_error(format!("{} failed: {}", context, error), line, col)
}

// BSON <-> Value conversion helpers
fn value_to_bson_doc(value: &Value) -> mongodb::bson::Document {
    match value_to_bson(value) {
        mongodb::bson::Bson::Document(doc) => doc,
        _ => mongodb::bson::doc! {},
    }
}

fn value_to_bson(value: &Value) -> mongodb::bson::Bson {
    use mongodb::bson::Bson;
    match value {
        Value::String(s) => Bson::String(s.clone()),
        Value::Int(n) => Bson::Int64(*n),
        Value::Float(f) => Bson::Double(*f),
        Value::Bool(b) => Bson::Boolean(*b),
        Value::None => Bson::Null,
        Value::Array(arr) => Bson::Array(arr.iter().map(value_to_bson).collect()),
        Value::Struct { fields, .. } => {
            let mut doc = mongodb::bson::Document::new();
            for (k, v) in fields {
                doc.insert(k.clone(), value_to_bson(v));
            }
            Bson::Document(doc)
        }
        _ => Bson::String(value.to_string()),
    }
}

fn bson_doc_to_value(doc: &mongodb::bson::Document) -> Value {
    let mut fields = HashMap::new();
    for (k, v) in doc {
        fields.insert(k.clone(), bson_to_value(v));
    }
    Value::Struct { type_name: "Map".into(), fields }
}

fn bson_to_value(bson: &mongodb::bson::Bson) -> Value {
    use mongodb::bson::Bson;
    match bson {
        Bson::String(s) => Value::String(s.clone()),
        Bson::Int32(n) => Value::Int(*n as i64),
        Bson::Int64(n) => Value::Int(*n),
        Bson::Double(f) => Value::Float(*f),
        Bson::Boolean(b) => Value::Bool(*b),
        Bson::Null => Value::None,
        Bson::ObjectId(oid) => Value::String(oid.to_hex()),
        Bson::Array(arr) => Value::Array(arr.iter().map(bson_to_value).collect()),
        Bson::Document(doc) => bson_doc_to_value(doc),
        Bson::DateTime(dt) => Value::String(dt.to_string()),
        other => Value::String(other.to_string()),
    }
}

// JSON <-> Value conversion
fn value_to_json(value: &Value) -> JsonValue {
    match value {
        Value::String(s) => JsonValue::String(s.clone()),
        Value::Int(n) => JsonValue::Number(serde_json::Number::from(*n)),
        Value::Float(f) => serde_json::json!(f),
        Value::Bool(b) => JsonValue::Bool(*b),
        Value::None => JsonValue::Null,
        Value::Array(arr) => JsonValue::Array(arr.iter().map(value_to_json).collect()),
        Value::Struct { fields, .. } => {
            let mut map = serde_json::Map::new();
            for (k, v) in fields { map.insert(k.clone(), value_to_json(v)); }
            JsonValue::Object(map)
        }
        _ => JsonValue::String(value.to_string()),
    }
}

fn json_to_value(json: &JsonValue) -> Value {
    match json {
        JsonValue::String(s) => Value::String(s.clone()),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() { Value::Int(i) }
            else if let Some(f) = n.as_f64() { Value::Float(f) }
            else { Value::String(n.to_string()) }
        }
        JsonValue::Bool(b) => Value::Bool(*b),
        JsonValue::Null => Value::None,
        JsonValue::Array(arr) => Value::Array(arr.iter().map(json_to_value).collect()),
        JsonValue::Object(map) => {
            let mut fields = HashMap::new();
            for (k, v) in map { fields.insert(k.clone(), json_to_value(v)); }
            Value::Struct { type_name: "Map".into(), fields }
        }
    }
}

fn json_map_to_value(map: &serde_json::Map<String, JsonValue>) -> Value {
    let mut fields = HashMap::new();
    for (k, v) in map { fields.insert(k.clone(), json_to_value(v)); }
    Value::Struct { type_name: "Map".into(), fields }
}

fn value_to_redis_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Int(n) => n.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Bool(b) => if *b { "1".into() } else { "0".into() },
        Value::None => String::new(),
        _ => value.to_string(),
    }
}

fn redis_value_to_viet(val: redis::Value) -> Value {
    match val {
        redis::Value::Nil => Value::None,
        redis::Value::Int(n) => Value::Int(n),
        redis::Value::BulkString(bytes) => {
            Value::String(String::from_utf8_lossy(&bytes).to_string())
        }
        redis::Value::SimpleString(s) => Value::String(s),
        redis::Value::Array(arr) => Value::Array(arr.into_iter().map(redis_value_to_viet).collect()),
        redis::Value::Boolean(b) => Value::Bool(b),
        redis::Value::Double(f) => Value::Float(f),
        redis::Value::BigNumber(n) => Value::String(n.to_string()),
        _ => Value::None,
    }
}

fn cassandra_cql_to_value(val: &scylla::frame::response::result::CqlValue) -> Value {
    use scylla::frame::response::result::CqlValue;
    match val {
        CqlValue::Text(s) => Value::String(s.clone()),
        CqlValue::Ascii(s) => Value::String(s.clone()),
        CqlValue::Int(n) => Value::Int(*n as i64),
        CqlValue::BigInt(n) => Value::Int(*n),
        CqlValue::SmallInt(n) => Value::Int(*n as i64),
        CqlValue::TinyInt(n) => Value::Int(*n as i64),
        CqlValue::Float(f) => Value::Float(*f as f64),
        CqlValue::Double(f) => Value::Float(*f),
        CqlValue::Boolean(b) => Value::Bool(*b),
        CqlValue::Uuid(u) => Value::String(u.to_string()),
        CqlValue::Timeuuid(u) => Value::String(u.to_string()),
        CqlValue::List(items) => Value::Array(items.iter().map(cassandra_cql_to_value).collect()),
        CqlValue::Set(items) => Value::Array(items.iter().map(cassandra_cql_to_value).collect()),
        _ => Value::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_lazy_async_pools_without_network_io() {
        let pg = connect(
            &[Value::String("postgres://user:pass@localhost/db".into())],
            "postgres",
            1,
            1,
        )
        .unwrap();
        let mysql = connect(
            &[Value::String("mysql://user:pass@localhost/db".into())],
            "mysql",
            1,
            1,
        )
        .unwrap();
        assert!(matches!(pg, Value::Struct { .. }));
        assert!(matches!(mysql, Value::Struct { .. }));
    }

    #[test]
    fn redis_connect_creates_handle() {
        let handle = redis_connect(
            &[Value::String("redis://localhost:6379".into())],
            1,
            1,
        ).unwrap();
        assert!(matches!(handle, Value::Struct { .. }));
    }

    #[test]
    fn clickhouse_connect_creates_handle() {
        let handle = clickhouse_connect(
            &[Value::String("localhost".into()), Value::Int(8123), Value::String("default".into())],
            1,
            1,
        ).unwrap();
        assert!(matches!(handle, Value::Struct { .. }));
    }
}
