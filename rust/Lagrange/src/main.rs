extern crate rand;

use std::f64::consts;
use rand::distributions::{IndependentSample, Range};

enum XType {
    One,
    Two,
    Random,
}

fn f(x:f64) -> f64 {
    1.0 / (1.0 + x*x)
}

fn y(x:i32) -> f64 {
    -5.0 + 10.0/500.0 * x as f64
}

fn lagrange(x:f64, n:i32, xtype:&XType)  -> f64{
    let xlist = gen_xlist(n,xtype);
    xlist.iter()
        .map(|xi| *xi)
        .fold(0.0, |ln:f64, xi| ln +
            xlist.iter()
                .map(|xj| *xj)
                .filter(|&xj| xj != xi)
                .fold(1.0, |product:f64, xj| product * (x-xj)/(xi-xj))
            * f(xi)
        )
}

fn gen_xlist(n:i32,xtype:&XType) ->Vec<f64> {
    match *xtype {
        XType::One => (0..n+1).map(|i| -5.0+10.0 / n as f64 * i as f64).collect::<Vec<f64>>(),
        XType::Two => (0..n+1).map(|i| -5.0 * ((2.0 * i as f64 + 1.0)/(2.0 * n as f64 +2.0) * consts::PI).cos()).collect::<Vec<f64>>(),
        XType::Random => {
            let between = Range::new(-5f64,5f64);
            let mut rng = rand::thread_rng();
            (0..n+1).map(|_| between.ind_sample(&mut rng)).collect::<Vec<f64>>()
        },
    }
}

fn max_error(n:i32, xtype:&XType) -> f64{
    (0..501).map(|x| y(x)).map(|yi| (lagrange(yi,n,xtype) - f(yi)).abs())
            .fold(0.0,|max:f64,error| max.max(error))
}

fn main() {
    println!("第1组节点误差：");
    for i in [5,10,20,40].iter() {
        println!("N={} , {:16.12e}",i,max_error(*i,&XType::One));
    }
    println!("第2组节点误差：");
    for i in [5,10,20,40].iter() {
        println!("N={} , {:16.12e}",i,max_error(*i,&XType::Two));
    }
    println!("随机节点误差：");
    for i in [5,10,20,40].iter() {
        println!("N={} , {:16.12e}",i,max_error(*i,&XType::Random));
    }
}
