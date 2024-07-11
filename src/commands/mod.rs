pub mod decode;
pub mod details;
pub mod edit;
pub mod encode;
pub mod models;

pub fn exit_with_error(message: &str) -> ! {
    eprintln!("Error: {}", message);
    std::process::exit(1);
}
