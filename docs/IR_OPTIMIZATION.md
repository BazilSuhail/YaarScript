# Intermediate Representation (IR) Optimization Engine

## Overview
The **IR Optimizer** is a sophisticated pipeline component designed to transform raw **Three-Address Code (TAC)** into a more efficient, streamlined version. By analyzing data flow and patterns, it reduces instruction count, resolves computations at compile-time, and minimizes resource usage.

---

## The Optimization Pipeline

### 1. Constant Folding
**Logic**: Performs compile-time evaluation of arithmetic expressions when all operands are numeric literals.
- **New Feature**: Now includes support for the **Power (`**`) operator**.
- **Example**:
  - *Input*: `t0 = 2 ** 10`
  - *Output*: `t0 = 1024`

### 2. Constant Propagation
**Logic**: Substitutes variables known to contain a constant value with that constant.
- **Boundary Safety**: Tracking maps are cleared at `Label` and `FuncStart` to ensure optimistic assumptions never cross complex control-flow boundaries.

### 3. Copy Propagation
**Logic**: Replaces uses of a variable with its "original" source if it was a direct copy.
- **Example**:
  ```nginx
  val = original
  t0 = val * 2
  ```
  Result: `t0 = original * 2`

### 4. Peephole Optimization
**Logic**: Sliding window analysis for local algebraic simplifications.
- `x + 0` → `x`
- `x * 0` → `0`
- `x * 1` → `x`
- `x / 1` → `x`
- **Jump Elimination**: Removes `Goto` instructions that immediately precede their target label.

### 5. Dead Code Elimination (DCE)
**Logic**: Removes instructions whose result is never consumed.
- **Mark-and-Sweep**: Collects all used variables/temps and sweeps away any assignment to a variable not in the "used" set.
- **Side-Effect Preservation**: `print`, `suno`, and function calls with potentially side-effects are preserved.

---

## Iterative Fixed-Point
The optimizer runs its passes in a loop until a **Fixed-Point** is reached (i.e., no further modifications occur). This ensures that a discovery in one pass (like constant propagation) can be exploited in the next (like constant folding), continuing until the code is fully minimized.

---

## Detailed Transformation Trace

### Source Code
```rust
yaar {
    number a = 10;
    number b = a + 5;
    number c = b * 1 + 0;
    number d = 999; // Never used
    bolo(c);
}
```

### Pre-Optimization TAC
```nginx
number a = 10
t0 = a Plus 5
number b = t0
t1 = b Multiply 1
t2 = t1 Plus 0
number c = t2
number d = 999
print c
```

### Post-Optimization TAC (Fixed-Point)
```nginx
; (After Propagating 'a', Folding '10+5', then Propagating 'b' and 'c')
t0 = 15
print 15
```

---

## Optimization Summary
| Pass | Techniques | Benefit |
| :--- | :--- | :--- |
| **Folding** | Literal Evaluation | Runtime speedup |
| **Propagation** | Substitution | Unlocks folding |
| **Peephole** | Identitiy matching | Reduced instruction count |
| **DCE** | Dependency Removal | Smaller footprint |
