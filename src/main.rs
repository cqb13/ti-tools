pub mod cli;
pub mod commands;
pub mod tokens;

use cli::{Arg, Cli, Command};
use commands::convert::convert_command;

fn main() {
    let cli = Cli::new().with_default_command("help").with_commands(vec![
        Command::new("help", "Prints help information"),
        Command::new("version", "Prints version information"),
        Command::new("convert", "Converts between 8xp and text")
            .with_arg(
                Arg::new()
                    .with_name("input")
                    .with_long("input")
                    .with_short('i')
                    .with_value_name("INPUT")
                    .with_help("The input file path, either 8xp or txt"),
            )
            .with_arg(
                Arg::new()
                    .with_name("output")
                    .with_long("output")
                    .with_short('o')
                    .with_value_name("OUTPUT")
                    .with_help("The output file path"),
            )
            .with_arg(
                Arg::new()
                    .with_name("bytes")
                    .with_long("bytes")
                    .with_short('b')
                    .with_help("Shows byte values of the input file"),
            )
            .with_arg(
                Arg::new()
                    .with_name("display")
                    .with_long("display")
                    .with_short('d')
                    .with_help("Displays the output in the terminal"),
            )
            .with_arg(
                Arg::new()
                    .with_name("log")
                    .with_long("log")
                    .with_short('l')
                    .with_help("Shows log messages"),
            ),
    ]);

    let command = cli.match_commands();

    match command.name {
        "help" => cli.help(),
        "version" => cli.version(),
        "convert" => {
            let input_path_string = command.get_value_of("input").throw_if_none();
            let output_path_string = command.get_value_of("output").throw_if_none();
            let bytes = command.has("bytes");
            let display = command.has("display");
            let log_messages = command.has("log");

            convert_command(input_path_string, output_path_string, bytes, display, log_messages);
        }
        _ => cli.help(),
    }
}
