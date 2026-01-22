## Chapter 3 – Data Types

### Scalar Types
- **Integers**: `i8`–`i128`, `u8`–`u128`, plus `isize`/`usize` for pointer-sized integers.
- **Floats**: `f32` and `f64` (default). Floats are always signed.
- **Booleans and chars**: `bool` (`true`/`false`) and `char` (Unicode scalar values).

```rs
let x: i32 = 42;
let price: f64 = 19.99;
let is_active: bool = true;
let heart: char = '❤';
```

### Integer Literals
- You can use **decimal**, **hex**, **octal**, **binary**, and **byte** literals.

```rs
let decimal = 98_222;
let hex = 0xff;
let octal = 0o77;
let binary = 0b1111_0000;
let byte = b'A'; // u8 only
```

### Compound Types – Tuples and Arrays
- **Tuples**: fixed-size, can group different types.
- **Arrays**: fixed-size, all elements same type, stored on the stack.

```rs
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;

let a = [1, 2, 3, 4, 5];
let first = a[0];
let len = a.len();
```

### Extra Notes (from chapter summary)
- Ints can be signed or unsigned from `i/u8` to `i/u128`. In addition the types `isize` and `usize` are architecture dependent.
- Ints can be defined as:
  - Decimal: `98_222` = `98222`
  - Hex: `0xff`
  - Octal: `0o77`
  - Binary: `0b1111_0000`
  - Byte (`u8` only): `b'A'`
- Rust floating point values can be `f32` or `f64` (default).
- Floating points are always signed.
- Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
- Arrays allocate data to the stack.


