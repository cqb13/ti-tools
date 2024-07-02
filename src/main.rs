pub mod cli;
pub mod commands;
pub mod program;
pub mod tokens;
use std::path::Path;

use cli::{Arg, Cli, Command};
use commands::convert::convert_command;
use commands::models::models_command;
use commands::rename::rename_command;
use program::Program;
use tokens::OsVersion;

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
                    .with_name("name")
                    .with_long("name")
                    .with_short('n')
                    .with_value_name("NAME")
                    .with_help("Name of the program, only used when converting to 8xp"),
            )
            .with_arg(
                Arg::new()
                    .with_name("raw")
                    .with_long("raw")
                    .with_short('r')
                    .with_help("Display the raw content of the input file before conversion"),
            )
            .with_arg(
                Arg::new()
                    .with_name("display")
                    .with_long("display")
                    .with_short('d')
                    .with_help("Displays the output"),
            )
            .with_arg(
                Arg::new()
                    .with_name("log")
                    .with_long("log")
                    .with_short('l')
                    .with_help("Shows log messages"),
            ),
        Command::new("rename", "Renames the program name in a 8xp file")
            .with_arg(
                Arg::new()
                    .with_name("input")
                    .with_long("input")
                    .with_short('i')
                    .with_value_name("INPUT")
                    .with_help("File path to the 8xp file"),
            )
            .with_arg(
                Arg::new()
                    .with_name("name")
                    .with_long("name")
                    .with_short('n')
                    .with_value_name("NAME")
                    .with_help("New program number (8 or less uppercase alphabetic characters)"),
            )
            .with_arg(
                Arg::new()
                    .with_name("new-file")
                    .with_long("new-file")
                    .with_short('f')
                    .with_help("Create a new file with the same name as the program"),
            )
            .with_arg(
                Arg::new()
                    .with_name("delete-old")
                    .with_long("delete-old")
                    .with_short('d')
                    .with_help("Delete the old file"),
            ),
        Command::new("tokens", "Prints the tokens"),
        Command::new("models", "Prints the supported TI calculator models"),
    ]);

    let command = cli.match_commands();

    match command.name {
        "help" => cli.help(),
        "version" => cli.version(),
        "convert" => {
            let input_path_string = command.get_value_of("input").throw_if_none();
            let output_path_string = command.get_value_of("output").to_option();
            let name = command.get_value_of("name").to_option();
            let raw = command.has("raw");
            let display = command.has("display");

            if output_path_string.is_none() && !display {
                println!("You must specify at least one output method");
            }

            convert_command(input_path_string, output_path_string, name, raw, display);
        }
        "rename" => {
            let input_path_string = command.get_value_of("input").throw_if_none();
            let name = command.get_value_of("name").throw_if_none();
            let new_file = command.has("new-file");
            let delete_old = command.has("delete-old");

            if delete_old && !new_file {
                println!("Wont delete the old file if a new one is not created")
            }

            rename_command(input_path_string, name, new_file, delete_old);
        }
        "models" => models_command(),
        "tokens" => {
            let version = OsVersion {
                model: "TI-84+CE".to_string(),
                version: "4.0".to_string(),
            };

            let program =
                Program::load(Path::new("./src/tests/ALLTOKS.8xp").to_path_buf(), version);

            let program = match program {
                Ok(program) => program,
                Err(err) => {
                    println!("{}", err);
                    std::process::exit(0);
                }
            };

            println!("{}", program.display());
            println!();

            program
                .save_to("./src/tests/tokens.txt")
                .expect("failed to save");
        }
        _ => cli.help(),
    }
}
