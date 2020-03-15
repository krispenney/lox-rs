use std::fmt;

#[derive(Debug)]
pub struct LoxErr {
    line: usize,
    message: String,
}

impl LoxErr {
    pub fn new(line: usize, message: String) -> LoxErr {
        LoxErr {
            line: line,
            message: message,
        }
    }

    pub fn display_message(&self) -> String {
        format!("[Line {}] Error: {}", self.line, self.message)
    }
}

impl fmt::Display for LoxErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_message())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let error = LoxErr::new(11, String::from("testing..."));
        let expected_err = LoxErr {
            line: 11,
            message: String::from("testing..."),
        };

        assert_eq!(error.line, expected_err.line);
        assert_eq!(error.message, expected_err.message);
    }

    #[test]
    fn display_message() {
        let error = LoxErr::new(11, String::from("testing..."));
        let expected_message = String::from("[Line 11] Error: testing...");
        assert_eq!(error.display_message(), expected_message);
    }
}
