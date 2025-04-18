//! Margin data structures

/// Represents margins for UI elements
#[derive(Debug, Clone, Copy)]
pub struct Margin {
    pub top: i32,
    pub left: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Margin {
    /// Create a new margin with all sides set to the same value
    pub fn all(value: i32) -> Self {
        Self {
            top: value,
            left: value,
            right: value,
            bottom: value,
        }
    }

    /// Convert to egui Margin
    pub fn to_egui(&self) -> eframe::egui::Margin {
        eframe::egui::Margin {
            top: self.top as i8,
            left: self.left as i8,
            right: self.right as i8,
            bottom: self.bottom as i8,
        }
    }

    /// Zero margin
    pub const ZERO: Self = Self {
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
    };
}

impl Default for Margin {
    fn default() -> Self {
        Self::ZERO
    }
}
