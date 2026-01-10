// ─── main.rs ─────────────────────────────────────────────────────────────────
//
// YaarScript CLI — entry point for the YaarScript compiler.
//
// Subcommands
// ───────────
//  yaar run   <file>   — compile and execute a .yaar source file
//  yaar check <file>   — type-check only (no codegen / execution)
//  yaar help  [cmd]    — show detailed help for a subcommand
//
// ─────────────────────────────────────────────────────────────────────────────

use std::path::Path;
use std::time::Instant;

use clap::{Parser, Subcommand};
use colored::*;

use compiler::codegen::execution::ExecutionEngine;
use compiler::core::token::TokenType;
use compiler::error::ErrorReporter;
use compiler::ir_pipeline::tac::TACGenerator;
use compiler::ir_pipeline::tac_optimizer::IROptimizer;
use compiler::lexer::lexer::Lexer;
use compiler::parser::parser::Parser as YaarParser;
use compiler::semantics::scope::ScopeAnalyzer;
use compiler::semantics::type_checker::TypeChecker;

// ── CLI definition ────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(
    name    = "yaar",
    version = "0.1.0",
    author  = "Bazil Suhail",
    about   = "YaarScript — a small systems language compiler",
    long_about = "\
YaarScript is an educational multi-phase compiler written in Rust. \
Built by Bazil Suhail to demonstrate advanced compiler construction \
techniques — semantic analysis, IR optimization, and bytecode execution \
— while using a uniquely fun, Urdu-infused slang syntax to make systems \
programming more relatable and engaging.\n\n\
GitHub : https://github.com/BazilSuhail/YaarScript\n\
Author : https://github.com/BazilSuhail\n\
Web    : https://yaarscript.netlify.app\n\n\
Use `yaar run <file>` to compile and execute a .yaar source file.",
    disable_help_subcommand = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile and execute a .yaar source file.
    Run {
        /// Path to the .yaar source file.
        file: String,

        /// Allow .txt files as input (otherwise rejected).
        #[arg(long)]
        allow_txt: bool,

        /// Skip execution, only compile.
        #[arg(long)]
        no_exec: bool,
    },

    /// Type-check a .yaar source file without running it.
    Check {
        /// Path to the .yaar source file.
        file: String,

        /// Allow .txt files as input (otherwise rejected).
        #[arg(long)]
        allow_txt: bool,
    },

    /// Show detailed help for a subcommand.
    Help {
        #[arg(value_name = "COMMAND")]
        command: Option<String>,
    },
}

// ── Banner ────────────────────────────────────────────────────────────────────

fn print_banner() {
    println!("{}", r"
██╗   ██╗ █████╗  █████╗ ██████╗    ██████╗ ██████╗  ██████╗ ██╗ ██████╗ ████████╗
╚██╗ ██╔╝██╔══██╗██╔══██╗██╔══██╗   ██╔════╝██╔════╝ ██╔══██╗██║ ██╔══██╗╚══██╔══╝
 ╚████╔╝ ███████║███████║██████╔╝   ╚█████╗ ██║      ██████╔╝██║ ██████╔╝   ██║   
  ╚██╔╝  ██╔══██║██╔══██║██╔══██╗    ╚═══██╗██║      ██╔══██╗██║ ██╔═══╝    ██║   
   ██║   ██║  ██║██║  ██║██║  ██║   ██████╔╝╚██████╗ ██║  ██║██║ ██║        ██║   
   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝   ╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝ ╚═╝        ╚═╝   ".bright_cyan());
    println!(
        "  {} v{}",
        "YaarScript  |  Systems Language Compiler".bright_white().bold(),
        "0.1.0".dimmed()
    );
    println!(
        "  {}",
        "by Bazil Suhail  •  yaarscript.netlify.app".dimmed()
    );
    println!();
}

// ── Custom help ───────────────────────────────────────────────────────────────

fn print_help(cmd: Option<&str>) {
    match cmd {
        Some("run") => println!(r#"
run — Compile & Execute

Compiles a .yaar source file and runs the generated IR.

Usage:
  yaar run <file.yaar>
  yaar run myprogram          (searches for myprogram.yaar)

Flags:
  --allow-txt    Allow .txt files as input (default: reject)
  --no-exec      Compile only, do not execute

Example:
  yaar run examples/hello.yaar
  yaar run test --no-exec
"#),

        Some("check") => println!(r#"
check — Type-Check Only

Runs lexing, parsing, scope analysis, and type checking.
Stops before IR generation and execution.

Usage:
  yaar check <file.yaar>

Flags:
  --allow-txt    Allow .txt files as input

Example:
  yaar check examples/hello.yaar
"#),

        _ => println!(r#"
YaarScript Compiler — CLI

An educational Urdu-slang multi-phase compiler written in Rust.
Built by Bazil Suhail.

GitHub : https://github.com/BazilSuhail/YaarScript
Author : https://github.com/BazilSuhail
Web    : https://yaarscript.netlify.app

Usage:
  yaar <command> [options] <file>

Commands:
  run     Compile and execute a .yaar source file
  check   Type-check only (no execution)
  help    Show help for a command

Flags:
  --help     Show this help
  --version  Show version

Examples:
  yaar run hello.yaar
  yaar run myprogram
  yaar check types.yaar
  yaar help run
"#),
    }
}

// ── Source file resolution ────────────────────────────────────────────────────

fn resolve_source_file(file: &str, allow_txt: bool) -> Result<String, String> {
    let path = Path::new(file);

    // .yaar files are always accepted
    if path.extension().and_then(|e| e.to_str()) == Some("yaar") {
        return Ok(file.to_string());
    }

    // .txt files require --allow-txt
    if path.extension().and_then(|e| e.to_str()) == Some("txt") {
        if allow_txt {
            return Ok(file.to_string());
        } else {
            return Err(format!(
                ".txt files are not allowed. Use --allow-txt to allow, or rename to .yaar"
            ));
        }
    }

    // No .yaar extension — try appending .yaar
    let yaar_path = format!("{}.yaar", file);
    if Path::new(&yaar_path).exists() {
        return Ok(yaar_path);
    }

    // Fall back: original path if it exists
    if path.exists() {
        return Ok(file.to_string());
    }

    Err(format!(
        "Could not find source file '{}' or '{}.yaar' in the current directory",
        file, file
    ))
}

// ── Compilation pipeline (shared by run & check) ──────────────────────────────

struct CompilationResult {
    optimized_tac: Vec<compiler::ir_pipeline::tac::Instruction>,
}

fn run_pipeline(filename: &str) -> Result<CompilationResult, ()> {
    let source = match std::fs::read_to_string(filename) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "  {} Cannot read '{}': {}",
                "ERROR:".bright_red(),
                filename,
                e
            );
            return Err(());
        }
    };

    let reporter = ErrorReporter::new(&source);

    // ── Lexing ────────────────────────────────────────────────────────────────
    let t0 = Instant::now();
    let mut lexer = Lexer::new(source.clone());
    let tokens = lexer.tokenize();
    let has_lex_errors = tokens.iter().any(|t| t.token_type == TokenType::Error);

    if has_lex_errors {
        for token in &tokens {
            if token.token_type == TokenType::Error {
                reporter.report_lexical(&token.value, token.line, token.column);
            }
        }
        eprintln!(
            "  {} Lexical analysis failed — unrecognized tokens.",
            "ERROR:".bright_red()
        );
        return Err(());
    }
    println!(
        "  {} Lexing      — {:.2?}",
        "✓".bright_green(),
        t0.elapsed()
    );

    // ── Parsing ───────────────────────────────────────────────────────────────
    let t0 = Instant::now();
    let mut parser = YaarParser::new(tokens);
    let ast = match parser.parse_program() {
        Ok(ast) => ast,
        Err(err) => {
            reporter.report_syntax(&err.message, err.token.line, err.token.column);
            eprintln!("  {} Syntax analysis failed.", "ERROR:".bright_red());
            return Err(());
        }
    };
    println!(
        "  {} Parsing     — {:.2?}",
        "✓".bright_green(),
        t0.elapsed()
    );

    // ── Scope analysis ────────────────────────────────────────────────────────
    let t0 = Instant::now();
    let mut scope_analyzer = ScopeAnalyzer::new();
    if let Err(errors) = scope_analyzer.analyze(&ast) {
        for error in &errors {
            reporter.report_scope(&error.message, error.line, error.column);
        }
        eprintln!(
            "  {} Scope analysis failed — {} error(s)",
            "ERROR:".bright_red(),
            errors.len()
        );
        return Err(());
    }
    println!(
        "  {} Scoping     — {:.2?}",
        "✓".bright_green(),
        t0.elapsed()
    );

    // ── Type checking ─────────────────────────────────────────────────────────
    let t0 = Instant::now();
    let mut type_checker = TypeChecker::new(scope_analyzer.get_global_scope());
    if let Err(errors) = type_checker.check(&ast) {
        for error in &errors {
            reporter.report_type(&error.message, error.line, error.column);
        }
        eprintln!(
            "  {} Type checking failed — {} error(s)",
            "ERROR:".bright_red(),
            errors.len()
        );
        return Err(());
    }
    println!(
        "  {} Typing      — {:.2?}",
        "✓".bright_green(),
        t0.elapsed()
    );

    // ── IR generation ─────────────────────────────────────────────────────────
    let t0 = Instant::now();
    let mut tac_gen = TACGenerator::new();
    let raw_tac = tac_gen.generate(&ast);

    if let Err(e) = tac_gen.save_to_file("three-address-code.txt") {
        eprintln!("  {} Failed to save raw TAC: {}", "WARN:".bright_yellow(), e);
    }
    println!(
        "  {} IR Gen      — {:.2?}",
        "✓".bright_green(),
        t0.elapsed()
    );

    // ── IR optimization ───────────────────────────────────────────────────────
    let t0 = Instant::now();
    let mut optimizer = IROptimizer::new(raw_tac);
    optimizer.run();
    let optimized_tac = optimizer.get_instructions();

    if let Err(e) = optimizer.save_to_file("optimal-three-address-code.txt") {
        eprintln!("  {} Failed to save optimized TAC: {}", "WARN:".bright_yellow(), e);
    }
    println!(
        "  {} Optimizing  — {:.2?}",
        "✓".bright_green(),
        t0.elapsed()
    );

    Ok(CompilationResult { optimized_tac })
}

// ── Subcommand: run ───────────────────────────────────────────────────────────

fn cmd_run(file: String, allow_txt: bool, no_exec: bool) {
    println!("{}", "[ Compile & Execute ]".bold().bright_cyan());

    let filename = match resolve_source_file(&file, allow_txt) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("  {} {}", "ERROR:".bright_red(), e);
            return;
        }
    };

    println!(        "  Source   : {}", filename.bright_white());
    println!();

    let total_start = Instant::now();

    let result = match run_pipeline(&filename) {
        Ok(r) => r,
        Err(()) => {
            eprintln!("\n  {} Compilation failed.", "✗".bright_red().bold());
            return;
        }
    };

    let total_duration = total_start.elapsed();
    println!(
        "\n  {} Compilation finished in {:.2?}",
        "✓".bright_green().bold(),
        total_duration
    );

    if no_exec {
        println!("  {} Skipping execution (--no-exec).", "…".bright_cyan());
        return;
    }

    // ── Execution ─────────────────────────────────────────────────────────────
    println!(
        "\n  {} Running compiled program...\n",
        "▶".bright_blue().bold()
    );

    let engine = ExecutionEngine::new(result.optimized_tac);
    if let Err(e) = engine.execute() {
        eprintln!(
            "\n  {} Execution failed: {}",
            "ERROR:".bright_red(),
            e
        );
        return;
    }

    println!(
        "\n  {} Program finished.",
        "✓".bright_green().bold()
    );
}

// ── Subcommand: check ─────────────────────────────────────────────────────────

fn cmd_check(file: String, allow_txt: bool) {
    println!("{}", "[ Type-Check Only ]".bold().bright_cyan());

    let filename = match resolve_source_file(&file, allow_txt) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("  {} {}", "ERROR:".bright_red(), e);
            return;
        }
    };

    println!(        "  Source   : {}", filename.bright_white());
    println!();

    let total_start = Instant::now();

    let _result = match run_pipeline(&filename) {
        Ok(r) => r,
        Err(()) => {
            eprintln!("\n  {} Type-check failed.", "✗".bright_red().bold());
            return;
        }
    };

    let total_duration = total_start.elapsed();
    println!(
        "\n  {} Type-check passed in {:.2?}  {}",
        "✓".bright_green().bold(),
        total_duration,
        "(no execution)".dimmed()
    );
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    print_banner();
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            file,
            allow_txt,
            no_exec,
        } => cmd_run(file, allow_txt, no_exec),

        Commands::Check { file, allow_txt } => cmd_check(file, allow_txt),

        Commands::Help { command } => print_help(command.as_deref()),
    }
}
