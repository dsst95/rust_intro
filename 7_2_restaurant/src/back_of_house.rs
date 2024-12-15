mod appetizer;
mod breakfast;

pub use appetizer::Appetizer;
pub use breakfast::Breakfast;

fn deliver_order() {}

fn fix_incorrect_order() {
  cook_order();
  deliver_order();
}

fn cook_order() {}
