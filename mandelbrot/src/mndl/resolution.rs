use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ResolutionError;

impl std::fmt::Display for ResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Invalid resolution string - must be in the format: WIDTHxHEIGHT"
        )
    }
}

#[derive(Debug, Clone)]
pub struct Resolution {
    width: u32,
    height: u32,
}

impl FromStr for Resolution {
    type Err = ResolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find('x') {
            Some(p) => match (u32::from_str(&s[..p]), u32::from_str(&s[p + 1..])) {
                (Ok(width), Ok(height)) => Ok(Resolution::new(width, height)),
                _ => Err(ResolutionError {}),
            },
            None => Err(ResolutionError {}),
        }
    }
}

impl Resolution {
    /// Create a new Resolution with the specified width and height
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Get an immutable ref to the width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get an immutable ref to the height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get the total pixel count
    pub fn pixel_count(&self) -> usize {
        self.width as usize * self.height as usize
    }
}
