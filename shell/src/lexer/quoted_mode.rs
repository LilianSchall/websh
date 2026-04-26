pub enum QuotedMode {
    None,
    Single,
    Double,
    Backslash
}

impl QuotedMode {
    pub fn is_quoted(&self) -> bool {
        match self {
            QuotedMode::None => false,
            _ => true
        }
    }
}
