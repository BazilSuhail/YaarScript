# Lexer Module Documentation

## Overview

The **Lexer** (Lexical Analyzer) is the first phase of the YaarScript compiler pipeline. It transforms raw source code text into a stream of meaningful **tokens**—the fundamental building blocks that the parser uses to construct the Abstract Syntax Tree.

With its uniquely flavored Urdu-slang keywords, the lexer identifies identifiers, literals, and symbols while handling whitespace and comments. It produces a linear sequence of tokens with attached position information (line and column numbers) crucial for accurate error reporting.

---

## Architecture

### Input & Output
```text
┌──────────────────────────────┐
│    Source Code (String)      │
│  ┌────────────────────────┐  │
│  │ number x = 10;         │  │
│  │ float y = 3.14;        │  │
│  │ agar (x > 5) { ... }   │  │
│  └────────────────────────┘  │
└──────────────────────────────┘
              │
              ▼
┌──────────────────────────────┐
│  Lexer: Character Analysis   │
│  - Categorization            │
│  - Token recognition         │
│  - Slang-to-Token Mapping    │
└──────────────────────────────┘
              │
              ▼
┌──────────────────────────────┐
│   Token Stream               │
│  ┌────────────────────────┐  │
│  │ Token(T_INT, "number") │  │
│  │ Token(Ident, "x")      │  │
│  │ Token(Assign, "=")     │  │
│  │ Token(IntLit, "10")    │  │
│  │ Token(Semi, ";")       │  │
│  │ ...                    │  │
│  │ Token(EOF, "")         │  │
│  └────────────────────────┘  │
└──────────────────────────────┘
```

### Recognition Strategies

##### Keyword & Identifier Recognition
The Lexer distinguishes between:
- **Urdu Slang Keywords**: `yaar`, `number`, `faisla`, `agar`, `warna`, `jabtak`, etc.
- **Intrinsic Functions**: `bolo`, `suno`, `waqt`, `ittifaq` are lexed as specific token types.
- **Identifiers**: User-defined names matching `[a-zA-Z_][a-zA-Z0-9_]*`.

---

## Token System

### TokenType Enumeration (Partial Mapping)

| Keyword | Token Type | Purpose |
|---------|------------|---------|
| `number` | `T_INT` | 64-bit signed integer |
| `float` | `T_FLOAT` | 64-bit floating-point |
| `faisla` | `T_BOOL` | Boolean type |
| `lafz` | `T_STRING` | String type |
| `khaali` | `T_VOID` | Void type |
| `agar` | `T_IF` | Conditional statement |
| `warna` | `T_ELSE` | Alternative branch |
| `jabtak` | `T_WHILE` | Loop statement |
| `dohrao` | `T_FOR` | Iterative loop |
| `intekhab` | `T_SWITCH` | Multi-way branch |
| `yaar` | `T_MAIN` | Entry point block |
| `pakka` | `T_CONST` | Constant qualifier |
| `bas_kar` | `T_BREAK` | Loop/switch exit |
| `wapsi` | `T_RETURN` | Function return |
| `sahi` | `T_BOOLLIT` | Boolean literal true |
| `galat` | `T_BOOLLIT` | Boolean literal false |

### Operators
| Lexeme | Token Type | Operator Type |
|--------|------------|---------------|
| `**`   | `T_POWER`  | Multiplication (Exponentiation) |
| `==`, `!=` | `T_EQUALOP`, `T_NE` | Equality |
| `<`, `>` | `T_LT`, `T_GT` | Comparison |
| `<=`, `>=` | `T_LE`, `T_GE` | Comparison |
| `&&`, `\|\|`, `!` | `T_AND`, `T_OR`, `T_NOT` | Logical |
| `++`, `--` | `T_INCREMENT`, `T_DECREMENT` | Unary |
| `=` | `T_ASSIGNOP` | Assignment |

---

## Lexical Rules

### Rule 1: Multi-Character Operators (Greedy Matching)
The lexer uses greedy matching for multi-character sequences:
- `*` followed by `*` → `**` (Power operator)
- `+` followed by `+` → `++` (Increment)
- `=` followed by `=` → `==` (Equality)

### Rule 2: Floating Point Literals
- **Format**: `[0-9]+\.[0-9]+`
- Must have digits on both sides of the decimal point.
- Examples: `3.14`, `0.5`, `123.000`

---

## Examples

### Example 1: Simple Variable Declaration
```rust
number count = 42;
```
**Token Stream**:
- `Token(T_INT, "number", line: 1, col: 1)`
- `Token(Identifier, "count", line: 1, col: 8)`
- `Token(T_ASSIGNOP, "=", line: 1, col: 14)`
- `Token(T_INTLIT, "42", line: 1, col: 16)`
- `Token(T_SEMICOLON, ";", line: 1, col: 18)`

### Example 2: Control Flow with Urdu Slang
```rust
agar (sahi) {
    bolo("Acha!");
}
```
**Relevant Tokens**:
- `Token(T_IF, "agar", ...)`
- `Token(T_BOOLLIT, "true", ...)` // Normalized value
- `Token(T_LBRACE, "{", ...)`
- `Token(T_PRINT, "bolo", ...)`

### Example 3: Exponentiation Operator
```rust
number result = 2 ** 8;
```
**Relevant Tokens**:
- `Token(T_INTLIT, "2", ...)`
- `Token(T_POWER, "**", ...)`
- `Token(T_INTLIT, "8", ...)`

---

## Implementation Details

### State Variables
```rust
pub struct Lexer {
    input: Vec<char>,            // Source code as character vector
    pos: usize,                  // Byte offset
    line: usize,                 // Line tracking
    column: usize,               // Column tracking
    keywords: HashMap<String, TokenType>, // Slang mapping table
}
```

### Key Methods
- `tokenize()`: High-level loop producing the vector of tokens.
- `next_token()`: Identifies the next logical unit from current position.
- `try_match_operator()`: Specifically handles multi-char sequences like `**` and `==`.
- `try_match_identifier()`: Matches words and consults the `keywords` hash-map for slang translations.
