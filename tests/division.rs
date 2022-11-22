use matrix::matrix::Matrix;

#[test]
fn test_matrix_multiplication() {
    let sample_a_matrix_a = Matrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let sample_a_matrix_b = Matrix::new(vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]]);
    let sample_a_matrix_result = Matrix::new(vec![vec![58.0, 64.0], vec![139.0, 154.0]]);

    assert_eq!(
        sample_a_matrix_a * sample_a_matrix_b,
        sample_a_matrix_result
    );

    let sample_b1_matrix_a = Matrix::new(vec![vec![1.0, 2.0, 3.0]]);
    let sample_b1_matrix_b = Matrix::new(vec![vec![4.0], vec![5.0], vec![6.0]]);
    let sample_b1_matrix_result = Matrix::new(vec![vec![32.0]]);

    assert_eq!(
        sample_b1_matrix_a * sample_b1_matrix_b,
        sample_b1_matrix_result
    );

    let sample_b2_matrix_a = Matrix::new(vec![vec![4.0], vec![5.0], vec![6.0]]);
    let sample_b2_matrix_b = Matrix::new(vec![vec![1.0, 2.0, 3.0]]);
    let sample_b2_matrix_result = Matrix::new(vec![
        vec![4.0, 8.0, 12.0],
        vec![5.0, 10.0, 15.0],
        vec![6.0, 12.0, 18.0],
    ]);

    assert_eq!(
        sample_b2_matrix_a * sample_b2_matrix_b,
        sample_b2_matrix_result
    );

    let sample_c1_matrix_a = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let sample_c1_matrix_b = Matrix::new(vec![vec![2.0, 0.0], vec![1.0, 2.0]]);
    let sample_c1_matrix_result = Matrix::new(vec![vec![4.0, 4.0], vec![10.0, 8.0]]);

    assert_eq!(
        sample_c1_matrix_a * sample_c1_matrix_b,
        sample_c1_matrix_result
    );

    let sample_c2_matrix_a = Matrix::new(vec![vec![2.0, 0.0], vec![1.0, 2.0]]);
    let sample_c2_matrix_b = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let sample_c2_matrix_result = Matrix::new(vec![vec![2.0, 4.0], vec![7.0, 10.0]]);

    assert_eq!(
        sample_c2_matrix_a * sample_c2_matrix_b,
        sample_c2_matrix_result
    );

    let sample_d_matrix_a = Matrix::new(vec![
        vec![12.0, 8.0, 4.0],
        vec![3.0, 17.0, 14.0],
        vec![9.0, 8.0, 10.0],
    ]);
    let sample_d_matrix_b = Matrix::new(vec![
        vec![5.0, 19.0, 3.0],
        vec![6.0, 15.0, 9.0],
        vec![7.0, 8.0, 16.0],
    ]);
    let sample_d_matrix_result = Matrix::new(vec![
        vec![136.0, 380.0, 172.0],
        vec![215.0, 424.0, 386.0],
        vec![163.0, 371.0, 259.0],
    ]);

    assert_eq!(
        sample_d_matrix_a * sample_d_matrix_b,
        sample_d_matrix_result
    );
}
