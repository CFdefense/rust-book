## Chapter 3 – Variables and Mutability

### Immutability and `mut`
- Variables are **immutable by default**.
- Use `mut` to make a binding mutable.

```rs
let x = 5;
// x = 6; // error: cannot assign twice to immutable variable

let mut y = 5;
y = 6; // ok
```

### Constants
- Declared with `const`, must have a **type annotation** and a **compile-time value**.
- Can be declared in any scope (including global).

```rs
const MAX_POINTS: u32 = 100_000;
```

### Shadowing
- You can **shadow** a variable by redeclaring it with `let`.
- Shadowing lets you reuse the name with a new type or value.

```rs
let spaces = "   ";
let spaces = spaces.len();
```

### Extra Notes (from chapter summary)
- Variables are IMMUTABLE by default but can be made mutable.
- Constants CANNOT be mutable and are declared using `const`.
- Constants can be declared in any scope including global.
- Constants must be set only to a constant expression, not to anything that must be calculated during runtime.
- Variables can be shadowed, meaning the same variable name can be reused and will shadow existing ones depending on scope.


