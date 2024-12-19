use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
  // panic!("crash and burn");

  // let v = vec![1, 2, 3];

  // v[99];

  let greeting_file_result = File::open("hello.txt");

  // let greeting_file = match greeting_file_result {
  //     Ok(file) => file,
  //     Err(error) => panic!("Problem opening the file: {error:?}"),
  // };

  // let greeting_file = File::open("hello.txt")
  //       .expect("hello.txt should be included in this project");

  // let greeting_file = read_username_from_file();
  // let greeting_file = read_username_from_file_alternative();

  let greeting_file = File::open("hello.txt")?;

  Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
  let mut username = String::new();

  File::open("hello.txt")?.read_to_string(&mut username)?;

  Ok(username)
}

fn read_username_from_file_alternative() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}
