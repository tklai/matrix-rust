use std::ops::{Add, Mul, Sub};

/// An alias represents matrix row.
pub type Row = Vec<f32>;

#[derive(Debug)]
/// Matrix Metadata Storage
struct MatrixMeta {
    /// Row count
    row_count: usize,
    /// Column count
    column_count: usize,
}

#[derive(Debug)]
/// Matrix itself
pub struct Matrix {
    /// The matrix metadata. This meta will be generated automatically.
    meta: MatrixMeta,
    /// The matrix data
    data: Vec<Row>,
}

impl Matrix {
    /// Return a new matrix with metadata generated.
    ///
    /// # Arguments
    ///
    /// * `data` - A vector that stores multiple `Row`s.
    ///
    /// # Notes
    ///
    /// Try to use macros `create_matrix!()`.
    /// ```
    /// use matrix::{create_matrix, create_matrix_row};
    ///
    /// let matrix = create_matrix!(
    ///     create_matrix_row!(1.0,2.0),
    ///     create_matrix_row!(3.0,4.0)
    /// );
    pub fn new(data: Vec<Row>) -> Matrix {
        let meta = MatrixMeta {
            row_count: data.len(),
            column_count: data.get(0).unwrap().len(),
        };

        for row_index in 0..data.len() {
            if data.get(row_index).unwrap().len() != meta.column_count {
                panic!(
                    "Column counts not match in row {}. (Found {}, Expected {})",
                    row_index,
                    data.get(row_index).unwrap().len(),
                    meta.column_count,
                );
            }
        }

        Matrix { meta, data }
    }
}

impl PartialEq<Matrix> for Matrix {
    /// Compare two matrices with same structure and data.
    fn eq(&self, other: &Matrix) -> bool {
        self.meta.row_count == other.meta.row_count
            && self.meta.column_count == other.meta.column_count
            && self.data == other.data
    }
}

impl Add<Matrix> for Matrix {
    /// A new matrix object will be returned.
    type Output = Self;

    /// Perform additions between two matrices and return a result matrix.
    ///
    /// ![additions](https://latex.codecogs.com/png.image?%5Cinline%20%5Clarge%20%5Cdpi%7B110%7D%5Cbg%7Bwhite%7D%5Cbegin%7Bbmatrix%7D1%20&%205%5C%5C-4%20&%203%5Cend%7Bbmatrix%7D&plus;%5Cbegin%7Bbmatrix%7D2%20&%20-1%20%5C%5C4%20&%20-1%5Cend%7Bbmatrix%7D=%5Cbegin%7Bbmatrix%7D3%20&%204%20%5C%5C%200%20&%202%5Cend%7Bbmatrix%7D)
    ///
    /// # Example
    /// ```
    ///   use matrix::{create_matrix, create_matrix_row};
    ///
    ///   let lhs = create_matrix!(create_matrix_row!(1.0, 5.0), create_matrix_row!(-4.0, 3.0));
    ///   let rhs = create_matrix!(create_matrix_row!(2.0, -1.0), create_matrix_row!(4.0, -1.0));
    ///   let returning_result = create_matrix!(create_matrix_row!(3.0, 4.0), create_matrix_row!(0.0, 2.0));
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        if self.meta.row_count != rhs.meta.row_count
            || self.meta.column_count != rhs.meta.column_count
        {
            panic!("These two matrices cannot be added.");
        } else {
            let mut matrix_vec = vec![];
            for row_index in 0..self.meta.row_count {
                let mut matrix_column_vec = vec![];
                for column_index in 0..self.meta.column_count {
                    matrix_column_vec.push(
                        self.data.get(row_index).unwrap().get(column_index).unwrap()
                            + rhs.data.get(row_index).unwrap().get(column_index).unwrap(),
                    );
                }
                matrix_vec.push(matrix_column_vec);
            }

            Matrix::new(matrix_vec)
        }
    }
}

/// Perform substraction between two matrices and return a result matrix.
impl Sub<Matrix> for Matrix {
    /// A new matrix object will be returned.
    type Output = Self;

    /// Perform substraction between two matrices and return a result matrix.
    ///
    /// ![substraction](https://latex.codecogs.com/png.image?%5Cinline%20%5Clarge%20%5Cdpi%7B110%7D%5Cbg%7Bwhite%7D%5Cbegin%7Bbmatrix%7D4%20&%205%20&%206%5C%5C2%20&%203%20&%204%5Cend%7Bbmatrix%7D-%5Cbegin%7Bbmatrix%7D2%20&%204%20&%206%20%5C%5C1%20&%202%20&%203%5Cend%7Bbmatrix%7D=%5Cbegin%7Bbmatrix%7D2%20&%201%20&%200%20%5C%5C%201%20&%201%20&%201%5Cend%7Bbmatrix%7D)
    ///
    /// # Example
    /// ```
    /// use matrix::{create_matrix, create_matrix_row};
    ///
    /// let lhs = create_matrix!(create_matrix_row!(4.0, 5.0, 6.0), create_matrix_row!(2.0, 3.0, 4.0));
    /// let rhs = create_matrix!(create_matrix_row!(2.0, 4.0, 6.0), create_matrix_row!(1.0, 2.0, 3.0));
    /// let returning_result = create_matrix!(create_matrix_row!(2.0, 1.0, 0.0), create_matrix_row!(1.0, 1.0, 1.0));
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        if self.meta.row_count != rhs.meta.row_count
            || self.meta.column_count != rhs.meta.column_count
        {
            panic!("These two matrices cannot be substracted.");
        } else {
            let mut matrix_vec = vec![];
            for row_index in 0..self.meta.row_count {
                let mut matrix_column_vec = vec![];
                for column_index in 0..self.meta.column_count {
                    matrix_column_vec.push(
                        self.data.get(row_index).unwrap().get(column_index).unwrap()
                            - rhs.data.get(row_index).unwrap().get(column_index).unwrap(),
                    );
                }
                matrix_vec.push(matrix_column_vec);
            }

            Matrix::new(matrix_vec)
        }
    }
}

impl Mul<Matrix> for Matrix {
    /// A new matrix object will be returned.
    type Output = Self;

    /// Perform multiplication between two matrices and return a result matrix.
    ///
    /// ![multiplication_1](https://latex.codecogs.com/png.image?%5Cinline%20%5Clarge%20%5Cdpi%7B110%7D%5Cbg%7Bwhite%7D%5Cbegin%7Bbmatrix%7D1%20&%202%20&%203%5C%5C4%20&%205%20&%206%5Cend%7Bbmatrix%7D*%5Cbegin%7Bbmatrix%7D7%20&%208%20%5C%5C9%20&%2010%5C%5C%2011%20&%2012%5Cend%7Bbmatrix%7D=%5Cbegin%7Bbmatrix%7D58%20&%2064%20%5C%5C%20139%20&%20154%5Cend%7Bbmatrix%7D)
    ///
    /// # Example 1
    /// ```
    /// use matrix::{create_matrix, create_matrix_row};
    ///
    /// let lhs = create_matrix!(create_matrix_row!(1.0, 2.0, 3.0), create_matrix_row!(4.0, 5.0, 6.0));
    /// let rhs = create_matrix!(create_matrix_row!(7.0, 8.0), create_matrix_row!(9.0, 10.0), create_matrix_row!(11.0, 12.0));
    /// let returning_result = create_matrix!(create_matrix_row!(58.0, 64.0), create_matrix_row!(139.0, 154.0));
    /// ```
    ///
    /// ![multiplication_2](https://latex.codecogs.com/png.image?%5Cinline%20%5Clarge%20%5Cdpi%7B110%7D%5Cbg%7Bwhite%7D%5Cbegin%7Bbmatrix%7D1%20&%202%20&%203%5Cend%7Bbmatrix%7D*%5Cbegin%7Bbmatrix%7D4%20%5C%5C5%20%5C%5C%206%5Cend%7Bbmatrix%7D=%5Cbegin%7Bbmatrix%7D32%5Cend%7Bbmatrix%7D)
    ///
    /// # Example 2
    /// ```
    /// use matrix::{create_matrix, create_matrix_row};
    ///
    /// let lhs = create_matrix!(create_matrix_row!(1.0, 2.0, 3.0));
    /// let rhs = create_matrix!(create_matrix_row!(4.0), create_matrix_row!(5.0), create_matrix_row!(6.0));
    /// let return_result = create_matrix!(create_matrix_row!(32.0));
    /// ```
    ///
    /// ![multiplication_3](https://latex.codecogs.com/png.image?%5Cinline%20%5Clarge%20%5Cdpi%7B110%7D%5Cbg%7Bwhite%7D%5Cbegin%7Bbmatrix%7D4%20%5C%5C5%20%5C%5C%206%5Cend%7Bbmatrix%7D*%5Cbegin%7Bbmatrix%7D1%20&%202%20&%203%5Cend%7Bbmatrix%7D=%5Cbegin%7Bbmatrix%7D4%20&%208%20&%2012%20%5C%5C%205%20&%2010%20&%2015%20%5C%5C%206%20&%2012%20&%2018%20%5Cend%7Bbmatrix%7D)
    ///
    /// # Example 3
    /// ```
    /// use matrix::{create_matrix, create_matrix_row};
    ///
    /// let lhs = create_matrix!(create_matrix_row!(4.0), create_matrix_row!(5.0), create_matrix_row!(6.0));
    /// let rhs = create_matrix!(create_matrix_row!(1.0, 2.0, 3.0));
    /// let return_result = create_matrix!(
    ///     create_matrix_row!(4.0, 8.0, 12.0),
    ///     create_matrix_row!(5.0, 10.0, 15.0),
    ///     create_matrix_row!(6.0, 12.0, 18.0)
    /// );
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        if self.meta.column_count != rhs.meta.row_count {
            panic!("These two matrices cannot be multiplied.");
        } else {
            let mut matrix_vec = vec![];
            for row_index in 0..self.meta.row_count {
                let mut matrix_column_vec = vec![];
                for column_index in 0..rhs.meta.column_count {
                    let mut result: f32 = 0.0;
                    for (index, value) in self.data.get(row_index).unwrap().iter().enumerate() {
                        result += value * rhs.data.get(index).unwrap().get(column_index).unwrap();
                    }

                    matrix_column_vec.push(result);
                }
                matrix_vec.push(matrix_column_vec);
            }

            Matrix::new(matrix_vec)
        }
    }
}

impl Mul<Matrix> for f32 {
    /// A new matrix object will be returned.
    type Output = Matrix;

    /// Perform multiplication between a matrix and a float number, then return a result matrix.
    fn mul(self, matrix: Matrix) -> Self::Output {
        let mut matrix_vec = vec![];
        for row_index in 0..matrix.meta.row_count {
            let mut matrix_column_vec = vec![];
            for column_index in 0..matrix.meta.column_count {
                matrix_column_vec.push(
                    self * matrix
                        .data
                        .get(row_index)
                        .unwrap()
                        .get(column_index)
                        .unwrap(),
                );
            }
            matrix_vec.push(matrix_column_vec);
        }

        Matrix::new(matrix_vec)
    }
}

#[macro_export]
/// Macro the create a matrix row.
///
/// # Examples
///
/// ```
/// use matrix::{create_matrix, create_matrix_row};
///
/// // This matrix contains two rows and each row contains two columns
/// let matrix = create_matrix!(
///     create_matrix_row!(1.0,2.0),
///     create_matrix_row!(3.0,4.0)
/// );
macro_rules! create_matrix {
    ($($x:expr), *) => (
        {
            let mut matrix = Vec::new();
            $(
                matrix.push($x);
            )*
            matrix::matrix::Matrix::new(matrix)
        }
    );
}

#[macro_export]
/// Macro the create a matrix row.
///
/// # Examples
///
/// ```
/// use matrix::create_matrix_row;
///
/// // This row contains two columns
/// let row = create_matrix_row!(1.0, 2.0);
/// ```
macro_rules! create_matrix_row {
    ($($x:expr), *) => (
        {
            let mut row = matrix::matrix::Row::new();
            $(
                row.push($x);
            )*
            row
        }
    );
}
