# Parser Module Documentation

## Overview
The Parser is the core of the syntax analysis phase. It takes a stream of tokens from the Lexer and constructs an **Abstract Syntax Tree (AST)**. Our compiler utilizes a **Hybrid Architectural Design**, combining the clarity of **Recursive Descent** for high-level structural constructs (like `agar`, `jabtak`, and `yaar`) with the power of **Pratt Parsing** for complex expression trees.

---

## Architecture

### 1. High-Level Recursive Descent
Methods like `parse_program()`, `parse_statement()`, and `parse_block()` navigate the token stream by matching expected keywords:
- Seeing `agar` immediately triggers `parse_if_statement`.
- Seeing `dohrao` triggers `parse_for_statement`.

### 2. Expression Parsing via Pratt Algorithm
For expressions where arithmetic precedence and associativity (e.g., `a + b * c ** d`) make traditional recursive descent cumbersome, we utilize **Pratt Parsing** (Top-Down Operator Precedence).

### 3. Operator Precedence Table
The parser implements the following 12 precedence levels, including our high-power exponentiation operator:

| Level | Precedence | Operators |
|-------|------------|-----------|
| 1 | Assignment | `=` |
| 2 | Logical OR | `||` |
| 3 | Logical AND | `&&` |
| 4 | Equality | `==`, `!=` |
| 5 | Comparison | `<`, `>`, `<=`, `>=` |
| 6 | Bitwise | `&`, `|`, `^`, `<<`, `>>` |
| 7 | Term | `+`, `-` |
| 8 | Factor | `*`, `/`, `%` |
| 9 | **Power** | `**` |
| 10 | Unary | `-`, `!`, `++`, `--` (prefix) |
| 11 | Postfix | `++`, `--` (postfix) |
| 12 | Call | `()` |

---

## Abstract Syntax Tree (AST)

The AST is the internal representation of the program structure. 

### Custom Nodes for Intrinsic Utilities:
- `ReadExpr`: Handles the `suno()` (read standard input) syntax.
- `TimeExpr`: Handles the `waqt()` (system time) syntax.
- `RandomExpr`: Handles the `ittifaq(min, max)` syntax.

---

## Examples & AST Visualization

### 1. Combined Operator Precedence
The Pratt parser respects the high precedence of the power operator.

**Source:**
```rust
number x = 5 + 10 * 2 ** 3;
```

**AST Representation:**
```text
VarDecl(number, "x")
  BinaryExpr(+)
    IntLiteral(5)
    BinaryExpr(*)
      IntLiteral(10)
      BinaryExpr(**)
        IntLiteral(2)
        IntLiteral(3)
```

### 2. Control Flow (Agar-Warna)
```rust
agar (x > 0) {
    bolo("Positive");
} warna {
    bolo("Non-positive");
}
```

**AST Representation:**
```text
IfStmt
  Condition: BinaryExpr(>)
  IfBody: PrintStmt
  ElseBody: PrintStmt
```

---

## Language Grammar (EBNF Snippets)

### Program & Declarations
```ebnf
Program          ::= Declaration* MainDecl?
VarDecl          ::= "pakka"? "number" Identifier ("=" Expression)? ";"
MainDecl         ::= "yaar" "{" Statement* "}"
EnumDecl         ::= "qism" Identifier "{" Identifier ("," Identifier)* "}" ";"
```

### Control Flow
```ebnf
IfStmt           ::= "agar" "(" Expression ")" Block ("warna" Block)?
WhileStmt        ::= "jabtak" "(" Expression ")" Block
ForStmt          ::= "dohrao" "(" (VarDecl | ExpressionStmt | ";") (Expression)? ";" (Expression)? ")" Block
DoWhileStmt      ::= "karo" Block "jabtak" "(" Expression ")" ";"
```

---

## Testing the Parser
The parser is tested using comprehensive input files like `test_input.yaar`. These tests verify:
1. **Precedence**: `a + b * c ** d` is `a + (b * (c ** d))`.
2. **Nesting**: Deep `agar-warna` and `dohrao` structures.
3. **Slang Accuracy**: Correct keyword-to-node mapping for Urdu slang.

### Running a Test:
```bash
cargo run -- test_input.yaar
```