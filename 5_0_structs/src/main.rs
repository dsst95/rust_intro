#[derive(Debug)]
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
struct Point { x: i32, y: i32 }

#[derive(PartialEq)]
struct AlwaysEqual;

fn main() {
  let mut user1 = build_user(
    String::from("someone@example.com"),
    String::from("someusername123"),
  );

  user1.email = String::from("anotheremail@example.com");

  let user2 = User {
    email: String::from("another@example.com"),
    ..user1
  };
  println!("{user2:?}");

  let black = Color(0, 0, 0);
  // let origin = Point(0, 0, 0);
  println!("{black:?}");

  let subject = AlwaysEqual;

  if subject == AlwaysEqual {
    println!("Always equal");
  }

  let mut p = Point { x: 0, y: 0 };
  print_point(&p);
  let x = &mut p.x;
  *x += 1;
}

fn build_user(email: String, username: String) -> User {
  User {
    active: true,
    username,
    email,
    sign_in_count: 1,
  }
}

fn print_point(p: &Point) {
  println!("{}, {}", p.x, p.y);
}
