use matrix::matrix::Matrix;

#[test]
fn test_scalar_multiplication() {
    let matrix_to_be_multiplied = Matrix::new(vec![vec![4.0, 0.0], vec![1.0, -9.0]]);
    let matrix_result = Matrix::new(vec![vec![8.0, 0.0], vec![2.0, -18.0]]);

    assert_eq!(2.0 * matrix_to_be_multiplied, matrix_result);
}
