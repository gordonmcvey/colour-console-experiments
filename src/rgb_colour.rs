pub struct RgbColour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RgbColour {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn from_int(colour: u32) -> Self {
        Self::new((colour >> 16) as u8, (colour >> 8) as u8, colour as u8)
    }

    pub fn as_int(&self) -> u32 {
        ((self.red as u32) << 16) + ((self.green as u32) << 8) + (self.blue as u32)
    }

    pub fn gradient(&self, end: &RgbColour, steps: u32) -> Vec<RgbColour> {
        (0..=steps)
            .map(|step| {
                let ratio = step as f64 / steps as f64;
                Self::new(
                    Self::lerp(self.red, end.red, ratio),
                    Self::lerp(self.green, end.green, ratio),
                    Self::lerp(self.blue, end.blue, ratio),
                )
            })
            .collect()
    }

    // Linear interpolation between start and end values
    fn lerp(start: u8, end: u8, ratio: f64) -> u8 {
        (start as f64 + ratio * (end as f64 - start as f64)).round() as u8
    }
}
