** Packages and Crates **

- A crate is the smallest amount of code that the Rust compiler considers at a time.

- A crate can come in one of two forms: a binary crate or a library crate. 

- Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server.

- Library crates don’t have a main function, and they don’t compile to an executable. 
  Instead, they define functionality intended to be shared with multiple projects. For example, the rand crate.

- A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.

- A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.

- If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package.


** Modules **

- When compiler runs it looks at crate root either src/lib.rs for library crate or src/main.rs for binary crate.

- Within this crate root one can declare modules ** ex mod garden; **, 
  the compiler will then look for this module inline, in src/garden.rs and in src/garden/mod.rs

- In any file other than the crate root you can declare submodules ** ex mod vege in src/garden.rs **, 
  the compiler will then look for this module inline, in src/garden/vege.rs and in src/garden/vege/mod.rs

- Once a module is apart of the root crate we can refer to code in the module from anywhere in the crate using its path. 

- Code within a module is private by default you can use ** pub mod ** instead of mod to make it public

- Can use lib.rs root to better organize the mod system/heirarchy

- Super keyword is the same as doing '..' in cli it starts the relative path from the current module's parent

- When bringing modules into scope it is best practice to bring them in with their parent module such that we understand its not a locally defined fn

- However when bringing in enums, structs and other items it is best practice to omit the parent such to avoid CPU::CPU

- The above is not best practice only when bringing in two same named sub structs with differing parent names

- We can also use 'as' to change the name in which we call imported modules