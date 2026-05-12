use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoxError {
    #[error("syntax error: {0}")]
    Syntax(String),
    #[error("runtime error: {0}")]
    Runtime(String),
}

pub type Result<T> = std::result::Result<T, LoxError>;

pub fn run(source: &str) -> Result<()> {
    let _ = source;
    todo!("implement scanner -> parser -> interpreter pipeline")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lox_error_displays_syntax_kind() {
        let err = LoxError::Syntax("unexpected token".into());
        assert_eq!(err.to_string(), "syntax error: unexpected token");
    }
}
