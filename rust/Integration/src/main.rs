fn trapezoid<F>(f:F, start:f64, end:f64, n:i32) -> f64
where F:Fn(f64)->f64 {
    let h = (end - start) / n as f64;
    ((1..n).map(|i| f(start+h*i as f64))
         .fold(0.5*(f(start)+f(end)), |sum:f64, e| sum + e)) * h
}

fn simpson<F>(f:F, start:f64, end:f64, n:i32) -> f64
where F:Fn(f64)->f64 {
    let m = n/2;
    let h = (end - start) / n as f64;
    let sum_part1 = (0..m).map(|i| f(start+h*((2*i+1) as f64)))
                        .fold(0.0, |sum:f64, e| sum + e);
    let sum_part2 = (1..m).map(|i| f(start+h*((2*i) as f64)))
                        .fold(0.0, |sum:f64, e| sum + e);
    h/3.0 * (f(start) + 4.0*sum_part1 + 2.0*sum_part2 + f(end))
}

fn f(x:f64) -> f64{
    x.sin()
}

fn estimates(x:f64, y:f64, k:f64) -> f64{
    (x/y).abs().log10() / k.log10()
}

fn main() {
    let accurate =  0.2566401204049134529342974;
    let mut trapezoid_deviation = Vec::new();
    let mut simpson_deviation = Vec::new();
    for i in (0..13){
        trapezoid_deviation.push(trapezoid(f,1.0,5.0,(2 as i32).pow(i))-accurate);
        simpson_deviation.push(simpson(f,1.0,5.0,(2 as i32).pow(i))-accurate);
    }

    println!("N={:4}, {:16.12e}", 1, trapezoid_deviation[0]);
    for i in (1..13){
        println!("N={:4}, {:16.12e}, {:16.12e}", (2 as i32).pow(i), trapezoid_deviation[i as usize], estimates(trapezoid_deviation[i as usize],
                                                                            trapezoid_deviation[i as usize -1], 0.5));
    }
    println!("N={:4}, {:16.12e}", 1, simpson_deviation[0]);
    for i in (1..13){
        println!("N={:4}, {:16.12e}, {:16.12e}", (2 as i32).pow(i), simpson_deviation[i as usize], estimates(simpson_deviation[i as usize],
                                                                            simpson_deviation[i as usize -1], 0.5));
    }
}
