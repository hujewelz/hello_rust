pub fn run() {
  let mut str1 = "hello".to_string();
  let str2 = &str1; // borrowed as inmutable
  println!("str1: {}, str2: {}: ", str1, str2);
  // update(&mut str1); // borrowed as mutable, it's ok.

  //         +----------+    +-+
  // str1 -> |  pointer | -> |h| <- +
  //         |  length  |    |e|    | slice
  //         | capacity |    |l| <- +
  //         +----------+    |l|
  //                         |o|
  //                         +-+
  str1.clear();
  let slice = &str1[..3]; // panic: index out of bounds
  println!("slice of str1 from 0 to 10: {}", slice);
}

fn update(str: &mut String) {
  str.push_str("new haha");
  let len = str.len();
  println!("the length is: {}", len);
}
