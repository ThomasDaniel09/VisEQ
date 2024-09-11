pub struct Point {
  pub x: f64,
  pub y: f64,
}
impl Point {
  pub fn logSelf(&self) -> () {
    print!("(");
    print!("{}", self.x);
    print!(", ");
    print!("{}", self.y);
    println!(")");
  }
}