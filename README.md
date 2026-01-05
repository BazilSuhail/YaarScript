# YaarScript: an Urdu-Slang Multi-phase Compiler

> A sophisticated multi-phase compiler written in Rust, designed to demonstrate advanced compiler construction techniques through semantic analysis, intermediate representation optimization, and code execution—now with a uniquely fun Urdu-slang flavored syntax.

## 📋 Table of Contents

- [Project Overview](#project-overview)
- [Tech Stack](#tech-stack)
- [Architecture & Compilation Pipeline](#architecture--compilation-pipeline)
- [Features](#features)
- [Compiler Phases](#compiler-phases)
- [Urdu Slang Keywords & Traditional Equivalents](#urdu-slang-keywords--traditional-equivalents)
- [Operator Precedence Table](#operator-precedence-table)
- [Compiler Optimizations](#compiler-optimizations)
- [Test Cases & Examples](#test-cases--examples)
- [Building & Running](#building--running)

---

## Project Overview

**YaarScript** is an educational compiler that parses an Urdu-slang flavored, C-like language and compiles it to **Three-Address Code (TAC)** with sophisticated intermediate representation optimizations. The compiler demonstrates modern compiler theory concepts including:

- **Hybrid parsing architecture** combining Recursive Descent and Pratt Parsing
- **Multi-phase semantic analysis** with scope and type checking
- **Advanced IR optimization** with constant folding, dead code elimination, and peephole optimization
- **Execution engine** capable of interpreting compiled TAC instructions
- **Fun Localization**: Keywords and slang that make coding more engaging and relatable

This project serves as a reference implementation for compiler design patterns and offers a complete compilation pipeline from source code to execution.

---

## Tech Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| **Language** | Rust (2024 edition) | Memory safety, performance, and low-level control |
| **Module System** | Cargo workspace | Project organization and dependency management |
| **Parsing** | Pratt Parser + Recursive Descent | Efficient expression and statement parsing |
| **Intermediate Code** | Three-Address Code (TAC) | Platform-independent intermediate representation |
| **Analysis** | Symbol Table + Type System | Semantic validation and type checking |
| **Optimization** | Multi-pass IR optimizer | Performance enhancement at compile-time |
| **Execution** | Custom TAC interpreter | Direct program execution without code generation |

---

## Architecture & Compilation Pipeline

```
┌─────────────────────────────────────────────────────────────────┐
│                         Source Code                             │
│                      (test_input.yaar)                          │
└────────────────────────────┬────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  1. LEXICAL ANALYSIS (Lexer)                                    │
│     - Tokenization: Source → Tokens                             │
│     - Mapping: Urdu Slang → Internal Tokens                     │
│     - Error Reporting: Invalid symbols                          │
└────────────────────────────┬────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. SYNTAX ANALYSIS (Parser)                                    │
│     - Parsing: Tokens → Abstract Syntax Tree (AST)              │
│     - Algorithm: Hybrid (Recursive Descent + Pratt)             │
│     - Output: Colored AST visualization                         │
└────────────────────────────┬────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. SEMANTIC ANALYSIS - SCOPE (ScopeAnalyzer)                   │
│     - Symbol collection and validation                          │
│     - Forward reference checking                                │
│     - Error Detection: 8 types of scope errors                  │
│     - Output: Symbol table (ScopeFrame tree)                    │
└────────────────────────────┬────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  4. SEMANTIC ANALYSIS - TYPE (TypeChecker)                      │
│     - Type compatibility validation                             │
│     - Function signature verification                           │
│     - Error Detection: 17 types of type errors                  │
│     - Strict type checking (no implicit conversions)            │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  5. IR GENERATION (TACGenerator)                                │
│     - Conversion: AST → Three-Address Code                      │
│     - Operands: Temps, Vars, Literals, Labels                   │
│     - Instructions: 16+ TAC instruction types                   │
│     - Features: Power operator, Intrinsic function support      │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  6. IR OPTIMIZATION (IROptimizer) - Multi-Pass                  │
│     - Constant Folding (including Power operator)               │
│     - Constant Propagation                                      │
│     - Copy Propagation                                          │
│     - Peephole Optimization                                     │
│     - Dead Code Elimination (DCE)                               │
│     - Iterative fixed-point optimization                        │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  7. EXECUTION (ExecutionEngine)                                 │
│     - TAC interpretation                                        │
│     - Intrinsic Libraries: Random, Time, Read                   │
│     - Runtime evaluation                                        │
│     - Output printing                                           │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
                    ┌─────────────────────┐
                    │   Program Output    │
                    └─────────────────────┘
```

---

## Features

### ✨ Language Features
- ✅ **Data Types**: `number` (64-bit int), `float`, `faisla` (bool), `char`, `khaali` (void), `lafz` (string)
- ✅ **Variables**: Global and local scope with qualifiers (`pakka` for const)
- ✅ **Functions**: Prototypes, definitions, return types, parameters
- ✅ **Enumerations**: `qism` for named constants with automatic integer mapping
- ✅ **Operators**: 
  - Arithmetic: `+`, `-`, `*`, `/`, `%`, `**` (Power)
  - Logical: `&&`, `||`, `!`
  - Bitwise: `&`, `|`, `^`, `<<`, `>>`
  - Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
  - Unary: `-`, `!`, `++`, `--` (prefix/postfix)
  - Assignment: `=`
- ✅ **Control Flow**:
  - Conditional: `agar` (if), `warna` (else)
  - Loops: `jabtak` (while), `dohrao` (for), `karo` (do)
  - Switching: `intekhab` with `agar_ho` (case) and `aakhir` (default)
  - Jump: `bas_kar` (break), `wapsi` (return)
- ✅ **Standard Library Utilities**: 
  - `bolo()`: Output to console
  - `suno()`: Standard input (returns `number`)
  - `waqt()`: Returns UNIX timestamp
  - `ittifaq(min, max)`: Returns random integer in range

### 📊 Compiler Features
- ✅ **Hybrid Parser**: Combines Recursive Descent and Pratt parsing for optimal efficiency
- ✅ **Detailed Error Reporting**: Comprehensive reporting across all phases
- ✅ **Symbol Table Management**: Hierarchical scope frames with forward reference support
- ✅ **Strict Type Checking**: No implicit conversions, explicit type validation
- ✅ **Multi-Pass Optimization**: High-performance iterative optimizer
- ✅ **Direct Execution**: Integrated TAC interpreter with intrinsic support

---

## Urdu Slang Keywords & Traditional Equivalents

| YaarScript Keyword | Traditional Equivalent | Purpose |
|--------------------|-------------------------|---------|
| `number` | `int` | 64-bit signed integer |
| `float` | `float` | 64-bit floating-point |
| `faisla` | `bool` | Boolean value |
| `lafz` | `string` | String literal |
| `khaali` | `void` | No return value / type |
| `pakka` | `const` | Read-only constant |
| `yaar` | `main` | Primary execution block |
| `agar` | `if` | Conditional execution |
| `warna` | `else` | Alternative execution |
| `jabtak` | `while` | Loop while condition true |
| `dohrao` | `for` | Iterative loop |
| `karo` | `do` | Start of do-while loop |
| `intekhab` | `switch` | Multi-way branching |
| `agar_ho` | `case` | Switch branch label |
| `aakhir` | `default` | Default switch branch |
| `bas_kar` | `break` | Exit loop/switch |
| `wapsi` | `return` | Function return |
| `qism` | `enum` | Enumeration type |
| `bolo` | `print` | Console output |
| `suno` | `read` | Console input |
| `waqt` | `time` | System timestamp |
| `ittifaq` | `random` | Random number generation |
| `sahi` | `true` | Boolean true |
| `galat` | `false` | Boolean false |

---

## Operator Precedence Table

The parser enforces operator precedence from lowest to highest binding power:

| Level | Precedence | Operators | Associativity | Example |
|-------|-----------|-----------|---------------|---------|
| 1 | Assignment | `=` | Right-to-left | `a = b = c` |
| 2 | Logical OR | `\|\|` | Left-to-right | `a \|\| b \|\| c` |
| 3 | Logical AND | `&&` | Left-to-right | `a && b && c` |
| 4 | Equality | `==`, `!=` | Left-to-right | `a == b != c` |
| 5 | Comparison | `<`, `>`, `<=`, `>=` | Left-to-right | `a < b <= c` |
| 6 | Bitwise | `&`, `\|`, `^`, `<<`, `>>` | Left-to-right | `a & b \| c` |
| 7 | Additive | `+`, `-` | Left-to-right | `a + b - c` |
| 8 | Multiplicative | `*`, `/`, `%` | Left-to-right | `a * b / c` |
| 9 | **Power** | `**` | Left-to-right | `a ** b ** c` |
| 10 | Unary | `-`, `!`, `++`, `--` (prefix) | Right-to-left | `!-x` |
| 11 | Postfix | `++`, `--` (postfix) | Left-to-right | `x++--` |
| 12 | **Call** | `()` | **Highest** | `f(x + 1)` |

---

## Test Cases & Examples

### 💎 Generating a Numeric Diamond
Demonstrates nested loops (`dohrao`), I/O (`bolo`), and power operator (`**`).

```rust
yaar {
    number size = 5;
    
    // Top Half
    dohrao (number i = 1; i <= size; i++) {
        dohrao (number s = 1; s <= (size - i); s++) { bolo(" "); }
        dohrao (number j = 1; j <= i; j++) { 
            bolo(" ", i); 
         }
        bolo("\n");
    }
    
    bolo("Aakhir mein power test: ", size ** 2, "\n");
}
```

### ⚡ Sensor Calibration Test
Demonstrates the `karo-jabtak` (do-while) loop and system intrinsics.

```rust
yaar {
    number attempt = 1;
    faisla sensor_ready = galat;

    karo {
        bolo("Checking probe #", attempt, " at time: ", waqt(), "\n");
        agar (attempt >= 10) {
            sensor_ready = sahi;
            bolo("SYSTEM READY!\n");
        } warna {
            attempt++;
        }
    } jabtak (!sensor_ready);
}
```

---

## Building & Running

### Prerequisites
- Rust 2024 edition (or compatible)
- Cargo package manager

### Build
```bash
cargo build --release
```

### Run
```bash
# Execute compiler on Urdu slang file
cargo run -- test_input.yaar
```

---

## Comprehensive Documentation

For detailed information about each compiler phase:

| Phase | Documentation | Details |
|-------|-------------|---------|
| **Lexer** | [docs/LEXER.md](./docs/LEXER.md) | Tokenization, slang mapping, error handling |
| **Parser** | [docs/PARSER.md](./docs/PARSER.md) | Hybrid parsing, AST structure, precedence |
| **Scope Analyzer** | [docs/SCOPE_ANALYZER.md](./docs/SCOPE_ANALYZER.md) | Scope management, symbol validation |
| **Type Checker** | [docs/TYPE_ANALYZER.md](./docs/TYPE_ANALYZER.md) | Strict type system, error detection |
| **IR / Optimization** | [docs/IR_OPTIMIZATION.md](./docs/IR_OPTIMIZATION.md) | TAC generation, fixed-point optimizers |