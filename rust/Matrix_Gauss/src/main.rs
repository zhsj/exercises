// Ax = b
// A:m*n, x:n*1, b:m*1

fn max_in_column(matrix: &Vec<Vec<f64>>, col: usize) -> usize {
    let m = matrix.len();
    let mut result = col;
    let mut max_t = matrix[col][col];
    for i in (col+1)..m {
        if matrix[i][col] > max_t {
            max_t = matrix[i][col];
            result = i;
        }
    }
    result
}

fn gauss(matrix_a: Vec<Vec<f64>>, matrix_b: &Vec<f64>) -> Vec<f64> {
    let m = matrix_a.len();
    assert!(m>0);
    assert!(matrix_b.len() == m);
    let n = matrix_a[0].len();
    assert!(m==n);
    let mut matrix = matrix_a;
    for row in 0..m {
        matrix[row].push(matrix_b[row]);
    }
    for row in 0..m {
        let max_index = max_in_column(&matrix, row);
        if max_index != row {
            let row_temp = matrix[row].clone();
            matrix[row] = matrix[max_index].clone();
            matrix[max_index] = row_temp;
        }
        for row_to_eliminate in (row+1)..m {
            let t = matrix[row_to_eliminate][row]/matrix[row][row];
            for col in (row)..(m+1) {
                matrix[row_to_eliminate][col] -= t * matrix[row][col];
            }
        }
    }

    let mut result = vec![0.0; m];
    for row in (0..m).rev() {
        let mut s = matrix[row][m];
        for col in (row+1)..m {
            s-= matrix[row][col]*result[col];
        }
        result[row] = s/matrix[row][row];
    }
    return result;
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
    let matrix_x = gauss(matrix_a, &matrix_b);
    println!("根为：");
    for xi in matrix_x {
        println!("{:+16.12e} ", xi);
    }
}
