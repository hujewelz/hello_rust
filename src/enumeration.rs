pub enum Color {
  Red,
  Green,
  Blue,
  Other(u8, u8, u8),
}

impl Color {
  pub fn value(&self) -> (u8, u8, u8) {
    match self {
      Color::Red => (255, 0, 0),
      Color::Green => (0, 255, 0),
      Color::Blue => (0, 0, 255),
      Color::Other(r, g, b) => {
        println!("custom color whit r: {}, g: {}, b: {}", r, g, b);
        (*r, *g, *b)
      }
    }
  }
}

pub fn info(color: &Color) {
  match color {
    Color::Red => {
      println!("Color is Red");
    }
    Color::Green => {
      println!("Color is Green");
    }
    Color::Blue => {
      println!("Color is Blue");
    }
    Color::Other(r, g, b) => {
      println!("custom color whit r: {}, g: {}, b: {}", r, g, b);
    }
  }
}
