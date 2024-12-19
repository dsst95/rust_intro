fn largest<T>(list: &[T]) -> &T
where
  T: std::cmp::PartialOrd,
{
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

struct Point<X1, Y1> {
  x: X1,
  y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
  fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
      Point {
          x: self.x,
          y: other.y,
      }
  }
}

struct PointTwo<T> {
  x: T,
  y: T,
}

impl<T> PointTwo<T> {
  fn x(&self) -> &T {
      &self.x
  }
}

impl PointTwo<f32> {
  fn distance_from_origin(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

enum Option_i32 {
  Some(i32),
  None,
}

enum Option_f64 {
  Some(f64),
  None,
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {result}");

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest(&char_list);
  println!("The largest char is {result}");

  let both_integer = Point { x: 5, y: 10 };
  let both_float = Point { x: 1.0, y: 4.0 };
  let integer_and_float = Point { x: 5, y: 4.0 };


  let p = PointTwo { x: 5.0, y: 10.0 };

  println!("p.x = {}, distance = {}", p.x(), p.distance_from_origin());

  let p1 = Point { x: 5, y: 10.4 };
  let p2 = Point { x: "Hello", y: 'c' };

  let p3 = p1.mixup(p2);

  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

  let integer = Option_i32::Some(5);
  let float = Option_f64::Some(5.0);
}
