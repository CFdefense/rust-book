fn main() {
    // creating a new vector
    // Vect<T> will hold i32 type
    let v: Vec<i32> = Vec::new();

    // we can also init like this
    // Vec<T> is inferred as i32 because thats default int type
    // lets also make it mutable because well add some values
    let mut v = vec![1, 2, 3];

    // lets add some values to the vector
    v.push(4);
    v.push(5);
    v.push(6);

    // lets get some of the values we inserted

    // get a reference to the third index
    let third: &i32 = &v[2];

    // lets attempt to get the 3rd element directly
    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("Found the value {third} at the third index"),
        None => println!("No third index"),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    // have an immutable reference here
    let first = &v[0];

    // cannot reference this as mutable
    // v.push(6);

    println!("The first element is: {first}");

    // lets iterate over our vector now
    // we need to dereference this value to update it
    for i in &mut v {
        *i += 50;
    }

    // but what if we want to store multiple types in our vector?
    // we can use enums whos variants have differing types and use the enum as the type of the vector

    let v: Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Int(3), 
        SpreadSheetCell::Float(2.5), 
        SpreadSheetCell::Text(String::from("Heya"))
    ];

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

}   

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
