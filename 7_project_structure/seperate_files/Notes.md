## Chapter 7 – Separate Files for Modules

### Splitting Modules Across Files
- You can move module definitions into their own files to keep the crate root small.

```rs
mod front_of_house; // uses front_of_house.rs
```

- Inside `front_of_house.rs` you can further split:

```rs
pub mod hosting; // uses front_of_house/hosting.rs
```


