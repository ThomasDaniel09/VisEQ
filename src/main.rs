mod lib;
mod linear;
mod quadratic;

use crate::lib::*;
use crate::linear::*;
use crate::quadratic::*;

fn main() {
    let testQuadratic:QuadraticStandardForm = QuadraticStandardForm { //DEBUG!
        a: 1.0,
        b: 0.0,
        c: 0.0,
    };
    let roots:Vec<Point> = testQuadratic.findRoots();
    let mut i:usize = 0;
    while i < roots.len() {
        roots[i].logSelf();
        i += 1;
    }
}
