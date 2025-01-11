use super::dimension::Dimension;

#[derive(Debug, serde::Deserialize)]
pub struct StrokeColor {
    pub name: String,
    pub hex: String,
    pub id: String,
}

pub trait Strokeable {
    fn has_stroke(&self) -> bool;
    fn stroke_color(&self) -> Option<StrokeColor>;
    fn stroke_width(&self) -> Option<Dimension>;
}
