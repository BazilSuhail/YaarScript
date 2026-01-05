# Type Analyzer Documentation

## Overview
The Type Analyzer is the final semantic analysis stage of the compiler. It ensures that all operations, assignments, and function calls are performed with compatible types. It leverages the symbol table built by the `ScopeAnalyzer` to perform its checks.

---

## Detected Errors (Slang-Compatible)

### 1. **ErroneousVarDecl**
- **Description**: Occurs when a variable is declared with an invalid type, specifically `khaali` (void).
- **Example**: `khaali x; // ERROR: Variable cannot be khaali`

### 2. **FnCallParamCount**
- **Description**: Number of arguments in a function call does not match the declaration.
- **Example**: 
  ```rust
  khaali func(number a, number b);
  func(10); // ERROR: Arg count mismatch
  ```

### 3. **FnCallParamType**
- **Description**: The type of an argument passed to a function does not match the parameter type.
- **Example**: `func(10, 3.14); // ERROR: Arg 2 type mismatch (Expected number, got float)`

### 4. **ErroneousReturnType**
- **Description**: Function returns a value that doesn't match its declared return type.
- **Example**: 
  ```rust
  number test() { wapsi 3.14; } // ERROR: Incorrect return type
  ```

### 5. **ExpressionTypeMismatch**
- **Description**: General type mismatch in assignments or arithmetic between incompatible types.
- **Example**: `number x = 3.14; // ERROR: Assignment type mismatch`

### 6. **ErroneousBreak**
- **Description**: `bas_kar` statement used outside of a `jabtak`, `dohrao`, or `intekhab` block.
- **Example**: `yaar { bas_kar; } // ERROR`

### 7. **NonBooleanCondStmt**
- **Description**: The condition in an `agar`, `jabtak`, or `dohrao` statement is not a `faisla` (bool).
- **Example**: `agar (10) { ... } // ERROR: Condition must be faisla`

---

## Type System Rules

### Strict Typing
The compiler enforces strict type checking. There are **no implicit conversions** between types. For example, `number` and `float` cannot be added together directly.

### Numeric Types
- Arithmetic operations (`+`, `-`, `*`, `/`, `%`, `**`) are allowed between two `number`s or two `float`s. 
- Mixed arithmetic (e.g., `number + float`) is treated as an `ExpressionTypeMismatch`.

### Boolean Logic
- Logical operators only accept `faisla`.
- Comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`) require both operands to be of the same type and return `faisla`.

### Intrinsic Functions
- `suno()`: Infers as `number`.
- `waqt()`: Infers as `number`.
- `ittifaq(min, max)`: Infers as `number`.

---

## Usage

```rust
use compiler::semantics::type_checker::TypeChecker;

// scope_analyzer.get_global_scope() provides the symbol table
let mut type_checker = TypeChecker::new(scope_analyzer.get_global_scope());
if let Err(errors) = type_checker.check(&ast) {
    for err in errors {
        eprintln!("[Type Error] {}", err.message);
    }
}
```