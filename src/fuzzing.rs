//! Deterministic mutation fuzzer used in CI and available through `vietlang fuzz`.

use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::{http_runtime, interpreter::value::Value, lexer::Lexer, parser::Parser, stdlib};

pub fn run(iterations: usize, seed: u64) -> Result<(), String> {
    let corpus = [
        "let value: Result<Int, String> = Ok(1)",
        "fn serve(req) { return req }",
        "{\"name\":\"package\",\"dependencies\":{}}",
        "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n",
        "match Some(1) { Some(value) => value, None => 0 }",
    ];
    let mut rng = Lcg(seed);
    for index in 0..iterations {
        let base = corpus[(rng.next() as usize) % corpus.len()].as_bytes();
        let mutated = mutate(base, &mut rng);
        let source = String::from_utf8_lossy(&mutated).to_string();
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            if let Ok(tokens) = Lexer::new(&source).tokenize() {
                let _ = Parser::new(tokens).parse();
            }
            let _ = stdlib::builtin_json_parse(&[Value::String(source.clone())], 1, 1);
            let _ = serde_json::from_str::<serde_json::Value>(&source);
            let mut fields = std::collections::HashMap::new();
            fields.insert(
                "port".into(),
                Value::Int((rng.next() as i64).wrapping_sub(i64::MAX / 2)),
            );
            fields.insert("max_body_bytes".into(), Value::Int(rng.next() as i64));
            let _ = http_runtime::validate_config_for_fuzz(Value::Struct {
                type_name: "Map".into(),
                fields,
            });
        }));
        if outcome.is_err() {
            return Err(format!(
                "panic found at iteration {} with seed {}",
                index, seed
            ));
        }
    }
    Ok(())
}

fn mutate(input: &[u8], rng: &mut Lcg) -> Vec<u8> {
    let mut output = input.to_vec();
    let operations = 1 + (rng.next() % 8) as usize;
    for _ in 0..operations {
        match rng.next() % 3 {
            0 if !output.is_empty() => {
                let index = rng.next() as usize % output.len();
                output[index] ^= rng.next() as u8;
            }
            1 if output.len() < 65_536 => {
                let index = rng.next() as usize % (output.len() + 1);
                output.insert(index, rng.next() as u8);
            }
            2 if !output.is_empty() => {
                let index = rng.next() as usize % output.len();
                output.remove(index);
            }
            _ => {}
        }
    }
    output
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn mutation_smoke_never_panics() {
        super::run(500, 0x5649_4554).unwrap();
    }
}
