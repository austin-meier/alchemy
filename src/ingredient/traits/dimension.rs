use std::{fmt::Display, ops::Mul};
use crate::ingredient::types::base::Ingredient;

pub trait Dimensionable {
    fn width(&self) -> Dimension;
    fn height(&self) -> Dimension;
}

pub struct Rectangle {
    width: Dimension,
    height: Dimension
}

impl Rectangle {
    pub fn new() -> Self {
        Rectangle {
            width: Dimension::new(0.0, DimensionUnit::Inch),
            height: Dimension::new(0.0, DimensionUnit::Inch)
        }
    }

    pub fn area(&self) -> Dimension {
        self.height * self.width
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

impl From<&Ingredient> for Rectangle {
    fn from(frame: &Ingredient) -> Self {
        match frame {
            Ingredient::Image(image) => {
                Rectangle {
                    width: image.frame.width(),
                    height: image.frame.height()
                }
            },

            Ingredient::Text(text) => {
                Rectangle {
                    width: text.frame.width(),
                    height: text.frame.height()
                }
            },

            Ingredient::Shape(shape) => {
                Rectangle {
                    width: shape.frame.width(),
                    height: shape.frame.height()
                }
            },

            _ => {
                Rectangle::new()
            }

        }
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} in. x {} in.", self.width, self.height)
    }
}
// Base dimension unit will be inches since it makes for the easiest conversion
// to Point. Most common for PDF format
#[derive(Copy,Clone, PartialEq, PartialOrd)]
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