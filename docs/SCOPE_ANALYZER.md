# Scope Analyzer Documentation

## Overview
The scope analyzer is a semantic analysis module that validates symbol declarations and usage in the YaarScript compiler. It builds a symbol table and detects 11 different types of scope-related errors, now fully compatible with Urdu slang keywords.

---

## Detected Errors

### 1. **UndeclaredVariableAccessed**
- **Description**: Variable used but never declared anywhere.
- **Example**:
  ```rust
  number x = unknownVar + 5;  // ERROR: unknownVar not declared
  ```

### 2. **UndefinedFunctionCalled**
- **Description**: Function called but never declared/defined.
- **Example**:
  ```rust
  number result = nonExistentFunc(10);  // ERROR
  ```

### 3. **VariableRedefinition**
- **Description**: Variable declared multiple times in the same scope.
- **Example**:
  ```rust
  number x = 10;
  number x = 20;  // ERROR: redefinition
  ```

### 4. **FunctionPrototypeRedefinition**
- **Description**: Function prototype declared multiple times with identical signature.
- **Example**:
  ```rust
  khaali helper(number x);
  khaali helper(number x);  // ERROR: duplicate prototype
  ```

### 5. **ConflictingFunctionDefinition**
- **Description**: Function defined with different signature than existing declaration.
- **Example**:
  ```rust
  khaali process(number a);
  khaali process(float a) { }  // ERROR: different parameter type
  ```

### 6. **ConflictingDeclaration**
- **Description**: Name used for different symbol types.
- **Example**:
  ```rust
  qism Color { Red };
  number Color = 10;  // ERROR: Color is already an enum
  ```

### 7. **ParameterRedefinition**
- **Description**: Same parameter name used twice in function.
- **Example**:
  ```rust
  number func(number x, float x) { }  // ERROR: duplicate parameter
  ```

### 8. **InvalidForwardReference**
- **Description**: Symbol referenced before declaration (within a single block).
- **Example**:
  ```rust
  number x = laterVar;  // ERROR: forward reference
  number laterVar = 10;
  ```

### 9. **InvalidStorageClassUsage**
- **Description**: Declaration in wrong scope level (e.g., enum in non-global scope).
- **Example**:
  ```rust
  yaar {
      qism LocalEnum { A, B };  // ERROR: enums must be global
  }
  ```

---

## Scope Rules

### Variable Scoping
- Variables are scoped to the block they're declared in.
- Inner scopes can shadow outer scope variables (allowed).
- Variables must be declared before use (no forward references within blocks).

### Function Scoping
- Functions can have prototypes (forward declarations).
- Prototypes must match definitions exactly (parameter types and count).
- `yaar` (main block) symbols are local to the entry point.

### Enum Scoping
- `qism` (enums) must be declared at the global scope.
- Enum type names and variant names are both added to the symbol table.
- Enum variants are treated as global `number` constants.

---

## Usage

```rust
use compiler::semantics::scope::ScopeAnalyzer;

let mut scope_analyzer = ScopeAnalyzer::new();
if let Err(errors) = scope_analyzer.analyze(&ast) {
    for error in errors {
        eprintln!("{}", error.message);
    }
}
```
