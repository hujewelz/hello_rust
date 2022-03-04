pub fn run() {
  let mut str1 = "hello, world".to_string();
  let str2 = &str1; // borrowed as inmutable
  println!("str1: {}, str2: {}: ", str1, str2);
  update(&mut str1); // borrowed as mutable, it's ok.
}

fn update(str: &mut String) {
  str.push_str("new hhh");
  let len = str.len();
  println!("the length is: {}", len);
}
