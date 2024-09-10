use std::ptr::eq;

pub struct Point {
  pub x: f64,
  pub y: f64,
}
pub struct LinearSlopeInterceptForm { // y = mx + b
  pub m: f64,
  pub b: f64,
}
pub struct QuadradicStandardForm { //y = ax^2 + bx + c
  pub a: f64,
  pub b: f64,
  pub c: f64,
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



impl LinearSlopeInterceptForm {
  pub fn xIntercept(&self) -> Point {
    let xValue:f64 = (self.b * -1.0) / self.m;
    return Point{
      x: xValue,
      y: 0.0,
    }
  }
  pub fn solveForYValue(&self, xValue: f64) -> Point {
    return Point { 
      x: xValue,
      y: xValue*self.m + self.b,
    }
  }
  pub fn solveForXValue(&self, yValue: f64) -> Point {
    let constants:f64 = yValue - self.b;
    let xValue:f64 = constants / self.m;
    return Point {
      x: xValue,
      y: yValue,
    }
  }
  pub fn createTable(&self, granularity:f64, range: Vec<f64>) -> Vec<Point> { //Potential infinite loop
    let mut i:f64 = range[0];
    let mut points:Vec<Point> = vec![];
    while i <= range[1] {
      points.push(Point{
        x: i,
        y: self.b + self.m * i as f64,
      });
      i += granularity;
    }
    return points;
  }
}



pub fn printVecOfPoints(points:&Vec<Point>) -> () {
let mut i:usize = 0;
println!("[");
while i < points.len() {
  print!("    ");
  points[i].logSelf();
  i += 1;
}
println!("]")
}

pub fn testForIntersection(eq_one: &LinearSlopeInterceptForm, eq_two:&LinearSlopeInterceptForm) -> bool {
if eq_one.m == eq_two.m { //They are parallel or indistinct.
  return false;
} else { //If the slopes are different, they are not parallel,
  return true; //so they will intersect
}
}

pub fn findIntersection(eq_one: &LinearSlopeInterceptForm, eq_two:&LinearSlopeInterceptForm) -> Point { // test more, not sure this really works
  if eq_one.b == eq_two.b {
    return Point {
      x: 0.0,
      y: 0.0,
    }
  } else {
    let equalityCoeffecient:f64 = eq_one.m / eq_two.m; //Potential divide by zero
    let eq_two_modified:LinearSlopeInterceptForm = LinearSlopeInterceptForm {
      m: eq_two.m *-1.0 * equalityCoeffecient,
      b: eq_two.b *-1.0 * equalityCoeffecient,
    };
    let solved_y:f64 = eq_one.b + eq_two_modified.b;
    let solved_x:f64 = eq_one.solveForXValue(solved_y).x;
    return Point{
      x: solved_x,
      y: solved_y,
    }
  }
}
