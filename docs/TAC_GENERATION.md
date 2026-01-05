# Three-Address Code (TAC) Generation

## Overview
The **TAC Generator** translates high-level YaarScript (Urdu slang) into a flat, linear sequence of instructions. This Three-Address Code acts as a bridge between semantic analysis and low-level execution or optimization.

---

## Architectural Design

### 1. Operands
The `Operand` enum represents all possible values in the IR:
- **Temp(`usize`)**: Virtual registers (e.g., `t0`, `t1`) for intermediate results.
- **Var(`String`)**: Named variables (e.g., `x`, `count`).
- **Literals**: `Int`, `Float`, `Bool`, `Char`, and `String`.
- **Label(`String`)**: Symbolic addresses for jumps.

### 2. Instruction Set (Extended)
| Instruction | Description |
|-------------|-------------|
| `Declare` | Registers a variable with its type and qualifiers. |
| `Assign` | `dest = src`. |
| `Binary` | Operations including `Plus`, `Minus`, `Mult`, `Div`, and **`Power`**. |
| `IfTrue / IfFalse`| Conditional jumps. |
| `Call / Param` | Function calling mechanism. |
| **`Read`** | Reads input from standard input: `dest = READ`. |
| **`Time`** | Gets system time: `dest = TIME`. |
| **`Random`** | Generates random number: `dest = RANDOM(min, max)`. |
| `Print` | Console output. |

---

## Examples

### 1. Power Operator (`**`)
**Source:**
```rust
number result = 2 ** 10;
```

**Generated TAC:**
```nginx
number result
t0 = 2 Power 10
result = t0
```

### 2. Input and Randomness
**Source:**
```rust
number x = suno();
number r = ittifaq(1, x);
bolo(r);
```

**Generated TAC:**
```nginx
number x
t0 = READ
x = t0
number r
t1 = RANDOM(1, x)
r = t1
print r
```

### 3. Loop with Urdu Slang (`dohrao`)
**Source:**
```rust
dohrao (number i = 0; i < 3; i++) {
    bolo(i);
}
```

**Generated TAC:**
```nginx
number i = 0
L0:
t3 = i Lt 3
ifFalse t3 goto L1
print i
i = i Plus 1
goto L0
L1:
```

---

## Implementation Details

The generator maintains several internal states:
- **Temp counter**: Unique virtual registers.
- **Label counter**: Unique jump targets.
- **Break Stack**: Tracks the exit labels for `bas_kar` statements in `jabtak`, `dohrao`, and `intekhab`.

### Intrinsic Translation
When the generator encounters `suno()`, `waqt()`, or `ittifaq()`, it doesn't emit a standard `Call` instruction. Instead, it emits specialized `Read`, `Time`, or `Random` TAC instructions that the execution engine handles as high-performance primitives.