use thiserror::Error;

/// Errors that can occur in the chorus crate
#[derive(Error, Debug)]
pub enum Error {
    // Bad hex input
    #[error("Bad hex input")]
    BadHexInput,

    // Output buffer too small
    #[error("Output buffer too small")]
    BufferTooSmall,

    // Config
    #[error("Config: {0}")]
    Config(#[from] ron::error::SpannedError),

    // End of Input
    #[error("End of input")]
    EndOfInput,

    // I/O Error
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),

    // JSON Bad Character
    #[error("JSON bad character: {0} at position {1}, {2} was expected")]
    JsonBadCharacter(char, usize, char),

    // JSON Bad (general)
    #[error("JSON bad: {0} at position {1}")]
    JsonBad(&'static str, usize),

    // JSON Bad Event
    #[error("JSON bad event: {0} at position {1}")]
    JsonBadEvent(&'static str, usize),

    // JSON Bad String Character
    #[error("JSON string bad character: codepoint {0}")]
    JsonBadStringChar(u32),

    // JSON Escape
    #[error("JSON string escape error")]
    JsonEscape,

    // JSON Escape Surrogate
    #[error("JSON string escape surrogate (ancient style) is not supported")]
    JsonEscapeSurrogate,

    // UTF-8
    #[error("UTF-8: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    // UTF-8
    #[error("UTF-8 error")]
    Utf8Error,
}
