use crate::lib::*; //https://www.cuemath.com/algebra/standard-form-to-vertex-form/

pub struct QuadraticStandardForm { //y = ax^2 + bx + c
  pub a: f64,
  pub b: f64,
  pub c: f64,
}
pub struct QuadraticVertexForm { //y = a(x-b)^2 + c
  pub a: f64,
  pub b: f64,
  pub c: f64,
}

pub fn StandardToVertex(StandardForm:QuadraticStandardForm) -> QuadraticVertexForm {
  return QuadraticVertexForm {
    a: StandardForm.a,
    b: (-1.0 * StandardForm.b) / StandardForm.a / 2.0,
    c: StandardForm.c - (StandardForm.b / StandardForm.a / 2.0) * (StandardForm.b / StandardForm.a / 2.0),
  }
}

impl QuadraticVertexForm {
  pub fn logSelf(&self) {
    print!("y = ");
    print!("{}", self.a);
    print!("(x - ");
    print!("{}", self.b);
    print!(")^2 + ");
    print!("{}", self.c);
    println!("");
  }
  pub fn evaluate(&self, val:f64) -> f64 {
    return self.a * (val - self.b)*(val - self.b) + self.c;
  }
}

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
  /*pub fn findVertex(&self) -> Point {
    
  }*/
}

