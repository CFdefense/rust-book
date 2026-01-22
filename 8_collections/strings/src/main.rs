fn main() {
    // lets make a new empty string
    let mut s = String::new();

    // or if we want to start with some data
    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();

    // dont forget String::from
    let s = String::from("initial contents");

    // strings are stored as UTF-8 so these are all valid
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // we can grow a string with many techniques

    // if we want to append a slice
    let mut s = String::from("foo");
    s.push_str("bar");

    // can also append a char
    let mut s = String::from("lo");
    s.push('l');

    // we can concat
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // for concatting more complex strs use format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    // this is an invalid way to index a str
    // rust does this to avoid complexities with index UTF-8
    // each char in UTF-8 is 2 bytes therefore it is content defined
    //    let s1 = String::from("hi");
    //    let h = s1[0];

    // rust forces you to use a range if you are indexing
    let hello = "Здравствуйте";

    // despite being 0..4 this in in UTF-8 returns 'Зд'
    let s = &hello[0..4];

    // if we want to index we must be explicit
    // do we want chars or bytes?
    // this will allow us to index
    for c in s.chars() {
        println!("{c}");
    }

    // if we want bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
