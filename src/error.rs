#[derive(Debug)]
pub enum ParseError {
    PeekTokenError { msg: String },
}

impl std::error::Error for ParseError {}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::PeekTokenError { msg } => write!(f, "{}", msg),
        }
    }
}
