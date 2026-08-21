//! VietLang - A Backend-First Programming Language
//!
//! Usage:
//!   vietlang <file.vl>    Run a VietLang source file
//!   vietlang               Start the REPL (interactive mode)
//!   vietlang --help        Show help

#![allow(unknown_lints)]
#![allow(clippy::all)]

mod error;
mod lexer;
mod parser;
mod interpreter;
mod stdlib;
pub mod vm;
mod pm;

use std::env;
use std::fs;
use std::io::{self, Write};

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use vm::compiler::Compiler;
use vm::VM;

const VERSION: &str = "0.1.0";

const BANNER: &str = r#"
 ╔════════════════════════════════════════════════════════════╗
 ║       __      ___      _   _                               ║
 ║       \ \    / (_)    | | | |                              ║
 ║        \ \  / / _  ___| |_| |     __ _ _ __   __ _         ║
 ║         \ \/ / | |/ _ \ __| |    / _` | '_ \ / _` |        ║
 ║          \  /  | |  __/ |_| |___| (_| | | | | (_| |        ║
 ║           \/   |_|\___|\__|______\__,_|_| |_|\__, |        ║
 ║                                               __/ |        ║
 ║              Backend-First Language v0.1.0    |___/        ║
 ║              Type 'help' for usage, 'exit' to quit         ║
 ╚════════════════════════════════════════════════════════════╝
"#;

const STANDALONE_MAGIC_FOOTER: &[u8; 16] = b"__VIETLANG_BIN__";

fn main() {
    // 0. Check if current binary is a standalone compiled executable (O(1) footer check)
    if let Ok(current_exe) = env::current_exe() {
        if let Ok(bytes) = fs::read(&current_exe) {
            let total_len = bytes.len();
            if total_len > 24 && &bytes[total_len - 16..] == STANDALONE_MAGIC_FOOTER {
                let mut len_bytes = [0u8; 8];
                len_bytes.copy_from_slice(&bytes[total_len - 24..total_len - 16]);
                let payload_len = u64::from_be_bytes(len_bytes) as usize;
                if payload_len > 0 && payload_len <= total_len - 24 {
                    let start = total_len - 24 - payload_len;
                    if let Ok(embedded_source) = std::str::from_utf8(&bytes[start..start + payload_len]) {
                        run_source(embedded_source, "<embedded>");
                        return;
                    }
                }
            }
        }
    }

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        run_repl();
    } else {
        match args[1].as_str() {
            "--version" | "-v" | "version" => {
                println!("VietLang v{}", VERSION);
            }
            "--help" | "-h" => print_help(),
            "build" | "compile" => {
                if args.len() < 3 {
                    eprintln!("Usage: vietlang build <source.vl> [-o <output_binary>] [--target <linux|windows|macos>]");
                    std::process::exit(1);
                }
                let source_file = &args[2];
                let mut output_file = source_file.trim_end_matches(".vl").to_string();
                let mut target_os = "linux".to_string();

                let mut i = 3;
                while i < args.len() {
                    if (args[i] == "-o" || args[i] == "--output") && i + 1 < args.len() {
                        output_file = args[i + 1].clone();
                        i += 2;
                    } else if args[i] == "--target" && i + 1 < args.len() {
                        target_os = args[i + 1].to_lowercase();
                        i += 2;
                    } else if !args[i].starts_with('-') {
                        output_file = args[i].clone();
                        i += 1;
                    } else {
                        i += 1;
                    }
                }
                if output_file.ends_with(".exe") {
                    target_os = "windows".to_string();
                }
                build_standalone_binary(source_file, &output_file, &target_os);
            }
            "pkg" | "package" | "pm" => {
                if args.len() >= 3 {
                    pm::handle_vpm_command(&args[2..]);
                } else {
                    pm::handle_vpm_command(&[]);
                }
            }
            "doc" | "docs" => {
                let target = if args.len() >= 3 {
                    &args[2]
                } else {
                    ""
                };
                pm::show_docs(target);
            }
            "install" | "add" | "update" | "remove" | "uninstall" | "search" | "init" | "list" | "ls" | "publish" | "verify" | "info" | "sync" | "registry" => {
                pm::handle_vpm_command(&args[1..]);
            }
            "run" => {
                if args.len() < 3 {
                    eprintln!("Usage: vietlang run <file.vl>");
                    std::process::exit(1);
                }
                run_file(&args[2]);
            }
            "check" => {
                if args.len() < 3 {
                    eprintln!("Usage: vietlang check <file.vl>");
                    std::process::exit(1);
                }
                check_file(&args[2]);
            }
            "test" => {
                let target = if args.len() >= 3 {
                    &args[2]
                } else {
                    "tests"
                };
                run_tests(target);
            }
            "--tokens" => {
                if args.len() < 3 {
                    eprintln!("Usage: vietlang --tokens <file.vl>");
                    std::process::exit(1);
                }
                show_tokens(&args[2]);
            }
            "--ast" => {
                if args.len() < 3 {
                    eprintln!("Usage: vietlang --ast <file.vl>");
                    std::process::exit(1);
                }
                show_ast(&args[2]);
            }
            "--vm" => {
                if args.len() < 3 {
                    eprintln!("Usage: vietlang --vm <file.vl>");
                    std::process::exit(1);
                }
                run_vm(&args[2]);
            }
            file => run_file(file),
        }
    }
}

fn check_file(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("\x1b[31mError:\x1b[0m Cannot read file '{}': {}", path, e);
            std::process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("\x1b[32m[PASS]\x1b[0m Syntax and AST check passed for '{}' ({} statements)", path, program.statements.len());
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn run_tests(target: &str) {
    let path = std::path::Path::new(target);
    if path.is_file() {
        run_file(target);
    } else if path.is_dir() {
        let mut count = 0;
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if let Some(ext) = p.extension() {
                    if ext == "vl" {
                        println!("\n\x1b[1;36m=== Running Test Suite: {} ===\x1b[0m", p.display());
                        run_file(p.to_str().unwrap_or_default());
                        count += 1;
                    }
                }
            }
        }
        if count == 0 {
            println!("No .vl test files found in '{}'", target);
        }
    } else {
        eprintln!("Error: Path '{}' not found", target);
        std::process::exit(1);
    }
}

fn print_help() {
    println!("VietLang v{} - A Backend-First Programming Language", VERSION);
    println!();
    println!("USAGE:");
    println!("  vietlang                          Start interactive REPL");
    println!("  vietlang <file.vl>                Execute a VietLang source file");
    println!("  vietlang --vm <file.vl>           Execute via Bytecode VM");
    println!("  vietlang install <pkg[@version]>  Install community package (e.g. redis@1.2.0)");
    println!("  vietlang update [pkg[@version]]   Update package(s) to latest or target version");
    println!("  vietlang remove <pkg>             Remove an installed package");
    println!("  vietlang search <query>           Search community package registry");
    println!("  vietlang init <name> [template]   Initialize project (api | lib | microservice)");
    println!("  vietlang list                     List installed dependencies");
    println!("  vietlang docs <module>            Inspect module exported functions");
    println!("  vietlang publish                  Validate & prepare release metadata");
    println!("  vietlang --tokens <file>          Show tokenized output");
    println!("  vietlang --ast <file>             Show parsed AST");
    println!("  vietlang --version                Show version");
    println!("  vietlang --help                   Show this help");
    println!();
    println!("EXAMPLES:");
    println!("  vietlang install redis@1.2.0      Install Redis module");
    println!("  vietlang search postgres          Search for PostgreSQL packages");
    println!("  vietlang hello.vl                 Run hello.vl");
    println!("  vietlang                          Start REPL");
}

fn run_source(source: &str, _name: &str) {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(program) => program,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();
    match interpreter.execute(&program) {
        Ok(_) => {}
        Err(e) => {
            if !e.message.starts_with("__") {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}

fn build_standalone_binary(source_path: &str, output_path: &str, target_os: &str) {
    println!("\x1b[36m[VietLang Compiler]\x1b[0m Compiling \x1b[33m{}\x1b[0m for target \x1b[35m[{}]\x1b[0m -> \x1b[32m{}\x1b[0m...", source_path, target_os, output_path);
    
    // 1. Read and validate source
    let source = match fs::read_to_string(source_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("\x1b[31mError:\x1b[0m Cannot read source file '{}': {}", source_path, e);
            std::process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("\x1b[31mSyntax Error:\x1b[0m {}", e);
            std::process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    if let Err(e) = parser.parse() {
        eprintln!("\x1b[31mParse Error:\x1b[0m {}", e);
        std::process::exit(1);
    }

    // 2. Select appropriate base runtime binary
    let base_binary = if target_os == "windows" || output_path.ends_with(".exe") {
        match fs::read("target/x86_64-pc-windows-gnu/release/vietlang.exe") {
            Ok(b) => b,
            Err(_) => match fs::read("vietlang-windows-x64.exe") {
                Ok(b) => b,
                Err(_) => match fs::read("/home/vantrong/.vietlang/bin/vietlang.exe") {
                    Ok(b) => b,
                    Err(e) => {
                        eprintln!("\x1b[31mError:\x1b[0m Cannot locate Windows base runtime binary (vietlang.exe): {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
    } else {
        let current_exe = env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("vietlang"));
        match fs::read(&current_exe) {
            Ok(b) if b.len() > 100_000 => b,
            _ => match fs::read("target/release/vietlang") {
                Ok(b) => b,
                Err(_) => match fs::read("/home/vantrong/.vietlang/bin/vietlang") {
                    Ok(b) => b,
                    Err(e) => {
                        eprintln!("\x1b[31mError:\x1b[0m Cannot locate Linux base runtime binary: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
    };

    // 3. Strip any preexisting footer from base_binary
    let total_len = base_binary.len();
    let clean_len = if total_len > 24 && &base_binary[total_len - 16..] == STANDALONE_MAGIC_FOOTER {
        let mut len_bytes = [0u8; 8];
        len_bytes.copy_from_slice(&base_binary[total_len - 24..total_len - 16]);
        let old_payload_len = u64::from_be_bytes(len_bytes) as usize;
        if old_payload_len <= total_len - 24 {
            total_len - 24 - old_payload_len
        } else {
            total_len
        }
    } else {
        total_len
    };

    let source_bytes = source.as_bytes();
    let payload_len = source_bytes.len() as u64;

    let mut standalone_bin = Vec::with_capacity(clean_len + source_bytes.len() + 24);
    standalone_bin.extend_from_slice(&base_binary[..clean_len]);
    standalone_bin.extend_from_slice(source_bytes);
    standalone_bin.extend_from_slice(&payload_len.to_be_bytes());
    standalone_bin.extend_from_slice(STANDALONE_MAGIC_FOOTER);

    // 4. Write output binary
    if let Err(e) = fs::write(output_path, &standalone_bin) {
        eprintln!("\x1b[31mError:\x1b[0m Cannot write output binary '{}': {}", output_path, e);
        std::process::exit(1);
    }

    // 5. Set executable permissions on Unix/Linux/macOS
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(output_path) {
            let mut perms = metadata.permissions();
            perms.set_mode(0o755);
            let _ = fs::set_permissions(output_path, perms);
        }
    }

    println!("\x1b[32m[SUCCESS]\x1b[0m Standalone executable created: \x1b[32;1m{}\x1b[0m ({} bytes, target: {})", output_path, standalone_bin.len(), target_os);
    if target_os == "windows" || output_path.ends_with(".exe") {
        println!("  -> Run on Windows with: \x1b[36m{}\x1b[0m (or 'wine {}')", output_path, output_path);
    } else {
        println!("  -> Run directly with: \x1b[36m./{}\x1b[0m", output_path);
    }
}

fn run_file(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("\x1b[31mError:\x1b[0m Cannot read file '{}': {}", path, e);
            std::process::exit(1);
        }
    };
    run_source(&source, path);
}

fn run_vm(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("\x1b[31mError:\x1b[0m Cannot read file '{}': {}", path, e);
            std::process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(program) => program,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let mut compiler = Compiler::new();
    let chunk = match compiler.compile(&program) {
        Ok(chunk) => chunk,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let mut vm = VM::new(chunk);
    match vm.run() {
        Ok(result) => {
            if result != crate::interpreter::value::Value::None {
                println!("Result: {}", result);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn run_repl() {
    println!("{}", BANNER);

    let mut interpreter = Interpreter::new();
    let mut line_num = 1;

    loop {
        print!("\x1b[36mvl:{}\x1b[0m \x1b[33m>\x1b[0m ", line_num);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,  // EOF
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        match input {
            "exit" | "quit" => {
                println!("\x1b[32mGoodbye! \x1b[0m");
                break;
            }
            "help" => {
                print_repl_help();
                continue;
            }
            "clear" => {
                print!("\x1b[2J\x1b[H");
                io::stdout().flush().unwrap();
                continue;
            }
            _ => {}
        }

        // Handle multi-line input (count braces)
        let mut full_input = input.to_string();
        let mut brace_count = count_braces(input);

        while brace_count > 0 {
            print!("\x1b[36m...\x1b[0m   ");
            io::stdout().flush().unwrap();

            let mut continuation = String::new();
            match io::stdin().read_line(&mut continuation) {
                Ok(0) => break,
                Ok(_) => {
                    brace_count += count_braces(continuation.trim());
                    full_input.push('\n');
                    full_input.push_str(continuation.trim());
                }
                Err(_) => break,
            }
        }

        // Lex
        let mut lexer = Lexer::new(&full_input);
        let tokens = match lexer.tokenize() {
            Ok(tokens) => tokens,
            Err(e) => {
                eprintln!("  \x1b[31m{}\x1b[0m", e);
                line_num += 1;
                continue;
            }
        };

        // Parse
        let mut parser = Parser::new(tokens);
        let program = match parser.parse() {
            Ok(program) => program,
            Err(e) => {
                eprintln!("  \x1b[31m{}\x1b[0m", e);
                line_num += 1;
                continue;
            }
        };

        // Execute
        match interpreter.execute(&program) {
            Ok(value) => {
                match &value {
                    interpreter::value::Value::None => {}
                    _ => println!("  \x1b[32m= {}\x1b[0m", value),
                }
            }
            Err(e) => {
                if !e.message.starts_with("__") {
                    eprintln!("  \x1b[31m{}\x1b[0m", e);
                }
            }
        }

        line_num += 1;
    }
}

fn count_braces(s: &str) -> i32 {
    let mut count = 0;
    for ch in s.chars() {
        match ch {
            '{' => count += 1,
            '}' => count -= 1,
            _ => {}
        }
    }
    count
}

fn show_tokens(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error: Cannot read file '{}': {}", path, e);
            std::process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source);
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("=== Tokens for '{}' ===", path);
            for (i, token) in tokens.iter().enumerate() {
                println!("  [{:3}] {}", i, token);
            }
            println!("=== Total: {} tokens ===", tokens.len());
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn show_ast(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error: Cannot read file '{}': {}", path, e);
            std::process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("=== AST for '{}' ===", path);
            for (i, stmt) in program.statements.iter().enumerate() {
                println!("  [{:3}] {:#?}", i, stmt);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn print_repl_help() {
    println!("\x1b[33m=== VietLang REPL Help ===\x1b[0m");
    println!();
    println!("  \x1b[36mCommands:\x1b[0m");
    println!("    help     Show this help message");
    println!("    clear    Clear the screen");
    println!("    exit     Exit the REPL");
    println!();
    println!("  \x1b[36mExamples:\x1b[0m");
    println!("    let x = 42");
    println!("    println(x + 8)");
    println!("    fn greet(name: String) {{ println(\"Hello, \" + name) }}");
    println!("    greet(\"VietLang\")");
    println!();
    println!("  \x1b[36mTypes:\x1b[0m Int, Float, String, Bool, None, Array");
    println!("  \x1b[36mKeywords:\x1b[0m let, mut, fn, if, else, for, while, match,");
    println!("            struct, enum, impl, return, break, continue");
    println!();
}
