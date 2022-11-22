use matrix::matrix::Matrix;

#[test]
fn test_substraction() {
    let matrix_a = Matrix::new(vec![vec![4.0, 5.0, 6.0], vec![2.0, 3.0, 4.0]]);
    let matrix_b = Matrix::new(vec![vec![2.0, 4.0, 6.0], vec![1.0, 2.0, 3.0]]);
    let matrix_result = Matrix::new(vec![vec![2.0, 1.0, 0.0], vec![1.0, 1.0, 1.0]]);

    assert_eq!(matrix_a - matrix_b, matrix_result);
}
