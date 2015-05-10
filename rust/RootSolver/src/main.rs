fn newton(primitive: fn(f64)->f64, deriative: fn(f64)->f64, start:f64, epsilon:f64) -> (f64,i32) {
    let iter_limit = 100000;
    let mut x_next:f64;
    let mut x = start;
    let mut iter_count = 0;
    loop {
        x_next = x - primitive(x)/deriative(x);
        //println!("Debug {:16.12e}",x_next);
        iter_count += 1;
        if iter_count >= iter_limit || (x_next-x).abs() < epsilon {break;}
        x = x_next;
    }
    return (x_next, iter_count);
}

fn secant(primitive: fn(f64)->f64, x1:f64, x2:f64, epsilon:f64) -> (f64,i32) {
    let iter_limit = 100000;
    let mut iter_count = 0;
    let mut xk = x1;
    let mut xk_next = x2;
    while iter_count < iter_limit && (xk_next - xk).abs() > epsilon {
        let xt = xk_next - primitive(xk_next)*(xk_next-xk)/(primitive(xk_next)-primitive(xk));
        xk = xk_next;
        xk_next = xt;
        iter_count += 1;
    }
    return (xk_next, iter_count);
}

fn test(x:f64) -> f64 {
    x*x*x / 3.0 - x
}

fn test_deriative(x:f64) -> f64 {
    x*x - 1.0
}

fn main() {
    println!("Newton迭代，初值、根和迭代步数:");
    let sample_newton = vec![0.1, 0.2, 0.9, 9.0];
    for x in &sample_newton {
        let (root,iter_count) = newton(test, test_deriative, *x, 1e-6);
        println!("初值 {:2.1}, {:16.12e}, {}", *x, root, iter_count);
    }
    println!("弦截法，初值、根和迭代步数:");
    let sample_secant = vec![0.0, 0.1, 0.2, 0.9, 8.0, 9.0];
    for i in 0..sample_secant.len()-1 {
        let (root,iter_count) = secant(test, sample_secant[i], sample_secant[i+1], 1e-4);
        println!("初值 {:2.1},{:2.1}, {:16.12e}, {}", sample_secant[i], sample_secant[i+1], root, iter_count);
    }
}
