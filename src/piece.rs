use crate::shape::Shape;

struct Piece {
    name: String,
    distinct_shapes: Vec<Shape>
}

impl Piece {
    fn new(name: String, base_shape: Shape) -> Self {
        Self {
            name,
            distinct_shapes: vec![], // TODO
        }
    }
}
