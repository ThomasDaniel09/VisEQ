mod types;
use std::collections::btree_set::Intersection;

use crate::types::*;
fn main() {
    println!("Hello, world!");
    let testeq:LinearSlopeInterceptForm = LinearSlopeInterceptForm {
        m: 4.0,
        b: 4.0,
    };
    let testeq2:LinearSlopeInterceptForm = LinearSlopeInterceptForm {
        m: 2.0,
        b: 3.0,
    };
    let points:Vec<Point> = testeq.createTable(1.0, vec![-5.0, 5.0]);
    printVecOfPoints(&points);
    Point{
        x: 0.0,
        y: 0.2,
    }.logSelf();
    let Intersection:Point = findIntersection(&testeq, &testeq2);
    Intersection.logSelf();
}
