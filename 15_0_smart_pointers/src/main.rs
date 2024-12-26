use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

fn hello(name: &str) {
  println!("Hello, {name}!");
}

struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

#[derive(Debug)]
enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil,
}

fn main() {
  let x = 5;
  let y = MyBox::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);

  let m = MyBox::new(String::from("Rust"));
  hello(&m);

  let c = CustomSmartPointer {
    data: String::from("my stuff"),
  };
  let d = CustomSmartPointer {
    data: String::from("other stuff"),
  };
  println!("CustomSmartPointers created.");

  let c = CustomSmartPointer {
    data: String::from("some data"),
  };
  println!("CustomSmartPointer created.");
  drop(c);
  println!("CustomSmartPointer dropped before the end of main.");

  let value = Rc::new(RefCell::new(5));

  let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

  let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
  let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

  *value.borrow_mut() += 10;

  println!("a after = {a:?}");
  println!("b after = {b:?}");
  println!("c after = {c:?}");
}
