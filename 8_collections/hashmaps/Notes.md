## Chapter 8 – Hash Maps

### Basics
- `HashMap<K, V>` stores key–value pairs using a hashing function.

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

### Ownership and Access
- Types that don’t implement `Copy` move into the map on `insert`.

```rs
let team = String::from("Blue");
let score = scores.get(&team);
```

### Extra Notes (from chapter summary)
- The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a hashing function.
- Like vectors, hash maps store their memory on the heap because they are dynamic data structures.
- Types which implement the `Copy` trait will copy their value to the hash map on insert, but more complex data like `String` will move ownership to the hash map.
- By default, `HashMap` uses a hashing function called SipHash that can provide resistance to certain DoS attacks.
- You can optionally decide to use a different hasher if you would like; some have speed/security tradeoffs.

