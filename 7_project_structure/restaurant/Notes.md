## Chapter 7 – Restaurant Module Layout

### Front of House / Back of House
- `front_of_house` and `back_of_house` are separate modules organizing responsibilities.
- `pub mod hosting` exports functions like `add_to_waitlist`.

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

### Paths and `use`
- You can call functions with absolute or relative paths, or bring them into scope with `use`.

```rs
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```


