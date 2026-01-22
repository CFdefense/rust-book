mod garden;

fn main() {
    println!("Hello, world!");

    let asparagus = crate::garden::vege::Asparagus::Asparagus::new(String::from("AsparagusName"));
}
