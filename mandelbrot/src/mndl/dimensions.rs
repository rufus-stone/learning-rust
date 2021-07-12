use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct DimensionsError;

impl std::fmt::Display for DimensionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Invalid resolution string - must be in the format: WIDTHxHEIGHT"
        )
    }
}

#[derive(Debug, Clone)]
pub struct Dimensions {
    width: usize,
    height: usize,
}

impl FromStr for Dimensions {
    type Err = DimensionsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find('x') {
            Some(p) => match (usize::from_str(&s[..p]), usize::from_str(&s[p + 1..])) {
                (Ok(width), Ok(height)) => Ok(Dimensions::new(width, height)),
                _ => Err(DimensionsError {}),
            },
            None => Err(DimensionsError {}),
        }
    }
}

impl Dimensions {
    /// Create a new Dimensions with the specified width and height
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    /// Get an immutable ref to the width
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get an immutable ref to the height
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get the total pixel count
    pub fn pixel_count(&self) -> usize {
        self.width as usize * self.height as usize
    }
}
