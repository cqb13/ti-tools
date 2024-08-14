use crate::prints;

pub fn search_command(token: String, token_type: String) {
    if token_type != "accessible" && token_type != "pretty" && token_type != "byte" {
        prints!("[color:bright-red]Error:[color:reset] [color:bright-cyan]{}[color:reset] is an invalid token type", token_type);
        std::process::exit(1);
    }

    println!("token: {}", token)
}
