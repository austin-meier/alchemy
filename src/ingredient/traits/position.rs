use super::dimension::Dimension;

pub trait Positionable {
    fn get_x(&self) -> Dimension;
    fn get_y(&self) -> Dimension;
}
