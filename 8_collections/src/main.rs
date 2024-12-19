use std::collections::HashMap;

fn main() {
  println!("Vetors:");
  vectors();
  println!("\nStrings:");
  strings();
  println!("\nMaps:");
  maps();
}

fn vectors() {
  let v: Vec<i32> = Vec::new();
  let v = vec![1, 2, 3];

  let mut v = Vec::new();

  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);

  let v = vec![1, 2, 3, 4, 5];

  let third: &i32 = &v[2];
  println!("The third element is {third}");

  let third: Option<&i32> = v.get(2);
  match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
  }

  let v = vec![100, 32, 57];
  for n_ref in &v {
    // n_ref has type &i32
    let n_plus_one: i32 = *n_ref + 1;
    println!("{n_plus_one}");
  }

  let mut v = vec![100, 32, 57];
  for n_ref in &mut v {
    // n_ref has type &mut i32
    *n_ref += 50;
  }

  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];
}

fn strings() {
  let data = "initial contents";

  let s = data.to_string();

  // the method also works on a literal directly:
  let s = "initial contents".to_string();

  let s = String::from("initial contents");

  let hello = String::from("السلام عليكم");
  let hello = String::from("Dobrý den");
  let hello = String::from("Hello");
  let hello = String::from("שלום");
  let hello = String::from("नमस्ते");
  let hello = String::from("こんにちは");
  let hello = String::from("안녕하세요");
  let hello = String::from("你好");
  let hello = String::from("Olá");
  let hello = String::from("Здравствуйте");
  let hello = String::from("Hola");

  let mut s = String::from("foo");
  s.push_str("bar");

  let mut s = String::from("lo");
  s.push('l');

  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

  let hello = "Здравствуйте";
  let s = &hello[0..4];
  println!("{s}");

  for c in "Зд".chars() {
    println!("{c}");
  }

  for b in "Зд".bytes() {
    println!("{b}");
  }
}

fn maps() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let team_name = String::from("Blue");
  let score = scores.get(&team_name).copied().unwrap_or(0);

  for (key, value) in &scores {
    println!("{key}: {value}");
  }

  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);

  println!("{scores:?}");

  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);

  println!("{scores:?}");

  let text = "hello world wonderful world";
  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{map:?}");
}
