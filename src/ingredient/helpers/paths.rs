use super::deserializers::parse_bootleg_boolean;

#[derive(Debug, serde::Deserialize)]
pub struct Path {
    #[serde(alias = "isClosed", deserialize_with = "parse_bootleg_boolean")]
    pub is_closed: bool,
    pub points: Vec<Point>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Point {
    #[serde(alias = "line")]
    Line(LinePath),
}

#[derive(Debug, serde::Deserialize)]
pub struct LinePath {
    pub x: f64,
    pub y: f64,
}
