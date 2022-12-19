// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// various log levels
pub fn log(level: LogLevel, message: &str) -> String {
    match level{
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
    }
}
pub fn info(message: &str) -> String {
    let mut string = String::from("[INFO]: {}");
    string.replace("{}", message)
}
pub fn warn(message: &str) -> String {
    let mut string = String::from("[WARNING]: {}");
    string.replace("{}", message)
}
pub fn error(message: &str) -> String {
    let mut string = String::from("[ERROR]: {}");
    string.replace("{}", message)
}