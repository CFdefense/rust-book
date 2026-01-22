pub struct Asparagus {
    name: String,
}

impl Asparagus {
    pub fn new(new_name: String) -> Self {
        Self { name: new_name }
    }
    fn getName(&self) -> &String {
        &self.name
    }
}
