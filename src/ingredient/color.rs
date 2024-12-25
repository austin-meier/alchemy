
pub trait Colorant {
    fn as_rgb(&self) -> RGBColor;
    fn as_rgba(&self) -> RGBAColor;
    fn as_cmyk(&self) -> CMYKColor;
}

#[derive(Copy, Clone, Debug)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl RGBColor {
    fn as_array(&self) -> [u8;3] {
        [self.r.into(), self.g.into(), self.b.into()]
    }

    fn estimate_cmyk(&self) -> CMYKColor {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let k = 1.0 - f32::max(f32::max(r, g), b);
        if k == 1.0 {
            CMYKColor { c: 0, m: 0, y: 0, k: 100 }
        } else {
            let c = ((1.0 - r - k) / (1.0 - k)) * 100.0;
            let m = ((1.0 - g - k) / (1.0 - k)) * 100.0;
            let y = ((1.0 - b - k) / (1.0 - k)) * 100.0;
            CMYKColor {
                c: c.round() as u8,
                m: m.round() as u8,
                y: y.round() as u8,
                k: k.round() as u8
            }
        }
    }
}

impl Default for RGBColor {
    fn default() -> RGBColor {
        RGBColor{r: 0, g: 0, b: 0}
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RGBAColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Default for RGBAColor {
    fn default() -> RGBAColor {
        RGBAColor{r: 0, g: 0, b: 0, a: 0}
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CMYKColor {
    pub c: u8,
    pub m: u8,
    pub y: u8,
    pub k: u8
}

impl CMYKColor {
    fn as_array(&self) -> [u8;4] {
        [self.c.into(), self.m.into(), self.y.into(), self.k.into()]
    }

    fn estimate_rgb(&self) -> RGBColor {
        RGBColor {
            r: 255 * (100 - self.c) * (100 - self.k),
            g: 255 * (100 - self.m) * (100 - self.k),
            b: 255 * (100 - self.y) * (100 - self.k)
        }
    }
}

impl Default for CMYKColor {
    fn default() -> CMYKColor {
        CMYKColor{c: 0, m: 0, y: 0, k: 0}
    }
}


pub enum ColorSpace {
    Unknown,
    sRGB,
    USWebCoatedSWOPV2,
}

pub struct Color {
    pub rgb: Option<RGBColor>,
    pub alpha: Option<u8>,
    pub cmyk: Option<CMYKColor>,
}

impl Default for Color {
    fn default() -> Color {
        Color{
            rgb: Some(RGBColor::default()),
            alpha: Some(100),
            cmyk: Some(CMYKColor::default())
        }
    }
}

pub struct NamedInk<T: Colorant> {
    pub name: String,
    pub color: T,
    pub color_space: Option<ColorSpace>
}

impl Color {
    fn to_rgba(&self) -> RGBAColor {
        let rgb = self.as_rgb();
        RGBAColor{
            r: rgb.r,
            g: rgb.b,
            b: rgb.b,
            a: self.alpha.unwrap_or_default()
        }
    }
}

impl Colorant for Color {
    fn as_rgb(&self) -> RGBColor {
        self.rgb.unwrap_or_else(|| {
            match self.cmyk {
                Some(cmyk) => cmyk.estimate_rgb(),
                None => RGBColor::default()
            }
        })
    }

    fn as_rgba(&self) -> RGBAColor {
        self.to_rgba()
    }

    fn as_cmyk(&self) -> CMYKColor {
        self.cmyk.unwrap_or_else(|| {
            match self.rgb {
                Some(rgb) => rgb.estimate_cmyk(),
                None => CMYKColor::default()
            }
        })
    }
}