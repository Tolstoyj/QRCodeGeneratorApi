use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrColors {
    #[serde(default = "default_foreground")]
    pub foreground: String,
    #[serde(default = "default_background")]
    pub background: String,
}

impl Default for QrColors {
    fn default() -> Self {
        Self {
            foreground: default_foreground(),
            background: default_background(),
        }
    }
}

impl QrColors {
    pub fn new(foreground: String, background: String) -> Result<Self, String> {
        let colors = Self {
            foreground,
            background,
        };
        colors.validate()?;
        Ok(colors)
    }

    pub fn validate(&self) -> Result<(), String> {
        self.validate_color(&self.foreground, "foreground")?;
        self.validate_color(&self.background, "background")?;
        
        if self.foreground == self.background {
            return Err("Foreground and background colors cannot be the same".to_string());
        }
        
        Ok(())
    }

    fn validate_color(&self, color: &str, color_type: &str) -> Result<(), String> {
        if !color.starts_with('#') {
            return Err(format!("{} color must start with '#'", color_type));
        }

        let hex_part = &color[1..];
        if hex_part.len() != 6 {
            return Err(format!("{} color must be 6 hex digits (e.g., #FF0000)", color_type));
        }

        if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(format!("{} color contains invalid hex characters", color_type));
        }

        Ok(())
    }

    pub fn foreground_rgb(&self) -> Result<(u8, u8, u8), String> {
        self.hex_to_rgb(&self.foreground)
    }

    pub fn background_rgb(&self) -> Result<(u8, u8, u8), String> {
        self.hex_to_rgb(&self.background)
    }

    fn hex_to_rgb(&self, hex: &str) -> Result<(u8, u8, u8), String> {
        let hex = &hex[1..]; // Remove #
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| "Invalid red component".to_string())?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| "Invalid green component".to_string())?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| "Invalid blue component".to_string())?;
        Ok((r, g, b))
    }

    pub fn contrast_ratio(&self) -> Result<f64, String> {
        let (r1, g1, b1) = self.foreground_rgb()?;
        let (r2, g2, b2) = self.background_rgb()?;

        let l1 = relative_luminance(r1, g1, b1);
        let l2 = relative_luminance(r2, g2, b2);

        let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
        Ok((lighter + 0.05) / (darker + 0.05))
    }

    pub fn has_sufficient_contrast(&self) -> Result<bool, String> {
        // WCAG AA standard requires contrast ratio of at least 3:1 for graphics
        Ok(self.contrast_ratio()? >= 3.0)
    }
}

fn default_foreground() -> String {
    "#000000".to_string()
}

fn default_background() -> String {
    "#FFFFFF".to_string()
}

fn relative_luminance(r: u8, g: u8, b: u8) -> f64 {
    fn linearize(c: u8) -> f64 {
        let c = c as f64 / 255.0;
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }

    0.2126 * linearize(r) + 0.7152 * linearize(g) + 0.0722 * linearize(b)
}

impl fmt::Display for QrColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fg: {}, bg: {}", self.foreground, self.background)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_colors() {
        let colors = QrColors::default();
        assert_eq!(colors.foreground, "#000000");
        assert_eq!(colors.background, "#FFFFFF");
    }

    #[test]
    fn test_color_validation_success() {
        let colors = QrColors::new("#FF0000".to_string(), "#00FF00".to_string());
        assert!(colors.is_ok());
    }

    #[test]
    fn test_color_validation_failure() {
        // Missing #
        assert!(QrColors::new("FF0000".to_string(), "#00FF00".to_string()).is_err());
        
        // Wrong length
        assert!(QrColors::new("#FF00".to_string(), "#00FF00".to_string()).is_err());
        
        // Invalid hex
        assert!(QrColors::new("#GGGGGG".to_string(), "#00FF00".to_string()).is_err());
        
        // Same colors
        assert!(QrColors::new("#FF0000".to_string(), "#FF0000".to_string()).is_err());
    }

    #[test]
    fn test_rgb_conversion() {
        let colors = QrColors::new("#FF0000".to_string(), "#00FF00".to_string()).unwrap();
        assert_eq!(colors.foreground_rgb().unwrap(), (255, 0, 0));
        assert_eq!(colors.background_rgb().unwrap(), (0, 255, 0));
    }

    #[test]
    fn test_contrast_ratio() {
        // Black on white should have high contrast
        let colors = QrColors::default();
        let ratio = colors.contrast_ratio().unwrap();
        assert!(ratio > 20.0); // Should be around 21:1
        assert!(colors.has_sufficient_contrast().unwrap());
    }

    #[test]
    fn test_low_contrast() {
        // Light gray on white should have low contrast
        let colors = QrColors::new("#CCCCCC".to_string(), "#FFFFFF".to_string()).unwrap();
        let ratio = colors.contrast_ratio().unwrap();
        assert!(ratio < 3.0);
        assert!(!colors.has_sufficient_contrast().unwrap());
    }
}