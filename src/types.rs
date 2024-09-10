
pub struct Point {
  pub x: f64,
  pub y: f64,
}
pub struct LinearSlopeInterceptForm {
  pub m: f64,
  pub b: f64,
}
pub struct QuadradicStandardForm {
  pub a: f64,
  pub b: f64,
  pub c: f64,
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