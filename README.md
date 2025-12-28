# YaarScript: A Modern Educational Compiler

> A sophisticated multi-phase compiler written in Rust, designed to demonstrate advanced compiler construction techniques through semantic analysis, intermediate representation optimization, and code execution.

---

## Project Overview

**YaarScript** is an educational compiler that parses a C-like language and compiles it to **Three-Address Code (TAC)** with sophisticated intermediate representation optimizations. The compiler demonstrates modern compiler theory concepts including:

- **Hybrid parsing architecture** combining Recursive Descent and Pratt Parsing
- **Multi-phase semantic analysis** with scope and type checking
- **Advanced IR optimization** with constant folding, dead code elimination, and peephole optimization
- **Execution engine** capable of interpreting compiled TAC instructions

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
│                         Source Code                              │
│                      (test_input.txt)                            │
└────────────────────────────┬────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  1. LEXICAL ANALYSIS (Lexer)                                    │
│     - Tokenization: Source → Tokens                             │
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
└────────────────────────────┬────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  5. IR GENERATION (TACGenerator)                                │
│     - Conversion: AST → Three-Address Code                      │
│     - Operands: Temps, Vars, Literals, Labels                   │
│     - Instructions: 13 TAC instruction types                    │
│     - Features: Enum constant optimization                      │
└────────────────────────────┬────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  6. IR OPTIMIZATION (IROptimizer) - Multi-Pass                  │
│     - Constant Folding                                          │
│     - Constant Propagation                                      │
│     - Copy Propagation                                          │
│     - Peephole Optimization                                     │
│     - Dead Code Elimination (DCE)                               │
│     - Iterative fixed-point optimization                        │
└────────────────────────────┬────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  7. EXECUTION (ExecutionEngine)                                 │
│     - TAC interpretation                                        │
│     - Runtime evaluation                                        │
│     - Output printing                                           │
└──────────────────────────── ┬────────────────────────────────────┘
                              │
                              ▼
                    ┌─────────────────────┐
                    │   Program Output    │
                    └─────────────────────┘
```

### Architectural Design Patterns

| Phase | Pattern | Implementation |
|-------|---------|-----------------|
| **Parser** | Hybrid Parsing | Recursive Descent (statements) + Pratt (expressions) |
| **Scope Analyzer** | Two-Pass Analysis | Declaration collection + Validation |
| **Type Checker** | Scope-Aware Traversal | Reuses symbol table with scope navigation |
| **TAC Generator** | AST Visitor | Linear instruction stream generation |
| **IR Optimizer** | Multi-Pass Fixed-Point | Repeated passes until fixed-point convergence |

---

## Features

### ✨ Language Features
- ✅ **Data Types**: `int`, `float`, `bool`, `char`, `void`
- ✅ **Variables**: Global and local scope with qualifiers (`const`, `global`)
- ✅ **Functions**: Prototypes, definitions, return types, parameters
- ✅ **Enumerations**: Named constants with automatic integer mapping
- ✅ **Operators**: 
  - Arithmetic: `+`, `-`, `*`, `/`, `%`
  - Logical: `&&`, `||`, `!`
  - Bitwise: `&`, `|`, `^`, `<<`, `>>`
  - Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
  - Unary: `-`, `!`, `++`, `--` (prefix/postfix)
  - Assignment: `=`
- ✅ **Control Flow**:
  - Conditional: `if`, `else if`, `else`
  - Loops: `while`, `for`
  - Switching: `switch` with `case` and `default`
  - Jump: `break`, `return`
- ✅ **I/O**: `print()` function for output

### 📊 Compiler Features
- ✅ **Hybrid Parser**: Combines Recursive Descent and Pratt parsing for optimal efficiency
- ✅ **Comprehensive Error Reporting**: 
  - 1 Lexical error type
  - 5+ Syntax error types
  - 8 Scope error types
  - 17 Type error types
- ✅ **Symbol Table Management**: Hierarchical scope frames with forward reference support
- ✅ **Strict Type Checking**: No implicit conversions, explicit type validation
- ✅ **TAC Generation**: Enum-optimized three-address code
- ✅ **Multi-Pass Optimization**:
  - Constant folding (arithmetic evaluation)
  - Constant propagation (literal substitution)
  - Copy propagation (variable aliasing)
  - Peephole optimization (algebraic simplifications)
  - Dead code elimination (recursive analysis)
- ✅ **Direct Execution**: TAC interpreter for immediate program execution

---

## Compiler Phases

### Phase 1: Lexical Analysis (Lexer)
**Documentation**: [See docs/README](https://github.com/BazilSuhail/YaarScript/blob/master/docs/LEXER.md)  
**Test Suite**: [tests/lexer/](tests/lexer/)

Tokenizes source code into a stream of tokens. Handles:
- Keywords, identifiers, literals
- Single/multi-character operators
- String and character literals
- Comments and whitespace

---

### Phase 2: Syntax Analysis (Parser)
**Documentation**: [See docs/PARSER.md](https://github.com/BazilSuhail/YaarScript/blob/master/docs/PARSER.md)  
**Test Suite**: Various parser test cases

Constructs an Abstract Syntax Tree using:
- **Recursive Descent** for statements and declarations
- **Pratt Parsing** for expressions with operator precedence

**Key Components**:
- `parse_program()`: Entry point, parses declarations and main block
- `parse_statement()`: Parses individual statements
- `parse_expression()`: Pratt parser engine for expressions
- `ast_printer.rs`: Colored tree visualization tool

**Example**:
```c
// Source Code
int x = 5 + 10 * 2;

// AST Representation
VarDecl(int, "x")
  ├─ InitExpr
      └─ BinaryExpr(+)
          ├─ IntLiteral(5)
          └─ BinaryExpr(*)
              ├─ IntLiteral(10)
              └─ IntLiteral(2)
```

---

### Phase 3: Scope Analysis
**Documentation**: [See docs/SCOPE_ANALYZER.md](https://github.com/BazilSuhail/YaarScript/blob/master/docs/SCOPE_ANALYZER.md)  
**Test Suite**: [tests/scope/](tests/scope/)

Analyzes symbol declarations and manages lexical scopes through two passes:

1. **Declaration Collection**: Gathers all top-level declarations
2. **Validation**: Performs scope analysis and error detection

**Detects 8 Error Types**:
1. `UndeclaredVariableAccessed` - Variable used but never declared
2. `UndefinedFunctionCalled` - Function called but never defined
3. `VariableRedefinition` - Variable declared multiple times
4. `FunctionPrototypeRedefinition` - Duplicate function prototype
5. `ConflictingFunctionDefinition` - Function with different signature
6. `ConflictingDeclaration` - Name used for different symbol types
7. `ParameterRedefinition` - Same parameter name used twice
8. `InvalidForwardReference` - Symbol referenced before declaration

**Valid Example**:
```c
int add(int a, int b);  // Forward declaration

int add(int a, int b) {  // Definition matches prototype
    return a + b;
}

main {
    int result = add(10, 20);  // Valid function call
    print(result);
}
```

**Error Example**:
```c
main {
    int x = 10;
    int x = 20;  // ERROR: VariableRedefinition
}
```

---

### Phase 4: Type Checking
**Documentation**: [See docs/TYPE_ANALYZER.md](https://github.com/BazilSuhail/YaarScript/blob/master/docs/TYPE_ANALYZER.md)  
**Test Suite**: [tests/type/](tests/type/)

Ensures type safety through strict type checking:

**Detects 17 Error Types**:
1. `ErroneousVarDecl` - Variable declared as void
2. `FnCallParamCount` - Wrong number of function arguments
3. `FnCallParamType` - Wrong function argument type
4. `ErroneousReturnType` - Return type mismatch
5. `ExpressionTypeMismatch` - Assignment type mismatch
6. `ExpectedBooleanExpression` - Boolean expected elsewhere
7. `ErroneousBreak` - Break outside loop
8. `NonBooleanCondStmt` - Non-boolean condition in if/while/for
9. `AttemptedBoolOpOnNonBools` - Logical operators on non-bools
10. `AttemptedBitOpOnNonInt` - Bitwise operators on non-integers
11. `AttemptedShiftOnNonInt` - Shift operators on non-integers
12. `AttemptedAddOpOnNonNumeric` - Arithmetic on non-numeric types
13. `ReturnStmtInVoid` - Return value in void function
14. `NonBooleanSwitchExpression` - Non-int/char switch expression
15. `InvalidCaseValueType` - Case type mismatch
16. `IncrementDecrementOnNonInt` - ++/-- on non-integers
17. `NotOnNonBool` - NOT operator on non-boolean

**Valid Example**:
```c
int calculateArea(int width, int height) {
    int area = width * height;
    return area;  // Correct return type
}

bool isPassing(float score) {
    if (score >= 0.75) {  // Boolean condition ✓
        return true;
    }
    return false;
}

main {
    int result = calculateArea(5, 10);  // Correct arg types ✓
    bool pass = isPassing(0.85);        // Type compatible ✓
}
```

**Error Example**:
```c
int add(int a, int b) {
    return a + b;
}

main {
    int x = add(10, 3.14);  // ERROR: Float to int parameter
    if (x) {                // ERROR: int in boolean context
        print("Test");
    }
}
```

---

### Phase 5: TAC Generation
**Documentation**: [See docs/TAC_GENERATION.md](https://github.com/BazilSuhail/YaarScript/blob/master/docs/TAC_GENERATION.md)  
**Test Suite**: [tests/ir-generation/](tests/ir-generation/)

Transforms the AST into Three-Address Code:

**TAC Instruction Set**:
| Instruction | Example | Purpose |
|------------|---------|---------|
| `Declare` | `int x` | Variable declaration with type |
| `Assign` | `x = 10` | Value assignment |
| `Binary` | `t0 = a + b` | Binary operations |
| `Unary` | `t1 = !x` | Unary operations |
| `Label` | `L0:` | Jump target |
| `Goto` | `goto L0` | Unconditional jump |
| `IfTrue` | `ifTrue x goto L1` | Conditional jump (true) |
| `IfFalse` | `ifFalse x goto L0` | Conditional jump (false) |
| `Call` | `Call(func_name, args)` | Function invocation |
| `Param` | `param x` | Function parameter |
| `Return` | `return x` | Function exit with value |
| `FuncStart` | `FuncStart(name, params)` | Function entry point |
| `FuncEnd` | `FuncEnd` | Function exit marker |
| `Print` | `print x` | Output instruction |

**Operand Types**:
```rust
enum Operand {
    Temp(usize),           // Virtual register: t0, t1, etc.
    Var(String),           // Named variable
    Int(i64),              // Integer literal
    Float(f64),            // Float literal
    Bool(bool),            // Boolean literal
    Char(char),            // Character literal
    String(String),        // String literal
    Label(String),         // Jump target
    None,                  // Void/no operand
}
```

**Example TAC Generation**:

*Source*:
```c
for (int i = 0; i < 3; i++) {
    print(i);
}
```

*Generated TAC*:
```
int i = 0
L1:
t4 = i Lt 3
ifFalse t4 goto L2
print i
i = i Plus 1
goto L1
L2:
```

---

### Phase 6: IR Optimization
**Documentation**: [See docs/IR_OPTIMIZATION.md](https://github.com/BazilSuhail/YaarScript/blob/master/docs/IR_OPTIMIZATION.md)  
**Test Suite**: [tests/ir-generation/ir-optimization.txt](tests/ir-generation/ir-optimization.txt)

Multi-pass optimization engine that improves code efficiency:

#### 6.1 Constant Folding
Evaluates arithmetic expressions at compile-time:
```
Input:  t0 = 20 * 5
Output: t0 = 100
```

#### 6.2 Constant Propagation
Substitutes known constants with their literal values:
```
Input:  x = 10
        t0 = x + 5

Output: x = 10
        t0 = 10 + 5  // Next pass: constant folding → 15
```

#### 6.3 Copy Propagation
Replaces variables with their source values:
```
Input:  val = original
        t0 = val * 2

Output: val = original
        t0 = original * 2
```

#### 6.4 Peephole Optimization
Applies algebraic identities and control flow simplifications:

**Algebraic Optimizations**:
- $x + 0 \rightarrow x$
- $x - 0 \rightarrow x$
- $x - x \rightarrow 0$
- $x \times 1 \rightarrow x$
- $x \times 0 \rightarrow 0$
- $x \div 1 \rightarrow x$

**Control Flow**:
```
Input:  goto L1
        L1:

Output: (goto removed - execution falls through)
```

#### 6.5 Dead Code Elimination (DCE)
Removes unused instructions through mark-and-sweep:

**Example**:
```c
// Source
main {
    int a = 10;
    int b = a + 5;
    int c = b * 1 + 0;  // Optimized to: c = b
    int d = 999;        // Never used → REMOVED
    print(c);           // c is used
}
```

**Optimization Trace**:
```
Pass 1: Constant Folding
  int c = b * 1 + 0  →  int c = b

Pass 2: Copy Propagation
  int b = a + 5
  int c = b          →  int c = a + 5

Pass 3: Dead Code Elimination
  int d = 999 is unused  →  REMOVED

Pass 4: Constant Propagation
  int a = 10
  int c = 10 + 5         →  int c = 15

Final TAC:
  int a = 10
  int c = 15
  print c
```

**Iterative Fixed-Point**: Optimizations repeat until no further changes occur.

---

## Keywords & C/C++ Relations

YaarScript keywords are inspired by C/C++ with key differences:

| Keyword | Type | C/C++ Equivalent | Purpose |
|---------|------|------------------|---------|
| `int` | Type | `int` | 64-bit signed integer |
| `float` | Type | `float` | 64-bit floating-point |
| `bool` | Type | `bool` | Boolean value (true/false) |
| `char` | Type | `char` | Single character |
| `void` | Type | `void` | No return value |
| `const` | Qualifier | `const` | Read-only variable |
| `global` | Qualifier | Static global scope | Global variable |
| `if` | Control | `if` | Conditional execution |
| `else` | Control | `else` | Alternative execution |
| `while` | Control | `while` | Loop while condition true |
| `for` | Control | `for` | Iterative loop |
| `switch` | Control | `switch` | Multi-way branch |
| `case` | Control | `case` | Switch case label |
| `default` | Control | `default` | Default switch case |
| `break` | Control | `break` | Break from loop/switch |
| `return` | Control | `return` | Return from function |
| `enum` | Type | `enum` | Named constant type |
| `main` | Block | `main()` | Entry point (special) |
| `print` | I/O | `printf/cout` | Output function |

### Key Differences from C/C++
| Feature | C/C++ | YaarScript | Reason |
|---------|-------|-----------|--------|
| **Type Safety** | Implicit conversions allowed | Strict, no implicit conversion | Educational rigor |
| **Main Function** | Function declaration | Special block | Simplification |
| **Prototypes** | Optional | Required pattern | Explicit forward refs |
| **Enums** | Global scope only | Global scope only | Enforced structure |
| **Type Qualifiers** | Multiple (`const`, `volatile`, etc.) | `const`, `global` | Focused subset |
| **Pointers** | Fully featured | Not supported | Simplification |
| **Memory** | Manual allocation (`malloc`) | Automatic | Educational focus |

---

## Operator Precedence Table

The parser enforces operator precedence from lowest to highest binding power:

| Level | Precedence | Operators | Associativity | Example |
|-------|-----------|-----------|---------------|---------|
| 0 | **Lowest** | - | - | - |
| 1 | Assignment | `=` | Right-to-left | `a = b = c` → `a = (b = c)` |
| 2 | Logical OR | `\|\|` | Left-to-right | `a \|\| b \|\| c` → `(a \|\| b) \|\| c` |
| 3 | Logical AND | `&&` | Left-to-right | `a && b && c` → `(a && b) && c` |
| 4 | Equality | `==`, `!=` | Left-to-right | `a == b != c` → `(a == b) != c` |
| 5 | Comparison | `<`, `>`, `<=`, `>=` | Left-to-right | `a < b <= c` → `(a < b) <= c` |
| 6 | Bitwise | `&`, `\|`, `^`, `<<`, `>>` | Left-to-right | `a & b \| c` → `(a & b) \| c` |
| 7 | Additive | `+`, `-` | Left-to-right | `a + b - c` → `(a + b) - c` |
| 8 | Multiplicative | `*`, `/`, `%` | Left-to-right | `a * b / c` → `(a * b) / c` |
| 9 | Unary | `-`, `!`, `++`, `--` (prefix) | Right-to-left | `!-x` → `!(-x)` |
| 10 | Postfix | `++`, `--` (postfix) | Left-to-right | `x++--` → `(x++)--` |
| 11 | **Call** | `()` | **Highest** | `f(x + 1)` → `f((x + 1))` |

**Complex Precedence Example**:
```c
// Parse: a + b * c == d || e
//
// Step-by-step:
// 1. b * c        (precedence 8)
// 2. a + (b*c)    (precedence 7)
// 3. (a + b*c) == d  (precedence 4)
// 4. ((a + b*c) == d) || e  (precedence 2)
//
// Final: (a + (b * c)) == d) || e
```

---

## Test Cases & Examples

### Scope Analysis Tests

**Valid Example** ([tests/scope/valid.txt](tests/scope/valid.txt)):
```c
enum Color {
    Red,
    Green,
    Blue
};

int globalCounter = 0;

void printMessage(int code);

int add(int a, int b) {
    int result = a + b;
    return result;
}

Color getColor(int index) {
    if (index == 0) {
        return Red;
    }
    return Blue;
}

main {
    int x = 42;
    float y = 3.14;
    int sum = add(x, 10);
    printMessage(sum);
    Color c = getColor(1);
}
```
✅ **No scope errors** - All symbols declared before use, no redefinitions

**Error Examples** ([tests/scope/erros.txt](tests/scope/erros.txt)):
```c
// ERROR 1: Variable Redefinition
int x = 10;
int x = 20;  // ❌ Redefinition in same scope

// ERROR 2: Undeclared Variable Access
int result = unknownVar + 5;  // ❌ unknownVar never declared

// ERROR 3: Undefined Function Call
int value = nonExistentFunc(10);  // ❌ Function never defined

// ERROR 4: Conflicting Declaration
enum Size { Small, Large };
int Size = 10;  // ❌ Size already enum type
```

### Type Checking Tests

**Valid Example** ([tests/type/valid.txt](tests/type/valid.txt)):
```c
enum Status {
    Active,
    Inactive,
    Pending
};

int globalId = 100;
float threshold = 0.75;

int calculateArea(int width, int height) {
    int area = width * height;
    return area;  // ✓ Correct return type (int)
}

bool isPassing(float score) {
    if (score >= threshold) {  // ✓ Boolean condition
        return true;
    }
    return false;
}

void logStatus(int s) {
    switch (s) {
        case 1:
            print("Status is Active");
            break;
        case 2:
            print("Status is Inactive");
            break;
        default:
            print("Status is Pending");
    }
}

main {
    int area = calculateArea(5, 10);    // ✓ Args match params
    bool pass = isPassing(0.85);        // ✓ Type compatible
    logStatus(area);                     // ✓ Type compatible
}
```
✅ **No type errors** - All types match, operations valid

**Error Examples** ([tests/type/error.txt](tests/type/error.txt)):
```c
enum Color { Red, Green };

// ERROR 1: Erroneous Variable Declaration
void invalidVar;  // ❌ Variable cannot be void

int add(int a, int b) {
    return a + b;
}

main {
    int i = 10;
    float f = 3.14;
    
    // ERROR 2: Function Parameter Count Mismatch
    add(10);  // ❌ Expected 2 args, got 1
    
    // ERROR 3: Function Parameter Type Mismatch
    add(10, f);  // ❌ Expected int, got float
    
    // ERROR 4: Expression Type Mismatch
    i = 3.14;  // ❌ Cannot assign float to int
    
    // ERROR 5: Erroneous Break
    break;  // ❌ break outside loop/switch
    
    // ERROR 6: Non-Boolean Condition Statement
    if (i) {  // ❌ if expects bool, got int
        print("wont work");
    }
    
    // ERROR 7: Attempted Boolean Operation on Non-Bools
    int result = 10 && 20;  // ❌ && expects bool operands
    
    // ERROR 8: Attempted Bitwise Operation on Non-Int
    float bitResult = f & 2.2;  // ❌ & expects int operands
    
    // ERROR 9: Attempted Shift on Non-Int
    float shiftResult = f << 2;  // ❌ << expects int operands
    
    // ERROR 10: Attempted Addition on Non-Numeric
    bool b = true;
    int sum = b + 5;  // ❌ Cannot add bool and int
}
```

### IR Optimization Tests

**Example Optimization** ([tests/ir-generation/ir-optimization.txt](tests/ir-generation/ir-optimization.txt)):

*Source Code*:
```c
main {
    int a = 10;
    int b = a + 5;
    int c = b * 1 + 0;
    int d = 999;  // Never used
    print(c);
}
```

*TAC Before Optimization*:
```
int a
a = 10
int b
t0 = a Plus 5
b = t0
int c
t1 = b Mult 1
t2 = t1 Plus 0
c = t2
int d
d = 999
print c
```

*TAC After Optimization*:
```
Pass 1: Constant Folding
  b * 1 + 0 → b

Pass 2: Constant Propagation
  a = 10; b = a + 5 → a = 10; b = 15

Pass 3: Copy Propagation
  c = b → c = 15

Pass 4: Dead Code Elimination
  d = 999 removed (never used)

Final TAC:
  int a
  a = 10
  int b
  b = 15
  int c
  c = 15
  print c
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
# Execute compiler on test file
cargo run -- test_input.txt
```

### Run with Optimizations
Edit your test case in `test_input.txt`, then:
```bash
cargo run --release
```

### Testing Individual Phases

#### Test Scope Analysis
Create test file and check [tests/scope/](tests/scope/) examples

#### Test Type Checking
Use examples from [tests/type/](tests/type/)

#### Test IR Optimization
Review [tests/ir-generation/ir-optimization.txt](tests/ir-generation/ir-optimization.txt)

---

## Comprehensive Documentation

For detailed information about each compiler phase:

| Phase | Documentation | Details |
|-------|-------------|---------|
| **Lexer** | `docs/LEXER.md` | Tokenization, error handling |
| **Parser** | [docs/PARSER.md](./docs/PARSER.md) | Hybrid parsing, AST structure, precedence |
| **Scope Analyzer** | [docs/SCOPE_ANALYZER.md](./docs/SCOPE_ANALYZER.md) | Symbol table, scope frames, error types |
| **Type Analyzer** | [docs/TYPE_ANALYZER.md](./docs/TYPE_ANALYZER.md) | Type rules, error detection, type system |
| **TAC Generation** | [docs/TAC_GENERATION.md](./docs/TAC_GENERATION.md) | Instruction set, code structure, operands |
| **IR Optimization** | [docs/IR_OPTIMIZATION.md](./docs/IR_OPTIMIZATION.md) | Optimization passes, algorithms, examples |

---

## Project Structure

```
YaarScript/
├── Cargo.toml                 # Project manifest
├── README.md                  # This file
├── test_input.txt            # Input program
│
├── src/
│   ├── main.rs               # Compiler driver
│   ├── lib.rs                # Module exports
│   ├── error.rs              # Error reporting
│   │
│   ├── core/                 # Core data structures
│   │   ├── token.rs          # Token definitions
│   │   └── ast.rs            # AST node types
│   │
│   ├── lexer/                # Lexical analysis
│   │   └── lexer.rs
│   │
│   ├── parser/               # Syntax analysis
│   │   ├── parser.rs         # Pratt parser
│   │   └── ast_printer.rs    # Tree visualization
│   │
│   ├── semantics/            # Semantic analysis
│   │   ├── scope.rs          # Scope checking
│   │   └── type_checker.rs   # Type checking
│   │
│   ├── ir_pipeline/          # Intermediate representation
│   │   ├── tac.rs            # TAC generation
│   │   └── tac_optimizer.rs  # IR optimization
│   │
│   └── codegen/              # Code generation
│       └── execution.rs      # TAC execution engine
│
├── docs/                     # Phase documentation
│   ├── PARSER.md
│   ├── SCOPE_ANALYZER.md
│   ├── TYPE_ANALYZER.md
│   ├── TAC_GENERATION.md
│   └── IR_OPTIMIZATION.md
│
└── tests/                    # Test cases by phase
    ├── scope/
    │   ├── valid.txt
    │   └── erros.txt
    ├── type/
    │   ├── valid.txt
    │   └── error.txt
    ├── ir-generation/
    │   └── ir-optimization.txt
    └── execution/
        └── exec.txt
```

---

## Contributing Considerations

This is an educational compiler project designed to teach:
- Compiler architecture and design patterns
- Intermediate representation techniques
- Optimization algorithms
- Semantic analysis strategies
- Type system implementation

Perfect as a:
- Learning resource for compiler construction
- Reference implementation for compiler phases
- Template for custom language design
- Advanced compiler optimization study

---

**Created**: February 2026  
**Language**: Rust 2024  
**Type**: Educational Compiler  
**Status**: Feature Complete