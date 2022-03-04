// mod string;
// mod struct_exp;
mod enumeration;

fn main() {
    // string::run();
    // let user = struct_exp::User::new("Jewelz".to_string(), 23);
    // user.info();

    let blue = enumeration::Color::Blue;
    enumeration::info(&blue);
    println!("Value is: {:?}", blue.value());

    let custom_color = enumeration::Color::Other(2, 4, 5);
    println!("Value is: {:?}", custom_color.value());
}
