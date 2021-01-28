mod front_of_house;

mod back_of_house {
  pub struct Breakfast {
    pub toast: String,
    season_fruit: String,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        season_fruit: String::from("peaches"),
      }
    }
  }
}

use front_of_house::hosting;

pub fn eat_at_restaurant() {
  // Relative path
  hosting::add_to_waitlist();

  let mut meal = back_of_house::Breakfast::summer("Rye");

  meal.toast = String::from("Wheat");

  println!("I'd like {} toast, please", meal.toast);
  
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn fmtResultFunction(result: Result) {
  
}

fn ioResultFunction(result: IoResult<()>) {

}