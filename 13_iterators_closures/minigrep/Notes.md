## Chapter 13 – Improving the `minigrep` CLI Program

### Reading Arguments
- The `minigrep` program reads command-line arguments to get the query and file name. With our new found knowledge of Iterators well look to improve this program.

### Improvement 1: Removing costly clones

```rs
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```
The above build function takes in as arguments slices of Strings, as such we must clone those arguments so Config can have its own. Instead lets change the build function to take ownership of an iterator as an argument instead of a slice.

#### Change main

First change how we create the args so we use an iterator in main:
```diff
fn main() {
-   let args: Vec<String> = env::args().collect();

-   let config = Config::build(&args).unwrap_or_else(|err| {
+   let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--
```
The env::args function returns an iterator! So lets skip over collecting it and go straight to passing it to the build function.

#### Change build() signature

Next lets change the build() function signature to accept a iterator
```diff
impl Config {
    fn build(
-       args: &[String] 
+       mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```
The standard library documentation for the env::args function shows that the type of the iterator it returns is std::env::Args, and that type implements the Iterator trait and returns String values.

By using this impl syntax we say that the function can accept any type that implement Iterator that return Item type String.

Remember we must always define type Item.

#### Change build() body

Lets now use the iterators next() method to parse our arguments.
```diff
    fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
-       if args.len() < 3 {
-           return Err("not enough arguments");
-       }
+       args.next()
+
-       let query = args[1].clone();
+       let query = match args.next() {
+           Some(arg) => arg,
+           None => return Err("Didn't get a query string)
+       }
-       let file_path = args[2].clone();
+       let file_path = match args.next() {
+           Some(arg) => arg,
+           None => return Err("Didn't get a file path)
+       }
+
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

We skip the first next() due to that being the name of the program which we do not care about. Then we call next to get the following values.

### Improvement 2: Clarifying Search With Iterator Adapters

```rs
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

We can further improve our search() method with iterators. Doing so will also help us avoid having a mutable intermediate results vector. The functional programming style prefers to minimize the amount of mutable state to make code clearer.

#### Use an Iterator for search_case_sensitive()

```diff
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
-   let mut results = Vec::new();
-
-   for line in contents.lines() {
-       if line.contains(query) {
-           results.push(line);
-       }
-   }

-   results
+   contents
+       .lines()
+       .filter(|line| line.contains(query))
+       .collect()
}
```

We can simply change this to an iterator by using the lines() method which returns an Iterator of string slices of lines. We can then filter() each line based on if it contains the query or not. Finally we collect() to consume the iterator.

#### Use an Iterator for search_case_insensitive()

```diff
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
-   let query = query.to_lowercase();
-   let mut results = Vec::new();

-   for line in contents.lines() {
-       if line.to_lowercase().contains(&query) {
-           results.push(line);
-       }
-   }

-   results
+   contents
+       .lines()
+       .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
+       .collect()
}
```

Here we do the same except for needing to convert both the line and the query to lowercase for case-insensitive comparison.