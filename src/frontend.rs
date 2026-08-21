//! Whole-project frontend: deterministic import resolution, cycle detection,
//! semantic analysis and typed-IR lowering.

use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use crate::{
    error::{VietError, VietResult},
    lexer::Lexer,
    parser::{
        ast::{Program, Statement},
        Parser,
    },
    semantic::SemanticAnalyzer,
    typed_ir::TypedModule,
};

#[derive(Debug, Clone)]
pub struct CheckedProject {
    pub entry: PathBuf,
    pub modules: Vec<TypedModule>,
}

pub fn check_project(entry: &Path) -> VietResult<CheckedProject> {
    let entry = canonical(entry)?;
    let root = project_root(&entry);
    let mut loader = Loader {
        root,
        visiting: HashSet::new(),
        visited: HashSet::new(),
        programs: HashMap::new(),
        order: Vec::new(),
    };
    loader.visit(&entry)?;

    // Analyze one dependency-ordered program so imported declarations carry
    // their real signatures across module boundaries. Imports are graph edges,
    // not executable declarations in the semantic IR.
    let combined = Program {
        statements: loader
            .order
            .iter()
            .flat_map(|path| {
                loader.programs[path]
                    .statements
                    .iter()
                    .filter(|statement| !matches!(statement, Statement::Import { .. }))
                    .cloned()
            })
            .collect(),
    };
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&combined)?;

    let mut modules = Vec::with_capacity(loader.order.len());
    for path in &loader.order {
        let program = loader.programs.get(path).expect("loaded module");
        modules.push(TypedModule::lower(path.clone(), program, &analyzer));
    }
    Ok(CheckedProject { entry, modules })
}

struct Loader {
    root: PathBuf,
    visiting: HashSet<PathBuf>,
    visited: HashSet<PathBuf>,
    programs: HashMap<PathBuf, Program>,
    order: Vec<PathBuf>,
}

impl Loader {
    fn visit(&mut self, path: &Path) -> VietResult<()> {
        let path = canonical(path)?;
        if self.visited.contains(&path) {
            return Ok(());
        }
        if !self.visiting.insert(path.clone()) {
            return Err(VietError::runtime_error(
                format!("Circular module dependency includes '{}'", path.display()),
                0,
                0,
            ));
        }
        let source = fs::read_to_string(&path).map_err(|error| {
            VietError::runtime_error(
                format!("Cannot read module '{}': {}", path.display(), error),
                0,
                0,
            )
        })?;
        let tokens = Lexer::new(&source).tokenize()?;
        let program = Parser::new(tokens).parse()?;
        let imports: Vec<_> = program
            .statements
            .iter()
            .filter_map(|statement| match statement {
                Statement::Import { path, span, .. } => Some((path.clone(), span.clone())),
                _ => None,
            })
            .collect();
        for (segments, span) in imports {
            let dependency = resolve_import(&self.root, &path, &segments).ok_or_else(|| {
                VietError::runtime_error(
                    format!(
                        "Cannot resolve import '{}' from '{}'",
                        segments.join("."),
                        path.display()
                    ),
                    span.line,
                    span.column,
                )
            })?;
            self.visit(&dependency)?;
        }
        self.visiting.remove(&path);
        self.visited.insert(path.clone());
        self.programs.insert(path.clone(), program);
        self.order.push(path);
        Ok(())
    }
}

fn resolve_import(root: &Path, importer: &Path, segments: &[String]) -> Option<PathBuf> {
    let joined = segments.join("/");
    let module_joined = segments
        .strip_prefix(&["modules".to_string()])
        .map(|parts| parts.join("/"));
    let std_joined = segments
        .strip_prefix(&["std".to_string()])
        .map(|parts| parts.join("/"));
    let parent = importer.parent().unwrap_or(root);
    let mut candidates = vec![
        parent.join(format!("{}.vl", joined)),
        root.join(format!("{}.vl", joined)),
        root.join("src").join(format!("{}.vl", joined)),
        root.join("modules").join(&joined).join("src/main.vl"),
        root.join("modules").join(&joined).join("src/lib.vl"),
        root.join("modules").join(&joined).join("mod.vl"),
    ];
    if let Some(name) = std_joined {
        candidates.push(root.join("std").join(format!("{}.vl", name)));
        if let Some(home) = std::env::var_os("HOME") {
            candidates.push(
                PathBuf::from(home)
                    .join(".vietlang/std")
                    .join(format!("{}.vl", name)),
            );
        }
    }
    if let Some(name) = module_joined {
        candidates.push(root.join("modules").join(&name).join("src/main.vl"));
        candidates.push(root.join("modules").join(&name).join("src/lib.vl"));
    }
    candidates.into_iter().find(|candidate| candidate.is_file())
}

fn canonical(path: &Path) -> VietResult<PathBuf> {
    fs::canonicalize(path).map_err(|error| {
        VietError::runtime_error(
            format!("Cannot resolve '{}': {}", path.display(), error),
            0,
            0,
        )
    })
}

fn project_root(entry: &Path) -> PathBuf {
    let mut current = entry
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();
    loop {
        if current.join("vietlang.json").is_file() || current.join("Cargo.toml").is_file() {
            return current;
        }
        if !current.pop() {
            return entry
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_project() -> PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("vietlang-frontend-{id}"));
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("vietlang.json"), "{}").unwrap();
        root
    }

    #[test]
    fn builds_dependency_ordered_typed_module_graph() {
        let root = temp_project();
        fs::write(
            root.join("src/util.vl"),
            "pub fn answer() -> Int { return 42 }",
        )
        .unwrap();
        fs::write(root.join("src/main.vl"), "import util\nlet value: Int = 42").unwrap();
        let checked = check_project(&root.join("src/main.vl")).unwrap();
        assert_eq!(checked.modules.len(), 2);
        assert!(checked.modules[0].path.ends_with("util.vl"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn checks_imported_function_signatures_across_modules() {
        let root = temp_project();
        fs::write(
            root.join("src/util.vl"),
            "pub fn add(a: Int, b: Int) -> Int { return a + b }",
        )
        .unwrap();
        fs::write(root.join("src/main.vl"), "import util\nadd(1)").unwrap();
        let error = check_project(&root.join("src/main.vl")).unwrap_err();
        assert!(
            error.message.contains("Call expects 2"),
            "unexpected error: {:?}",
            error
        );
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn rejects_module_cycles() {
        let root = temp_project();
        fs::write(root.join("src/a.vl"), "import b").unwrap();
        fs::write(root.join("src/b.vl"), "import a").unwrap();
        let error = check_project(&root.join("src/a.vl")).unwrap_err();
        assert!(error.message.contains("Circular module dependency"));
        let _ = fs::remove_dir_all(root);
    }
}
