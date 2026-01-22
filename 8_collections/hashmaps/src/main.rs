use std::collections::HashMap;
fn main() {
    // lets make a new hashmap and insert some values
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // now lets get some values from the hashmap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // we can also easily iterate
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // old values will be overwritten on insert
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    // or more useful we can add only if the key doesnt exist in the hashmap
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    // get and update old values
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
