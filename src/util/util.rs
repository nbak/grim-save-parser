pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct CustomError {
    message: String,
}

impl CustomError {
    pub fn new(message: String) -> Box<Self> {
        Box::new(CustomError { message })
    }
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl std::error::Error for CustomError {}

pub fn ensure_eq<T>(a: T, b: T, message: String) -> Result<()>
where
    T: Eq,
{
    if a != b {
        Err(CustomError::new(message))
    } else {
        Ok(())
    }
}