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

pub fn StandardToVertex(StandardForm:&QuadraticStandardForm) -> QuadraticVertexForm {
  let b:f64 = (-1.0 * StandardForm.b) / StandardForm.a / 2.0;
  return QuadraticVertexForm {
    a: StandardForm.a,
    b: b,
    c: StandardForm.a*b*b + StandardForm.b*b + StandardForm.c,
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
  pub fn findVertex(&self) -> Point {
    return Point {
      x: self.b,
      y: self.c,
    };
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
  pub fn findVertex(&self) -> Point {
    return StandardToVertex(self).findVertex();  
  }
  pub fn findRoots(&self) -> Vec<Point> {
    if self.a == 0.0 { panic!(); } else {
      let under_root:f64 = self.b *self.b - (4.0 * self.a * self.c);
      if under_root < 0.0 {panic!();} else {
        let x1:f64 = ((-1.0 * self.b) + under_root.sqrt())/(2.0*self.a);
        let x2:f64 = ((-1.0 * self.b) - under_root.sqrt())/(2.0*self.a);
        let ans1:Point = Point {x: x1, y: self.evaluate(x1),};
        let ans2:Point = Point {x: x2, y: self.evaluate(x2),};
        return vec![ans1, ans2];
      }
    }
  }
}