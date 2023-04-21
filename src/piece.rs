use std::collections::HashSet;
use crate::shape::Shape;

#[derive(Clone, Debug)]
pub(crate) struct Piece {
    pub(crate) name: String,
    pub(crate) distinct_shapes: HashSet<Shape>
}

impl Piece {
    pub(crate) fn new(name: String, base_shape: Shape) -> Self {
        Self {
            name,
            distinct_shapes: base_shape.generate_all_distinct_variations(),
        }
    }

    pub fn get_all_pieces() -> Vec<Piece> {
        vec![
            Piece::new("I".to_string(), Shape {
                tile_matrix: vec![
                    vec![" 🟦", " 🟦", " 🟦", " 🟦"],
                ]
            }),
            Piece::new("Short L".to_string(), Shape {
                tile_matrix: vec![
                    vec![" 🟥", " 🟥", " 🟥"],
                    vec!["   ", "   ", " 🟥"],
                ]
            }),
            Piece::new("Long L".to_string(), Shape {
                tile_matrix: vec![
                    vec![" 🟨", " 🟨", " 🟨", " 🟨"],
                    vec!["   ", "   ", "   ", " 🟨"],
                ]
            }),
            Piece::new("T".to_string(), Shape {
                tile_matrix: vec![
                    vec![" 🟫", " 🟫", " 🟫"],
                    vec!["   ", " 🟫", "   "],
                    vec!["   ", " 🟫", "   "],
                ]
            }),
            Piece::new("S".to_string(), Shape {
                tile_matrix: vec![
                    vec!["   ", " 🟩", " 🟩"],
                    vec![" 🟩", " 🟩", "   "],
                ]
            }),
            Piece::new(".S".to_string(), Shape {
                tile_matrix: vec![
                    vec!["   ", "   ", " 🟧", " 🟧"],
                    vec![" 🟧", " 🟧", " 🟧", "   "],
                ]
            }),
            Piece::new("d".to_string(), Shape {
                tile_matrix: vec![
                    vec!["   ", " 🟪"],
                    vec![" 🟪", " 🟪"],
                    vec![" 🟪", " 🟪"],
                ]
            }),
            Piece::new("U".to_string(), Shape {
                tile_matrix: vec![
                    vec![" ⬜️", "   ", " ⬜️"],
                    vec![" ⬜️", " ⬜️", " ⬜️"],
                ]
            }),
            Piece::new("Corner".to_string(), Shape {
                tile_matrix: vec![
                    vec![" ⬛️", " ⬛️", " ⬛️"],
                    vec![" ⬛️", "   ", "   "],
                    vec![" ⬛️", "   ", "   "],
                ]
            }),
            Piece::new("Z".to_string(), Shape {
                tile_matrix: vec![
                    vec![" 🔳", " 🔳", "   "],
                    vec!["   ", " 🔳", "   "],
                    vec!["   ", " 🔳", " 🔳"],
                ]
            }),
        ]
    }
}
