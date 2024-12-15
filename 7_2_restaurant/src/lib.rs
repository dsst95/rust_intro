mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::hosting;
pub use crate::back_of_house::*;

pub fn eat_at_restaurant() {
  // Absolute path
  crate::front_of_house::hosting::add_to_waitlist();

  // Relative path
  hosting::add_to_waitlist();

  // Order a breakfast in the summer with Rye toast
  let mut meal = Breakfast::summer("Rye");
  // Change our mind about what bread we'd like
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  // The next line won't compile if we uncomment it; we're not allowed
  // to see or modify the seasonal fruit that comes with the meal
  // meal.seasonal_fruit = String::from("blueberries");

  let order1 = Appetizer::Soup;
  let order2 = Appetizer::Salad;
}
