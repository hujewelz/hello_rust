// mod string;
mod struct_exp;

fn main() {
    // string::run();
    let user = struct_exp::User::new("Jewelz".to_string(), 23);
    user.info();
}
