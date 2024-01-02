//! This module contains the configuration options for the application.
//! 


pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains the configuration options for controlling logging.
/// # Examples
/// ```
/// use rust-proj-5::config::Logging;
/// let config = Logging::new();
/// ```
/// 
/// Create a new 'Logging" struct with custom values
/// ```
/// use rust-proj-5::config::Logging;
/// let config = Logging {
///     enabled: true,
///     level: LogLevel::Info,
///     destination: LogOutput::File("log.txt".to_string()),
/// }
/// ```


// NEED TO HAVE pub BEFORE THE PARTS OF THE STRUCT ( OR ENUM ) THAT ARE TO BE PUBLIC
// NO pub MAKES IT AUTOMATICALLY PRIVATE
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: true,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    
    }

}