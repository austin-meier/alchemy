use std::{fmt::Display, ops::Mul};

pub trait Dimensionable {
    fn width(&self) -> Dimension;
    fn height(&self) -> Dimension;
    fn area(&self) -> Dimension {
        self.width() * self.height()
    }
}

pub struct Rectangle {
    pub width: Dimension,
    pub height: Dimension,
}

impl Rectangle {
    pub fn new() -> Self {
        Rectangle { width: Dimension(0.0), height: Dimension(0.0) }
    }
}

impl Dimensionable for Rectangle {

    fn height(&self) -> Dimension {
        self.height
    }

    fn width(&self) -> Dimension {
        self.width
    }
}

// Base dimension unit will be inches since it makes for the easiest conversion
// to Point. Most common for PDF format
#[derive(Copy, Clone, PartialEq, PartialOrd)]
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

    pub fn multiply(&self, dim: Dimension) -> Dimension {
       Dimension(self.0 * dim.as_unit(DimensionUnit::Inch))
    }

}

/* Used for operator overloading the * operator */
impl Mul for Dimension {
    type Output = Dimension;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}


impl Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} inches", self.0)
    }
}