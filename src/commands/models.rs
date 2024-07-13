use crate::calculator::models::Model;

pub fn models_command() {
    println!("Supported TI calculator models:");
    Model::display_models();
}
