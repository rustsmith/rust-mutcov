// Cases where multiple argument suggestions are mixed

struct X {}

fn two_args(_a: i32, _b: f32) {}
fn three_args(_a: i32, _b: f32, _c: &str) {}

fn main() {
  // Extra + Invalid
  two_args(1, "", X {}); //~ ERROR this function takes
  three_args(1, "", X {}, ""); //~ ERROR this function takes

  // Missing and Invalid
  three_args(1, X {}); //~ ERROR this function takes

  // Missing and Extra
  three_args(1, "", X {});
  //~^ ERROR mismatched types
  //~| ERROR mismatched types

  // Swapped and Invalid
  three_args("", X {}, 1);
  //~^ ERROR mismatched types
  //~| ERROR mismatched types
  //~| ERROR mismatched types

  // Swapped and missing
  three_args("", 1); //~ ERROR this function takes
}
