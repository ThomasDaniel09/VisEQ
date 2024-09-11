mod lib;
mod linear;
mod quadratic;

use crate::lib::*;
use crate::linear::*;
use crate::quadratic::*;

fn main() {
    let testeq:LinearSlopeInterceptForm = LinearSlopeInterceptForm {
        m: 2.0,
        b: 4.0,
    };
    let testeq2:LinearSlopeInterceptForm = LinearSlopeInterceptForm {
        m: 2.0,
        b: 3.0,
    };
    let testQuadratic:QuadraticStandardForm = QuadraticStandardForm {
        a: -3.0,
        b: -6.0,
        c: -9.0,
    };
    let testQuadraticVertex:QuadraticVertexForm = QuadraticVertexForm {
        a: -2.0,
        b: 2.0,
        c: 12.0,
    };
    StandardToVertex(testQuadratic).logSelf();
}
