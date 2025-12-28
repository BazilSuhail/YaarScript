# Lexer Module Documentation

## Overview

The **Lexer** (Lexical Analyzer) is the first phase of the YaarScript compiler pipeline. It transforms raw source code text into a stream of meaningful **tokens**—the fundamental building blocks that the parser uses to construct the Abstract Syntax Tree.

The lexer operates on a character-by-character basis, identifying keywords, operators, identifiers, literals, and symbols while handling whitespace and comments. It produces a linear sequence of tokens with attached position information (line and column numbers) crucial for accurate error reporting.

---

## Architecture

### Input & Output
```
┌──────────────────────────────┐
│    Source Code (String)      │
│  ┌────────────────────────┐  │
│  │ int x = 10;            │  │
│  │ float y = 3.14;        │  │
│  │ if (x > 5) { ... }     │  │
│  └────────────────────────┘  │
└──────────────────────────────┘
              │
              ▼
┌──────────────────────────────┐
│  Lexer: Character Analysis   │
│  - Categorization            │
│  - Token recognition         │
│  - Position tracking         │
└──────────────────────────────┘
              │
              ▼
┌──────────────────────────────┐
│   Token Stream               │
│  ┌────────────────────────┐  │
│  │ Token(Keyword, "int")  │  │
│  │ Token(Ident, "x")      │  │
│  │ Token(Assign, "=")     │  │
│  │ Token(IntLit, "10")    │  │
│  │ Token(Semi, ";")       │  │
│  │ ...                    │  │
│  │ Token(EOF, "")         │  │
│  └────────────────────────┘  │
└──────────────────────────────┘
```

### Core Components

#### 1. **Lexer State Machine**
The lexer maintains:
- **Current position**: Index in the source string
- **Line & column tracking**: For error reporting
- **Character buffer**: Indexed access to source characters
- **Token accumulation**: Building the output token stream

#### 2. **Character Classification**
Characters are categorized into:
- **Whitespace**: Spaces, tabs, newlines (skipped with position tracking)
- **Alphanumeric**: Letters and digits (identifiers/keywords)
- **Operators**: Single/multi-character symbols (`+`, `==`, `<<`, etc.)
- **Delimiters**: Parentheses, braces, brackets, commas, semicolons
- **Quotes**: String and character literal delimiters

#### 3. **Recognition Strategies**

##### Keyword & Identifier Recognition
Lexer distinguishes between:
- **Reserved keywords**: `int`, `float`, `bool`, `void`, `if`, `while`, `for`, etc.
- **Identifiers**: User-defined names starting with letter or underscore
- Strategy: Lookahead comparison with keyword set

##### Numeric Literals
- **Integer literals**: Sequence of digits (e.g., `123`, `0`)
- **Float literals**: Digits with decimal point (e.g., `3.14`, `0.5`)
- **Format**: Rust-like number parsing without scientific notation

##### String & Character Literals
- **Strings**: Enclosed in double quotes `"..."` with escape sequences
- **Characters**: Enclosed in single quotes `'...'`
- **Escape sequences**: `\n`, `\t`, `\\`, `\'`, `\"`

##### Multi-Character Operators
Lexer handles operator sequences:
- **Two-character**: `==`, `!=`, `<=`, `>=`, `&&`, `||`, `++`, `--`, `<<`, `>>`, `+=`, `-=`
- **Single-character backtrack**: If lookahead doesn't form operator, use single char

#### 4. **Comment Handling**
Single-line comments starting with `//` are recognized and skipped:
```c
int x = 10;  // This is a comment
// Entire line comment
int y = 20;
```

---

## Token System

### Token Structure
```rust
struct Token {
    pub token_type: TokenType,    // What type of token is this?
    pub value: String,             // Lexeme (source text)
    pub line: usize,              // Line number (1-indexed)
    pub column: usize,            // Column number (1-indexed)
}
```

### TokenType Enumeration

#### Keywords
| Token Type | Lexeme | Purpose |
|-----------|--------|---------|
| `Keyword::Int` | `int` | Integer type |
| `Keyword::Float` | `float` | Float type |
| `Keyword::Bool` | `bool` | Boolean type |
| `Keyword::Char` | `char` | Character type |
| `Keyword::Void` | `void` | Void type |
| `Keyword::If` | `if` | Conditional statement |
| `Keyword::Else` | `else` | Alternative branch |
| `Keyword::While` | `while` | Loop statement |
| `Keyword::For` | `for` | Iterative loop |
| `Keyword::Switch` | `switch` | Multi-way branch |
| `Keyword::Case` | `case` | Switch case |
| `Keyword::Default` | `default` | Default case |
| `Keyword::Break` | `break` | Loop/switch exit |
| `Keyword::Return` | `return` | Function return |
| `Keyword::Enum` | `enum` | Enumeration type |
| `Keyword::Const` | `const` | Constant qualifier |
| `Keyword::Global` | `global` | Global scope qualifier |
| `Keyword::Main` | `main` | Entry point block |
| `Keyword::Print` | `print` | Output function |
| `Keyword::True` | `true` | Boolean literal |
| `Keyword::False` | `false` | Boolean literal |

#### Operators
| Token Type | Lexeme | Operator Type |
|-----------|--------|---------------|
| `Plus` | `+` | Arithmetic |
| `Minus` | `-` | Arithmetic |
| `Mult` | `*` | Arithmetic |
| `Div` | `/` | Arithmetic |
| `Mod` | `%` | Arithmetic |
| `Assign` | `=` | Assignment |
| `Equal` | `==` | Equality |
| `NotEqual` | `!=` | Equality |
| `Less` | `<` | Comparison |
| `Greater` | `>` | Comparison |
| `LessEqual` | `<=` | Comparison |
| `GreaterEqual` | `>=` | Comparison |
| `And` | `&&` | Logical |
| `Or` | `\|\|` | Logical |
| `Not` | `!` | Logical |
| `BitwiseAnd` | `&` | Bitwise |
| `BitwiseOr` | `\|` | Bitwise |
| `BitwiseXor` | `^` | Bitwise |
| `BitwiseNot` | `~` | Bitwise |
| `LeftShift` | `<<` | Bitwise shift |
| `RightShift` | `>>` | Bitwise shift |
| `Increment` | `++` | Unary |
| `Decrement` | `--` | Unary |

#### Delimiters & Punctuation
| Token Type | Lexeme | Purpose |
|-----------|--------|---------|
| `LeftParen` | `(` | Grouping |
| `RightParen` | `)` | Grouping |
| `LeftBrace` | `{` | Block start |
| `RightBrace` | `}` | Block end |
| `LeftBracket` | `[` | Array index |
| `RightBracket` | `]` | Array end |
| `Comma` | `,` | Separator |
| `Semicolon` | `;` | Statement terminator |
| `Colon` | `:` | Label/case marker |
| `Dot` | `.` | Member access |

#### Literals
| Token Type | Example | Format |
|-----------|---------|--------|
| `IntLiteral` | `123`, `0`, `999` | Decimal digits |
| `FloatLiteral` | `3.14`, `0.5`, `12.0` | Digits with decimal point |
| `StringLiteral` | `"hello"`, `"line\nbreak"` | Double-quoted with escapes |
| `CharLiteral` | `'a'`, `'x'`, `'1'` | Single-quoted |

#### Special Tokens
| Token Type | Purpose |
|-----------|---------|
| `Identifier` | User-defined names (variables, functions) |
| `EOF` | End of input marker |
| `Error` | Invalid character/sequence |

---

## Lexical Rules

### Rule 1: Whitespace & Position Tracking
Whitespace (spaces, tabs, newlines) is consumed and skipped, but **position is always tracked**:
- **Newline (`\n`)**: Increments line counter, resets column to 1
- **Other whitespace**: Increments column
- **Impact**: Accurate error reporting with line/column pairs

### Rule 2: Identifier & Keyword Rules
- **Identifiers**: Start with letter (a-z, A-Z) or underscore `_`, followed by letters, digits, or underscores
- **Case-sensitive**: `x` ≠ `X`, `main` ≠ `Main`
- **Keyword priority**: Reserved words cannot be used as identifiers
- **Examples**:
  - Valid: `x`, `_private`, `myVar123`, `_`
  - Invalid: `123abc` (starts with digit), `-invalid`, `if` (reserved)

### Rule 3: Number Literal Rules
- **Integers**: One or more decimal digits `[0-9]+`
  - Examples: `0`, `123`, `999999`
- **Floats**: Digits with decimal point `[0-9]+\.[0-9]+`
  - Must have digits on both sides of `.`
  - Examples: `0.5`, `3.14`, `100.0`
  - Invalid: `.5` (no leading digits), `5.` (no trailing digits), `5` (no decimal)
- **No scientific notation**: `1e5` is invalid

### Rule 4: String & Character Rules
- **Strings**: `"..."` with content between quotes
  - **Escape sequences supported**:
    - `\n` → newline
    - `\t` → tab
    - `\\` → backslash
    - `\"` → double quote
    - `\'` → single quote
  - **Unterminated**: Error if string reaches end of line without closing `"`
  - Examples: `"hello"`, `"line1\nline2"`, `""`

- **Characters**: `'...'` with single content
  - Must contain exactly one character (or escape sequence)
  - Examples: `'a'`, `'1'`, `'\n'`, `'\\\'`
  - Invalid: `''` (empty), `'ab'` (multiple chars)

### Rule 5: Comment Rules
- **Single-line comments**: `//` to end of line
  - Everything after `//` until newline is skipped
  - Examples:
    ```c
    int x = 10;  // Assignment
    // Entire line is comment
    float y = 3.14;
    ```
- **No multi-line comments**: `/* ... */` not supported

### Rule 6: Operator Precedence (Lexical)
Multi-character operator recognition uses greedy matching:
- `=` followed by `=` → `==` (not `=` then unmatched `=`)
- `+` followed by `+` → `++` (not `+` then unmatched `+`)
- `<` followed by `<` → `<<` (not `<` then unmatched `<`)

When lookahead doesn't complete an operator, lexer backtracks:
- `+ ;` → `+` then `;` (not `+` plus incomplete operator)
- `= (` → `=` then `(` (not `=` plus incomplete operator)

---

## Error Handling

### Lexical Error Type: `TokenType::Error`

The lexer detects one primary error category:

#### Invalid Character
**Trigger**: Character that doesn't belong to any valid token category
**Examples**:
```c
int x = 10 @ y;      // @ is invalid
float z = #value;    // # is invalid
bool b = `test`;     // ` is invalid
```

**Error Representation**:
```rust
Token {
    token_type: TokenType::Error,
    value: "@".to_string(),      // The problematic character
    line: 1,
    column: 10,
}
```

### Error Reporting Flow
1. **Detection**: Lexer encounters invalid character
2. **Token creation**: Creates `TokenType::Error` token with position
3. **Continuation**: Lexer skips the invalid character and continues
4. **Accumulation**: All tokens (including errors) added to stream
5. **Compiler handling**: Parser/main compiler checks for error tokens and reports them

---

## Lexical Analysis Process

### Algorithm Overview
```
initialize position = 0, line = 1, column = 1
initialize tokens = []

while position < source.length:
    char = source[position]
    
    if is_whitespace(char):
        skip_whitespace()
        
    else if is_letter_or_underscore(char):
        lexeme = consume_identifier()
        if lexeme is keyword:
            create Token(Keyword, lexeme)
        else:
            create Token(Identifier, lexeme)
            
    else if is_digit(char):
        if is_float():
            create Token(FloatLiteral, consume_float())
        else:
            create Token(IntLiteral, consume_integer())
            
    else if char == '"':
        create Token(StringLiteral, consume_string())
        
    else if char == ''':
        create Token(CharLiteral, consume_char())
        
    else if char == '/' and next == '/':
        skip_line_comment()
        
    else if is_operator_start(char):
        if can_extend_to_operator():
            create Token(MultiCharOp, consume_operator())
        else:
            create Token(SingleCharOp, char)
            
    else:
        create Token(Error, char)
        
    position++
    column++

create Token(EOF, "")
return tokens
```

### Position Tracking Detail

**Example**: Lexing `int x = 10;`

```
Position 0: 'i'  → line: 1, col: 1
Position 1: 'n'  → line: 1, col: 2
Position 2: 't'  → line: 1, col: 3
Position 3: ' '  → line: 1, col: 4 (whitespace, skip)
Position 4: 'x'  → line: 1, col: 5
Position 5: ' '  → line: 1, col: 6 (whitespace, skip)
Position 6: '='  → line: 1, col: 7
Position 7: ' '  → line: 1, col: 8 (whitespace, skip)
Position 8: '1'  → line: 1, col: 9
Position 9: '0'  → line: 1, col: 10
Position 10: ';' → line: 1, col: 11

Token Stream:
1. Token(Keyword, "int", line: 1, col: 1)
2. Token(Identifier, "x", line: 1, col: 5)
3. Token(Assign, "=", line: 1, col: 7)
4. Token(IntLiteral, "10", line: 1, col: 9)
5. Token(Semicolon, ";", line: 1, col: 11)
6. Token(EOF, "", line: 1, col: 12)
```

**Example with newline**: 
```
int x = 10;
float y = 3.14;

Position 10: '\n' → line: 2, col: 1 (newline resets column)
Position 11: 'f'  → line: 2, col: 1
Position 12: 'l'  → line: 2, col: 2
...
Position 18: 'y'  → line: 2, col: 8
```

---

## Examples

### Example 1: Simple Variable Declaration

**Source Code**:
```c
int count = 42;
```

**Tokenization Process**:
1. Read `i`, `n`, `t` → match keyword → emit `Keyword::Int`
2. Skip space
3. Read `c`, `o`, `u`, `n`, `t` → no keyword match → emit `Identifier("count")`
4. Skip space
5. Read `=` → emit `Assign`
6. Skip space
7. Read `4`, `2` → emit `IntLiteral("42")`
8. Read `;` → emit `Semicolon`

**Token Stream**:
```
Token(Keyword::Int, "int", line: 1, col: 1)
Token(Identifier, "count", line: 1, col: 5)
Token(Assign, "=", line: 1, col: 11)
Token(IntLiteral, "42", line: 1, col: 13)
Token(Semicolon, ";", line: 1, col: 15)
Token(EOF, "", line: 1, col: 16)
```

### Example 2: Multi-Character Operators

**Source Code**:
```c
if (x >= 10 && y <= 20) { }
```

**Operator Recognition**:
- `>` + `=` → `>=` (lookahead succeeds)
- `&` + `&` → `&&` (lookahead succeeds)
- `<` + `=` → `<=` (lookahead succeeds)

**Relevant Tokens**:
```
Token(Keyword::If, "if", ...)
Token(LeftParen, "(", ...)
Token(Identifier, "x", ...)
Token(GreaterEqual, ">=", ...)
Token(IntLiteral, "10", ...)
Token(And, "&&", ...)
Token(Identifier, "y", ...)
Token(LessEqual, "<=", ...)
Token(IntLiteral, "20", ...)
Token(RightParen, ")", ...)
Token(LeftBrace, "{", ...)
Token(RightBrace, "}", ...)
Token(EOF, "", ...)
```

### Example 3: String with Escape Sequences

**Source Code**:
```c
print("Hello\nWorld!");
```

**String Processing**:
1. Encounter `"`
2. Read until closing `"`
3. Handle escape: `\n` → newline character
4. Build token value: `"Hello\nWorld!"`

**Token Stream**:
```
Token(Keyword::Print, "print", line: 1, col: 1)
Token(LeftParen, "(", line: 1, col: 6)
Token(StringLiteral, "Hello\nWorld!", line: 1, col: 7)
Token(RightParen, ")", line: 1, col: 21)
Token(Semicolon, ";", line: 1, col: 22)
Token(EOF, "", ...)
```

### Example 4: Comments

**Source Code**:
```c
int x = 10;  // Initialize x to 10
// This entire line is a comment
float y = 3.14;  // Floating point value
```

**Comment Handling**:
- `// Initialize...` → completely skipped
- `// This entire...` → completely skipped
- `// Floating point...` → completely skipped
- Position tracking continues normally

**Token Stream** (comments omitted):
```
Token(Keyword::Int, "int", line: 1, col: 1)
Token(Identifier, "x", line: 1, col: 5)
Token(Assign, "=", line: 1, col: 7)
Token(IntLiteral, "10", line: 1, col: 9)
Token(Semicolon, ";", line: 1, col: 11)
Token(Keyword::Float, "float", line: 3, col: 1)
Token(Identifier, "y", line: 3, col: 7)
Token(Assign, "=", line: 3, col: 9)
Token(FloatLiteral, "3.14", line: 3, col: 11)
Token(Semicolon, ";", line: 3, col: 15)
Token(EOF, "", line: 3, col: 16)
```

### Example 5: Error Detection

**Source Code**:
```c
int x = 10 @ y;
```

**Lexing Process**:
1. Successfully lex: `int`, `x`, `=`, `10`
2. Skip space after `10`
3. Encounter `@` → not recognized → emit `Error` token
4. Continue from `y`

**Token Stream**:
```
Token(Keyword::Int, "int", line: 1, col: 1)
Token(Identifier, "x", line: 1, col: 5)
Token(Assign, "=", line: 1, col: 7)
Token(IntLiteral, "10", line: 1, col: 9)
Token(Error, "@", line: 1, col: 12)
Token(Identifier, "y", line: 1, col: 14)
Token(Semicolon, ";", line: 1, col: 15)
Token(EOF, "", ...)
```

**Compiler Response**:
- Main compiler detects `Error` token
- Reports: `Lexical Error at line 1, column 12: Invalid character '@'`
- Halts further processing

---

## Implementation Details

### State Variables Maintained

```rust
pub struct Lexer {
    source: String,              // Full source code
    position: usize,             // Current position in source
    current_char: Option<char>,  // Character at position
    line: usize,                 // Current line (1-indexed)
    column: usize,               // Current column (1-indexed)
}
```

### Key Methods

#### `pub fn tokenize(&mut self) -> Vec<Token>`
Main entry point that repeatedly calls `next_token()` until EOF.

#### `fn next_token(&mut self) -> Token`
- Returns next token from current position
- Advances position appropriately
- Updates line/column tracking

#### `fn advance(&mut self)`
- Moves position to next character
- Updates `current_char`
- Increments line/column counters

#### `fn peek(&self) -> Option<char>`
- Looks at next character without advancing
- Used for multi-character operator lookahead

#### `fn consume_identifier(&mut self) -> String`
- Reads alphanumeric/underscore sequence
- Advances position
- Returns collected identifier

#### `fn consume_number(&mut self) -> (String, bool)`
- Reads digits with optional decimal point
- Returns (number_string, is_float)

#### `fn consume_string(&mut self) -> String`
- Reads characters until closing `"`
- Handles escape sequences
- Returns processed string

#### `fn is_keyword(word: &str) -> bool`
- Checks if string is reserved keyword
- Uses static keyword set

---

## Integration with Parser

The lexer output feeds directly into the parser:

```
Lexer::tokenize() → Vec<Token>
                        ↓
                   Parser::new(tokens)
                        ↓
                   Parser::parse_program()
                        ↓
                   (Parsing begins)
```

**Key Contract**:
1. Last token must always be `TokenType::EOF`
2. Errors stop at lexer (reported with line/column)
3. Each token has accurate position information
4. No modifications to tokens after generation

---

## Summary

The Lexer phase is critical for:
- ✅ Converting raw text to structured tokens
- ✅ Precise position tracking for error reporting
- ✅ Handling keywords, operators, and literals
- ✅ Supporting escape sequences and comments
- ✅ Detecting lexical errors early

With a robust lexer in place, the parser can confidently assume:
- Valid token types
- Accurate position information
- No lexical ambiguities

This foundational phase enables all subsequent compiler phases to function reliably.
