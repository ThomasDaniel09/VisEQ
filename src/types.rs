use std::{mem::Discriminant, ptr::eq, thread::sleep};

pub struct Point {
  pub x: f64,
  pub y: f64,
}
pub struct LinearSlopeInterceptForm { // y = mx + b
  pub m: f64,
  pub b: f64,
}
pub struct QuadraticStandardForm { //y = ax^2 + bx + c
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
  pub fn logSelf(&self) {
    print!("y = ");
    print!("{}", self.m);
    print!("x + ");
    println!("{}", self.b);
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

pub fn findIntersection(eq_one: &LinearSlopeInterceptForm, eq_two:&LinearSlopeInterceptForm) -> Point {
  if eq_one.b == eq_two.b {
    return Point {
      x: 0.0,
      y: 0.0,
    }
  } else if eq_one.m != 0.0 && eq_two.b != 0.0 {
    let equalityCoeffecient:f64 = eq_one.m / eq_two.m;
    let eq_two_modified:LinearSlopeInterceptForm = LinearSlopeInterceptForm {
      m: eq_two.m * equalityCoeffecient,
      b: eq_two.b * equalityCoeffecient,
    };
    let solved_y:f64 = eq_two_modified.b - eq_one.b;
    let solved_x:f64 = eq_one.solveForXValue(solved_y).x;
    return Point{
      x: solved_x,
      y: solved_y,
    }
    } else { // One of the slopes is equal to zero
      if eq_one.m == 0.0 {
        let solved_y:f64 = eq_one.b;
        let solved_x:f64 = eq_two.solveForXValue(solved_y).x;
        return Point {
          x: solved_x,
          y: solved_y,
        } 
      } else {
        let solved_y:f64 = eq_two.b;
        let solved_x:f64 = eq_one.solveForXValue(solved_y).x;
        return Point {
          x: solved_x,
          y: solved_y,
      }
    }
  }
}

//QUADRADICS

impl QuadraticStandardForm {
  pub fn logSelf(&self) {
    print!("y = ");
    print!("{}", self.a);
    print!("x^2 + ");
    print!("{}", self.b);
    print!("x + ");
    print!("{}", self.c);
    println!("");
  }
  pub fn evaluate(&self, xval:f64) -> f64 {
    return self.a*xval*xval + self.b*xval + self.c;
  }
  pub fn numberOfRealRoots(&self) -> usize {
    let discriminant:f64 = self.b*self.b - (4.0*self.a*self.c);
    if discriminant > 0.0 {
      return 2;
    } else if discriminant == 0.0 {
      return 1;
    } else {
      return 0;
    }
  }
}

