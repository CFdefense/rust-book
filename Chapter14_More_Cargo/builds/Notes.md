## Chapter 14 – More About Cargo and Crates.io

### Customizing Builds with Release Profiles

In Rust, release profiles are predefined, customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code. Each profile is configured independently of the others.

Two main profiles
- dev (cargo build)
- release (cargo build --release)

Cargo has default settings for each of the profiles that apply when you haven’t explicitly added any [profile.*] sections in the project’s Cargo.toml file. 

For example, here are the default values for the opt-level setting for the dev and release profiles in Cargo.toml:

```rs
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

The opt-level setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3. 

**More Optimizations = Longer Compilation Time**