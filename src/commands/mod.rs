pub mod archive;
pub mod comment;
pub mod decode;
pub mod details;
pub mod encode;
pub mod lock;
pub mod models;
pub mod rename;
pub mod unarchive;
pub mod unlock;

pub fn exit_with_error(message: &str) -> ! {
    eprintln!("Error: {}", message);
    std::process::exit(1);
}
