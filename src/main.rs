mod lib;
mod linear;
mod quadratic;

use crate::lib::*;
use crate::linear::*;
use crate::quadratic::*;

fn main() {
    let testQuadratic:QuadraticStandardForm = QuadraticStandardForm {
        a: 1.0,
        b: 0.0,
        c: 0.0,
    };
    let testLinear:LinearSlopeInterceptForm = LinearSlopeInterceptForm {
        m: 2.0,
        b: 4.0,
    };
    let solved_eq:Vec<Point> = findIntersectionQuadraticSF(&testQuadratic, &linearToQD(&testLinear));
    solved_eq[0].logSelf();
    solved_eq[1].logSelf();
}
