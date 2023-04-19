use std::collections::HashSet;
use std::fmt;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Shape {
    pub tile_matrix: Vec<Vec<&'static str>>
}

impl Shape {

    pub(crate) fn generate_all_distinct_variations(self) -> HashSet<Shape> {
        let original = self;
        let rotated90 = original.rotate_90();
        let rotated180 = rotated90.rotate_90();
        let rotated270 = rotated180.rotate_90();

        let flipped = original.flip();
        let flipped_rotated90 = flipped.rotate_90();
        let flipped_rotated180 = flipped_rotated90.rotate_90();
        let flipped_rotated270 = flipped_rotated180.rotate_90();

        HashSet::from([
            original,
            rotated90,
            rotated180,
            rotated270,
            flipped,
            flipped_rotated90,
            flipped_rotated180,
            flipped_rotated270,
        ])
    }

    fn matrix_transpose<'a>(matrix: &Vec<Vec<&'a str>>) -> Vec<Vec<&'a str>> {
        let mut transposed = vec![Vec::with_capacity(matrix.len()); matrix[0].len()];
        for row in matrix {
            for i in 0..row.len() {
                transposed[i].push(row[i].clone());
            }
        }
        transposed
    }

    fn flip(&self) -> Shape {
        let mut result = self.tile_matrix.clone();
        result.iter_mut().for_each(|row| row.reverse());
        Shape { tile_matrix: result }
    }

    // TODO remove pub
    fn rotate_90(&self) -> Shape {
        let mut result = Self::matrix_transpose(&self.tile_matrix); // first transpose
        result.iter_mut().for_each(|row| row.reverse()); // then reverse each row
        Shape { tile_matrix: result }
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.tile_matrix {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
