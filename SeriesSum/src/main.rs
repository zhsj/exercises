#![feature(core)]
use std::f64::consts;

fn series(x:f64, k:i32) -> f64 {
    (1..k).fold(0.0,|sum:f64, i| sum + 1.0/((i as f64)*(i as f64 + x)))
}

fn main() {
    let xs:Vec<f64> = vec![0.0, 0.5, 1.0, consts::SQRT2, 10.0, 100.0, 300.0];
    for x in xs {
        println!("{:6.2} , {:16.12e}",x,series(x,2e6 as i32));
    }
}
