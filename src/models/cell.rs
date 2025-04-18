//! Cell data structures for the grid

/// A cell with two characters (for the main grid)
#[derive(Debug, Clone)]
pub struct CellPlural {
    pub combo: String,
    pub first: char,
    pub last: char,
}

impl CellPlural {
    /// Create a new empty cell
    pub fn new() -> Self {
        Self {
            combo: String::new(),
            first: char::default(),
            last: char::default(),
        }
    }

    /// Create a cell with the given characters
    pub fn with_chars(first: char, last: char) -> Self {
        Self {
            combo: format!("{}{}", first, last),
            first,
            last,
        }
    }
}

/// A cell with a single character (for the micro grid)
#[derive(Debug, Clone, Copy)]
pub struct CellSingular {
    pub unit: char,
}

/// Represents the currently focused cell
#[derive(Debug, Clone, Copy)]
pub struct FocusedCell {
    pub first: char,
    pub last: char,
    pub conclusion: char,
}

impl FocusedCell {
    /// Create a new empty focused cell
    pub fn new() -> Self {
        Self {
            first: char::default(),
            last: char::default(),
            conclusion: char::default(),
        }
    }

    /// Check if the first character is selected
    pub fn has_first(&self) -> bool {
        self.first != char::default()
    }

    /// Check if the last character is selected
    pub fn has_last(&self) -> bool {
        self.last != char::default()
    }

    /// Check if the conclusion character is selected
    pub fn has_conclusion(&self) -> bool {
        self.conclusion != char::default()
    }

    /// Check if this cell is fully selected
    pub fn is_complete(&self) -> bool {
        self.has_first() && self.has_last()
    }

    /// Reset the cell
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for FocusedCell {
    fn default() -> Self {
        Self::new()
    }
}
