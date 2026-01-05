// main.rs - using lib.rs for module organization
use compiler::lexer::lexer::Lexer;
use compiler::core::token::TokenType;
use compiler::parser::parser::Parser;
use compiler::semantics::scope::ScopeAnalyzer;
use compiler::semantics::type_checker::TypeChecker;
use compiler::ir_pipeline::tac::TACGenerator;
use compiler::ir_pipeline::tac_optimizer::IROptimizer;
use compiler::codegen::execution::ExecutionEngine;
use compiler::error::ErrorReporter;

use std::env;
use std::fs;
use std::time::Instant;

// Native ANSI Terminal Escape Codes (Zero Dependencies)
const RESET: &str = "\x1b[0m";
const RED_BOLD: &str = "\x1b[1;31m";
const GREEN: &str = "\x1b[32m";
const YELLOW_BOLD: &str = "\x1b[1;33m";
const BLUE_BOLD: &str = "\x1b[1;34m";
const CYAN_BOLD: &str = "\x1b[1;36m";

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "test_input.yaar" };
    
    let total_start = Instant::now();

    println!("   {}Compiling{} {}", CYAN_BOLD, RESET, filename);

    let source = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("       {}Error{} failed to read '{}'", RED_BOLD, RESET, filename);
            return;
        }
    };
    
    let reporter = ErrorReporter::new(&source);
    
    // --- Lexical Analysis ---
    let lex_start = Instant::now();
    let mut lexer = Lexer::new(source.clone());
    let tokens = lexer.tokenize();
    let has_lex_errors = tokens.iter().any(|t| t.token_type == TokenType::Error);
    
    if has_lex_errors {
        for token in &tokens {
            if token.token_type == TokenType::Error {
                reporter.report_lexical(&token.value, token.line, token.column);
            }
        }
        eprintln!("       {}Error{} lexical analysis failed due to unrecognized tokens.", RED_BOLD, RESET);
        return;
    }
    let lex_duration = lex_start.elapsed();
    println!("      {}Lexing{} in {:.2?}", GREEN, RESET, lex_duration);

    // --- Syntax Analysis ---
    let parse_start = Instant::now();
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse_program() {
        Ok(ast) => ast,
        Err(err) => {
            reporter.report_syntax(&err.message, err.token.line, err.token.column);
            eprintln!("       {}Error{} syntax analysis failed.", RED_BOLD, RESET);
            return;
        }
    };
    let parse_duration = parse_start.elapsed();
    println!("     {}Parsing{} in {:.2?}", GREEN, RESET, parse_duration);

    // --- Scope Analysis ---
    let scope_start = Instant::now();
    let mut scope_analyzer = ScopeAnalyzer::new();
    if let Err(errors) = scope_analyzer.analyze(&ast) {
        for error in &errors {
            reporter.report_scope(&error.message, error.line, error.column);
        }
        eprintln!("       {}Error{} scope analysis failed with {} error(s)", RED_BOLD, RESET, errors.len());
        return;
    }
    let scope_duration = scope_start.elapsed();
    println!("     {}Scoping{} in {:.2?}", GREEN, RESET, scope_duration);

    // --- Type Analysis ---
    let type_start = Instant::now();
    let mut type_checker = TypeChecker::new(scope_analyzer.get_global_scope());
    if let Err(errors) = type_checker.check(&ast) {
        for error in &errors {
            reporter.report_type(&error.message, error.line, error.column);
        }
        eprintln!("       {}Error{} type checking failed with {} error(s)", RED_BOLD, RESET, errors.len());
        return;
    }
    let type_duration = type_start.elapsed();
    println!("      {}Typing{} in {:.2?}", GREEN, RESET, type_duration);

    // --- IR Generation ---
    let tac_start = Instant::now();
    let mut tac_gen = TACGenerator::new();
    let raw_tac = tac_gen.generate(&ast);
    
    if let Err(e) = tac_gen.save_to_file("three-address-code.txt") {
        eprintln!("     {}Warning{} failed to save raw TAC: {}", YELLOW_BOLD, RESET, e);
    }
    let tac_duration = tac_start.elapsed();
    println!("      {}IR Gen{} in {:.2?}", GREEN, RESET, tac_duration);

    // --- IR Optimization ---
    let opt_start = Instant::now();
    let mut optimizer = IROptimizer::new(raw_tac);
    optimizer.run();
    let optimized_tac = optimizer.get_instructions();
    
    if let Err(e) = optimizer.save_to_file("optimal-three-address-code.txt") {
        eprintln!("     {}Warning{} failed to save optimized TAC: {}", YELLOW_BOLD, RESET, e);
    }
    let opt_duration = opt_start.elapsed();
    println!("  {}Optimizing{} in {:.2?}", GREEN, RESET, opt_duration);

    let total_duration = total_start.elapsed();
    println!("    {}Finished{} compilation pipeline in {:.2?}\n", CYAN_BOLD, RESET, total_duration);

    // --- Execution ---
    println!("     {}Running{} compiled program...\n\n", BLUE_BOLD, RESET);
    
    let engine = ExecutionEngine::new(optimized_tac);
    if let Err(e) = engine.execute() {
        eprintln!("\n       {}Error{} execution failed: {}", RED_BOLD, RESET, e);
        return;
    }
}
