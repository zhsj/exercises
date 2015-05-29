// Ax = b
// A:m*n, x:n*1, b:m*1

fn distance(matrix_a: &Vec<f64>, matrix_b: &Vec<f64>) -> f64 {
    let m = matrix_a.len();
    assert!(m>0);
    assert!(m==matrix_b.len());
    let mut max = 0.0;
    for i in 0..m {
        let sum = (matrix_a[i]-matrix_b[i]).abs();
        if sum > max {
            max = sum;
        }
    }
    return max;
}

// http://upload.wikimedia.org/math/5/8/7/58789cb3e9b043417e1d4042a25af2dc.png
fn gauss_seidel(matrix_a: &Vec<Vec<f64>>, matrix_b: &Vec<f64>) -> (Vec<f64>, i32) {
    let m = matrix_a.len();
    assert!(m>0);
    assert!(matrix_b.len() == m);
    let n = matrix_a[0].len();
    assert!(m==n);

    let mut result = matrix_b.clone();

    let mut iter_times = 0;
    while iter_times < 100000 {
        let mut result_new = vec![0.0;m];
        for i in 0..m {
            result_new[i] = matrix_b[i];
            for j in 0..i {
                result_new[i] -= matrix_a[i][j]*result_new[j];
            }
            for j in i+1..m {
                result_new[i]-= matrix_a[i][j]*result[j];
            }
            result_new[i] /= matrix_a[i][i];
        }
        if distance(&result_new, &result) < 1e-6 {
            break;
        }
        result = result_new;
        iter_times += 1;
    }
    return (result,iter_times);
}

fn sor(matrix_a: &Vec<Vec<f64>>, matrix_b: &Vec<f64>, w:f64) -> (Vec<f64>, i32) {
    let m = matrix_a.len();
    assert!(m>0);
    assert!(matrix_b.len() == m);
    let n = matrix_a[0].len();
    assert!(m==n);

    let mut result = matrix_b.clone();

    let mut iter_times = 0;
    while iter_times < 100000 {
        let mut result_new = vec![0.0;m];
        for i in 0..m {
            result_new[i] = matrix_b[i];
            for j in 0..i {
                result_new[i] -= matrix_a[i][j]*result_new[j];
            }
            for j in i+1..m {
                result_new[i]-= matrix_a[i][j]*result[j];
            }
            result_new[i] /= matrix_a[i][i];
            result_new[i] = (1.0-w)*result[i] + w*result_new[i];
        }
        if distance(&result_new, &result) < 1e-6 {
            break;
        }
        result = result_new;
        iter_times += 1;
    }
    return (result,iter_times);
}
fn main() {
    let matrix_a = vec![
        vec![ 31.0, -13.0,   0.0,   0.0,   0.0, -10.0,   0.0,   0.0,   0.0],
        vec![-13.0,  35.0,  -9.0,   0.0, -11.0,   0.0,   0.0,   0.0,   0.0],
        vec![  0.0,  -9.0,  31.0, -10.0,   0.0,   0.0,   0.0,   0.0,   0.0],
        vec![  0.0,   0.0, -10.0,  79.0, -30.0,   0.0,   0.0,   0.0,  -9.0],
        vec![  0.0,   0.0,   0.0, -30.0,  57.0,  -7.0,   0.0,  -5.0,   0.0],
        vec![  0.0,   0.0,   0.0,   0.0,  -7.0,  47.0, -30.0,   0.0,   0.0],
        vec![  0.0,   0.0,   0.0,   0.0,   0.0, -30.0,  41.0,   0.0,   0.0],
        vec![  0.0,   0.0,   0.0,   0.0,  -5.0,   0.0,   0.0,  27.0,  -2.0],
        vec![  0.0,   0.0,   0.0,  -9.0,   0.0,   0.0,   0.0,  -2.0,  29.0],
    ];
    let matrix_b = vec![-15.0, 27.0, -23.0, 0.0, -20.0, 12.0, -7.0, 7.0, 10.0];
    let (matrix_x, iter_time) = gauss_seidel(&matrix_a, &matrix_b);
    println!("根为：");
    for xi in matrix_x {
        println!("{:+16.12e} ", xi);
    }
    println!("Gauss-Seidel迭代总迭代步数为：{}", iter_time);

    println!("SOR迭代步数为:");
    let mut best = -1.0;
    let mut best_iter = 100000;
    for wi in 1..100 {
        let w = wi as f64/50.0;
        let (_, iter_time) = sor(&matrix_a, &matrix_b,w);
        println!("松弛因子：{:.2}, {}", w, iter_time);
        if iter_time < best_iter {
            best_iter = iter_time;
            best = w;
        }
    }
    println!("最佳松弛因子为：{:.2}", best);

}
