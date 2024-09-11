mod types;

use crate::types::*;
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
        a: 1.0,
        b: -4.0,
        c: 8.0,
    };
    testQuadratic.logSelf();
    println!("{}", testQuadratic.evaluate(0.0));
    println!("{}", testQuadratic.numberOfRealRoots());
}
