use ark_ff::fields::{Field};

#[derive(Clone, Debug)]
pub struct Matrix<T: Field> {
    entries: Vec<T>,
    myheight: usize,
    mywidth: usize,
}

impl<T: Field> Matrix<T> {
    pub fn height(&self) -> usize {
        self.myheight
    }

    pub fn width(&self) -> usize {
        self.mywidth
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        let width = self.width();
        self.entries[row * width + col]
    }

    pub fn set(&mut self, row: usize, col: usize, val: T) {
        let width = self.width();
        self.entries[row * width + col] = val;
    }

    pub fn from_nested_vec(height: usize, width: usize, vec: Vec<Vec<T>>) -> Self {
        let mut entries = Vec::new();
        entries.resize(height*width, T::default());
        let mut matrix = Self { entries, myheight: height, mywidth: width };

        assert!(vec.len() == height);
        for row in 0..height {
            assert!(vec[row].len() == width);
            for col in 0..width {
                matrix.set(row, col, vec[row][col]);
            }
        }

        matrix
    }

    fn from_diagonal_element(height: usize, off_diag_elem: T, on_diag_elem: T) -> Self {
        let mut entries = Vec::new();
        entries.resize(height*height, off_diag_elem);
        let mut matrix = Self { entries, myheight: height, mywidth: height };

        for row in 0..height {
            matrix.set(row, row, on_diag_elem);
        }

        matrix
    }

    fn hcat(&self, other: &Self) -> Self {
        assert!(self.height() == other.height());

        let height = self.height();
        let width = self.width() + other.width();

        let mut entries = Vec::new();
        entries.resize(height*width, T::default());
        let mut matrix = Self { entries, myheight: height, mywidth: width };

        for row in 0..height {
            for col in 0..self.width() {
                matrix.set(row, col, self.get(row, col));
            }
            for col in 0..other.width() {
                matrix.set(row, self.width() + col, other.get(row, col));
            }
        }

        matrix
    }

    fn hpick(&self, idxs: &[usize]) -> Self {
        let height = self.height();
        let width = idxs.len();

        let mut entries = Vec::new();
        entries.resize(height*width, T::default());
        let mut matrix = Self { entries, myheight: height, mywidth: width };

        for row in 0..height {
            for (col_new, col_old) in idxs.iter().enumerate() {
                matrix.set(row, col_new, self.get(row, *col_old));
            }
        }

        matrix
    }

    fn divide_row(&mut self, row: usize, val: T) {
        for col in 0..self.width() {
            self.set(row, col, self.get(row, col) / val);
        }
    }

    fn multiply_row_and_add_to_row(&mut self, row_src: usize, val: T, row_dst: usize) {
        for col in 0..self.width() {
            self.set(row_dst, col, self.get(row_dst, col) + val * self.get(row_src, col));
        }
    }

    pub fn invert(&self) -> Self {
        assert!(self.width() == self.height());

        let d = self.height();
        let mut matrix = self.hcat(&Matrix::from_diagonal_element(d, T::zero(), T::one()));

        for i in 0..d {
            let pivot = matrix.get(i, i);
            assert!(!pivot.is_zero());

            matrix.divide_row(i, pivot);

            for j in 0..d {
                if j != i {
                    matrix.multiply_row_and_add_to_row(i, -matrix.get(j, i), j);
                }
            }
        }
    
        matrix.hpick(&(d..2*d).collect::<Vec<usize>>())
    }
}
