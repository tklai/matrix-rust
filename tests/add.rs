use matrix::{create_matrix, create_matrix_row};

#[test]
fn test_addition() {
    let matrix_a = create_matrix!(create_matrix_row!(1.0, 5.0), create_matrix_row!(-4.0, 3.0));
    let matrix_b = create_matrix!(create_matrix_row!(2.0, -1.0), create_matrix_row!(4.0, -1.0));
    let matrix_result = create_matrix!(create_matrix_row!(3.0, 4.0), create_matrix_row!(0.0, 2.0));

    assert_eq!(matrix_a + matrix_b, matrix_result);
}
