pub mod comment;
pub mod decode;
pub mod details;
pub mod encode;
pub mod models;
pub mod rename;

pub fn exit_with_error(message: &str) -> ! {
    eprintln!("Error: {}", message);
    std::process::exit(1);
}
