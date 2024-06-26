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
                    .with_name("header")
                    .with_long("header")
                    .with_short('h')
                    .with_help("Include header information"),
            )
            .with_arg(
                Arg::new()
                    .with_name("metadata")
                    .with_long("metadata")
                    .with_short('m')
                    .with_help("Include metadata information"),
            )
            .with_arg(
                Arg::new()
                    .with_name("checksum")
                    .with_long("checksum")
                    .with_short('c')
                    .with_help("Include checksum information"),
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
            let output_path_string = command.get_value_of("output").to_option();
            let header = command.has("header");
            let metadata = command.has("metadata");
            let checksum = command.has("checksum");
            let bytes = command.has("bytes");
            let display = command.has("display");
            let log_messages = command.has("log");

            if output_path_string.is_none() && !display {
                println!("You must specify at least one output method");
            }

            convert_command(
                input_path_string,
                output_path_string,
                header,
                metadata,
                checksum,
                bytes,
                display,
                log_messages,
            );
        }
        _ => cli.help(),
    }
}
