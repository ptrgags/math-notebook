use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct ColorRGB(pub u8, pub u8, pub u8);

impl Display for ColorRGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(r, g, b) = self;
        write!(f, "#{:02x}{:02x}{:02x}", r, g, b)
    }
}

#[derive(Clone, Copy)]
pub struct Style {
    pub stroke: Option<ColorRGB>,
    pub fill: Option<ColorRGB>,
    pub width_percent: Option<f64>,
}

impl Style {
    pub fn new() -> Self {
        Self {
            stroke: None,
            fill: None,
            width_percent: None,
        }
    }

    pub fn stroke(r: u8, g: u8, b: u8) -> Self {
        Self {
            stroke: Some(ColorRGB(r, g, b)),
            fill: None,
            width_percent: None,
        }
    }

    pub fn with_stroke(&self, r: u8, g: u8, b: u8) -> Self {
        Self {
            stroke: Some(ColorRGB(r, g, b)),
            fill: self.fill,
            width_percent: self.width_percent,
        }
    }

    pub fn fill(r: u8, g: u8, b: u8) -> Self {
        Self {
            stroke: None,
            fill: Some(ColorRGB(r, g, b)),
            width_percent: None,
        }
    }

    pub fn with_fill(&self, r: u8, g: u8, b: u8) -> Self {
        Self {
            stroke: self.stroke,
            fill: Some(ColorRGB(r, g, b)),
            width_percent: self.width_percent,
        }
    }

    pub fn with_width(&self, width: f64) -> Self {
        Self {
            stroke: self.stroke,
            fill: self.fill,
            width_percent: Some(width),
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}
