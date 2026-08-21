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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        run_repl();
    } else {
        match args[1].as_str() {
            "--help" | "-h" => print_help(),
            "--version" | "-v" => println!("VietLang v{}", VERSION),
            "install" | "add" | "update" | "remove" | "uninstall" | "search" | "init" | "list" | "ls" | "publish" | "verify" | "docs" | "info" | "sync" | "registry" => {
                pm::handle_vpm_command(&args[1..]);
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

fn run_file(path: &str) {
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

    let mut interpreter = Interpreter::new();
    match interpreter.execute(&program) {
        Ok(_) => {}
        Err(e) => {
            // Filter out control flow signals
            if !e.message.starts_with("__") {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
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
