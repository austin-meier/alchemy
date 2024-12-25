pub trait Dimensionable {
    fn width() -> Dimension;
    fn height() -> Dimension;
}

// Base dimension unit will be inches since it makes for the easiest conversion
// to Point. Most common for PDF format
#[derive(Copy,Clone)]
pub struct Dimension(f64);

#[derive(Copy,Clone)]
pub enum DimensionUnit {
    Inch,
    Point,
    Centimeter,
    Meter,
}

impl DimensionUnit {
    fn in_inches(self) -> f64 {
        match self {
            Self::Inch => 1.0,
            Self::Point => 0.0138889,
            Self::Centimeter => 0.393701,
            Self::Meter => 39.3701,
        }
    }
}

impl Dimension {
    pub fn new(value: f64, unit: DimensionUnit)-> Self {
        Dimension(value * unit.in_inches())
    }

    pub fn as_unit(&self, unit: DimensionUnit) -> f64 {
        self.0 / unit.in_inches()
    }

    pub fn from_pixels(pixels: f64, dpi: f64) -> Self {
      Dimension(pixels/dpi)
    }

    pub fn as_pixels(value: f64, unit: DimensionUnit, dpi: f64) -> f64 {
        (value * unit.in_inches()) * dpi
    }
}