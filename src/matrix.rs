

pub struct Matrix<const N: usize, const M: usize> {
    data: [[f64; M]; N],
}

impl<N,M> Matrix<N,M>{
    
    // return a specific element
    fn element(&self, row: usize, column: usize) -> f64 {
        return self.data[row][column];
    }

    // return a whole row of data within the matrix
    fn row(&self, row: usize) -> [f64] {
        return self.data[row];
    }

    // return a column of data from within the matrix
    fn column(&self, column: usize) -> [f64] {
        let column_data: [f64; self.data.len()] = [0; self.data.len()];

        for i in 0..self.data.len() {
            column_data[i] = self.data[i][column];
        }

        return column_data;
    }

}

