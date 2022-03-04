pub struct User {
  name: String,  // field, default is private
  age: u8,
}

// functions defined whithin an `impl` are called associated functions
impl User {
  pub fn new(name: String, age: u8) -> User {
    User { name, age }
  }
  
  pub fn info(&self) {
    print!("My name is {} and {} years old.", self.name, self.age);
  }
}
