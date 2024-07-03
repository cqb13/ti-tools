pub mod cli;
pub mod commands;
pub mod program;
pub mod tokens;

use cli::{Arg, Cli, Command};
use commands::decode::decode_command;
use commands::models::models_command;
use commands::rename::rename_command;
use tokens::OsVersion;

use program::encode::encode;
use tokens::load_tokens;

fn main() {
    let cli = Cli::new().with_default_command("help").with_commands(vec![
        Command::new("help", "Prints help information"),
        Command::new("version", "Prints version information"),
        Command::new("decode", "Converts 8xp to txt")
            .with_arg(
                Arg::new()
                    .with_name("input")
                    .with_long("input")
                    .with_short('i')
                    .with_value_name("INPUT")
                    .with_help("The input path to an 8xp file"),
            )
            .with_arg(
                Arg::new()
                    .with_name("output")
                    .with_long("output")
                    .with_short('o')
                    .with_value_name("OUTPUT")
                    .with_help("The output path to a text file"),
            )
            .with_arg(
                Arg::new()
                    .with_name("display-mode")
                    .with_long("display-mode")
                    .with_short('d')
                    .with_value_name("DISPLAY_MODE")
                    .with_help("The characters to translate the tokens to [pretty, accessible, ti] | Default: accessible"),
            )
            .with_arg(
                Arg::new()
                    .with_name("model")
                    .with_long("model")
                    .with_short('m')
                    .with_value_name("MODEL")
                    .with_help("The model of calculator (use models command to see the supported models) | Default: latest"),
            )
            .with_arg(
                Arg::new()
                    .with_name("bytes")
                    .with_long("bytes")
                    .with_short('b')
                    .with_help("Display the bytes of the input file"),
            )
            .with_arg(
                Arg::new()
                    .with_name("preview")
                    .with_long("preview")
                    .with_short('p')
                    .with_help("Preview the output file in the terminal"),
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
        Command::new("test", "Prints the tokens"),
        Command::new("models", "Prints the supported TI calculator models"),
    ]);

    let command = cli.match_commands();

    match command.name {
        "help" => cli.help(),
        "version" => cli.version(),
        "decode" => {
            let input_path_string = command.get_value_of("input").throw_if_none();
            let output_path_string = command.get_value_of("output").to_option();
            let display_mode = command
                .get_value_of("display-mode")
                .to_option()
                .unwrap_or("accessible".to_string());
            let model = command
                .get_value_of("model")
                .to_option()
                .unwrap_or("latest".to_string());
            let bytes = command.has("bytes");
            let preview = command.has("preview");

            if output_path_string.is_none() && !preview {
                println!("No output path or preview option provided");
            }

            decode_command(
                input_path_string,
                output_path_string,
                display_mode,
                model,
                bytes,
                preview,
            );
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
        "test" => {
            let file =
                std::fs::read_to_string("./src/tests/tokens.txt").expect("failed to read file");

            let version = OsVersion {
                model: "latest".to_string(),
                version: "latest".to_string(),
            };

            let tokens = load_tokens(&version);

            let result = encode(file, &tokens, false, program::DisplayMode::Accessible);

            print_bytes(&result);
        }
        _ => cli.help(),
    }
}

fn print_bytes(file: &Vec<u8>) {
    let mut i = 0;
    for byte in file {
        print!("{:02X}", byte);
        i += 1;
        if i % 16 == 0 {
            println!();
        } else {
            print!(", ");
        }
    }
}
