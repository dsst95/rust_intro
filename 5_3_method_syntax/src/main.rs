#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }

  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  fn set_width(&mut self, width: u32) {
    self.width = width;
  }

  fn max(self, other: Rectangle) -> Rectangle {
    Rectangle {
      width: self.width.max(other.width),
      height: self.height.max(other.height),
    }
  }
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );

  let rect2 = Rectangle::square(20);
  let rect3 = Rectangle {
    width: 60,
    height: 45,
  };

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

  let mut r = Rectangle {
    width: 1,
    height: 2,
  };
  let area1 = r.area();
  let area2 = Rectangle::area(&r);
  assert_eq!(area1, area2);

  r.set_width(2);
  Rectangle::set_width(&mut r, 2);

  let rect = Rectangle {
    width: 0,
    height: 0,
  };
  println!("{}", rect.area());

  let other_rect = Rectangle {
    width: 1,
    height: 1,
  };
  let max_rect = rect.max(other_rect);
}
