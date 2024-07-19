pub mod calculator;
pub mod cli;
pub mod commands;
pub mod styles;
#[cfg(test)]
pub mod tests;
pub mod tokens;

use cli::{Arg, Cli, Command};
use commands::decode::decode_command;
use commands::details::details_command;
use commands::edit::archive::archive_command;
use commands::edit::comment::comment_command;
use commands::edit::lock::lock_command;
use commands::edit::rename::rename_command;
use commands::edit::unarchive::unarchive_command;
use commands::edit::unlock::unlock_command;
use commands::encode::encode_command;
use commands::models::models_command;

fn main() {
    let cli = Cli::new().with_default_command("help").with_commands(vec![
        Command::new("help", "Prints help information")
            .with_arg(
                Arg::new()
                .with_name("command")
                .with_value_name("COMMAND")
                .with_help("A command to help with"),
            ),
        Command::new("version", "Prints version information"),
        Command::new("decode", "Converts 8xp/82p/83p to txt")
            .with_arg(
                Arg::new()
                .with_name("input")
                .with_value_name("INPUT")
                .with_help("The input path to an 8xp/82p/83p file")
            )
            .with_arg(
                Arg::new()
                    .with_name("output")
                    .with_long("output")
                    .with_short('o')
                    .with_value_name("OUTPUT")
                    .with_help("The output path to a txt file"),
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
                    .with_name("content")
                    .with_long("content")
                    .with_short('c')
                    .with_help("Display the content of the input file"),
            )
            .with_arg(
                Arg::new()
                    .with_name("preview")
                    .with_long("preview")
                    .with_short('p')
                    .with_help("Preview the output file in the terminal"),
            ),
        Command::new("encode", "Converts txt to ")
            .with_arg(
                Arg::new()
                .with_name("input")
                .with_value_name("INPUT")
                .with_help("The input path to an 8xp/82p/83p file")
            )
            .with_arg(
                Arg::new()
                    .with_name("output")
                    .with_long("output")
                    .with_short('o')
                    .with_value_name("OUTPUT")
                    .with_help("The output path to a 8xp/82p/83p file"),
            )
            .with_arg(
                Arg::new()
                    .with_name("encode-mode")
                    .with_long("encode-mode")
                    .with_short('e')
                    .with_value_name("ENCODE_MODE")
                    .with_help("The mode used to parse tokens [min, max, smart] | Default: smart"),
            )
            .with_arg(
                Arg::new()
                    .with_name("content")
                    .with_long("content")
                    .with_short('c')
                    .with_help("Display the content of the input file"),
            )
            .with_arg(
                Arg::new()
                    .with_name("preview")
                    .with_long("preview")
                    .with_short('p')
                    .with_help("Preview the output file in the terminal"),
            ),
        Command::new("rename", "Renames the program name in a 8xp/82p/83p file")
            .with_arg(
                Arg::new()
                .with_name("input")
                .with_value_name("INPUT")
                .with_help("The input path to an 8xp/82p/83p file")
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
                    .with_value_name("NEW_FILE")
                    .with_help("Save the renamed program to a new file"),
            )
            .with_arg(
                Arg::new()
                    .with_name("delete-old")
                    .with_long("delete-old")
                    .with_short('d')
                    .with_help("Delete the old file"),
            ),
        Command::new("comment", "Write a custom comment to an 8xp/82p/83p file")
            .with_arg(
                Arg::new()
                .with_name("input")
                .with_value_name("INPUT")
                .with_help("The input path to an 8xp/82p/83p file")
            )
            .with_arg(
                Arg::new()
                    .with_name("comment")
                    .with_long("comment")
                    .with_short('c')
                    .with_value_name("COMMENT")
                    .with_help("New program comment (42 or less characters)"),
            )
            .with_arg(
                Arg::new()
                    .with_name("new-file")
                    .with_long("new-file")
                    .with_short('f')
                    .with_value_name("NEW_FILE")
                    .with_help("Save the program with a new comment to a new file"),
            )
            .with_arg(
                Arg::new()
                    .with_name("delete-old")
                    .with_long("delete-old")
                    .with_short('d')
                    .with_help("Delete the old file"),
            ),
        Command::new("lock", "Lock an 8xp/82p/83p file")
            .with_arg(
                Arg::new()
                .with_name("input")
                .with_value_name("INPUT")
                .with_help("The input path to an 8xp/82p/83p file")
            )
            .with_arg(
                Arg::new()
                    .with_name("new-file")
                    .with_long("new-file")
                    .with_short('f')
                    .with_value_name("NEW_FILE")
                    .with_help("Save the locked program to a new file"),
            )
            .with_arg(
                Arg::new()
                    .with_name("delete-old")
                    .with_long("delete-old")
                    .with_short('d')
                    .with_help("Delete the old file"),
            ),
        Command::new("unlock", "unlock an 8xp/82p/83p file")
            .with_arg(
                Arg::new()
                .with_name("input")
                .with_value_name("INPUT")
                .with_help("The input path to an 8xp/82p/83p file")
            )
            .with_arg(
                Arg::new()
                    .with_name("new-file")
                    .with_long("new-file")
                    .with_short('f')
                    .with_value_name("NEW_FILE")
                    .with_help("Save the unlocked program to a new file"),
            )
            .with_arg(
                Arg::new()
                    .with_name("delete-old")
                    .with_long("delete-old")
                    .with_short('d')
                    .with_help("Delete the old file"),
            ),
        Command::new("archive", "Set the program to be sent to Archive")
            .with_arg(
                Arg::new()
                .with_name("input")
                .with_value_name("INPUT")
                .with_help("The input path to an 8xp/82p/83p file")
            )
            .with_arg(
                Arg::new()
                    .with_name("new-file")
                    .with_long("new-file")
                    .with_short('f')
                    .with_value_name("NEW_FILE")
                    .with_help("Save the archived program to a new file"),
            )
            .with_arg(
                Arg::new()
                    .with_name("delete-old")
                    .with_long("delete-old")
                    .with_short('d')
                    .with_help("Delete the old file"),
            ),
        Command::new("unarchive", "Set the program to be sent to RAM")
            .with_arg(
                Arg::new()
                .with_name("input")
                .with_value_name("INPUT")
                .with_help("The input path to an 8xp/82p/83p file")
            )
            .with_arg(
                Arg::new()
                    .with_name("new-file")
                    .with_long("new-file")
                    .with_short('f')
                    .with_value_name("NEW_FILE")
                    .with_help("Save the un-archived program to a new file"),
            )
            .with_arg(
                Arg::new()
                    .with_name("delete-old")
                    .with_long("delete-old")
                    .with_short('d')
                    .with_help("Delete the old file"),
            ),
        Command::new("details", "Displays information about an 8xp/82p/83p file")
            .with_arg(
                Arg::new()
                .with_name("input")
                .with_value_name("INPUT")
                .with_help("The input path to an 8xp/82p/83p file")
            ),
        Command::new("models", "Prints the supported TI calculator models"),
    ]);

    let command = cli.match_commands();

    match command.name.as_str() {
        "help" => {
            let command = command.get_value().to_option();
            cli.help(command)
        }
        "version" => cli.version(),
        "decode" => {
            let input_path_string = command.get_value().throw_if_none();
            let output_path_string = command.get_value_of("output").to_option();
            let display_mode = command
                .get_value_of("display-mode")
                .to_option()
                .unwrap_or("accessible".to_string());
            let content = command.has("content");
            let preview = command.has("preview");

            if output_path_string.is_none() && !preview && !content {
                println!("No output path or preview option provided");
                std::process::exit(0);
            }

            decode_command(
                input_path_string,
                output_path_string,
                display_mode,
                content,
                preview,
            );
        }
        "encode" => {
            let input_path_string = command.get_value().throw_if_none();
            let output_path_string = command.get_value_of("output").to_option();
            let encode_mode = command
                .get_value_of("encode-mode")
                .to_option()
                .unwrap_or("smart".to_string());
            let content = command.has("content");
            let preview = command.has("preview");

            if output_path_string.is_none() && !preview && !content {
                println!("No output path or preview option provided");
                std::process::exit(0);
            }

            encode_command(
                input_path_string,
                output_path_string,
                encode_mode,
                content,
                preview,
            );
        }
        "rename" => {
            let input_path_string = command.get_value().throw_if_none();
            let name = command.get_value_of("name").throw_if_none();
            let new_file_path = command.get_value_of("new-file").to_option();
            let delete_old = command.has("delete-old");

            rename_command(input_path_string, name, new_file_path, delete_old);
        }
        "comment" => {
            let input_path_string = command.get_value().throw_if_none();
            let comment = command.get_value_of("comment").throw_if_none();
            let new_file_path = command.get_value_of("new-file").to_option();
            let delete_old = command.has("delete-old");

            comment_command(input_path_string, comment, new_file_path, delete_old);
        }
        "lock" => {
            let input_path_string = command.get_value().throw_if_none();
            let new_file_path = command.get_value_of("new-file").to_option();
            let delete_old = command.has("delete-old");

            lock_command(input_path_string, new_file_path, delete_old);
        }
        "unlock" => {
            let input_path_string = command.get_value().throw_if_none();
            let new_file_path = command.get_value_of("new-file").to_option();
            let delete_old = command.has("delete-old");

            unlock_command(input_path_string, new_file_path, delete_old);
        }
        "archive" => {
            let input_path_string = command.get_value().throw_if_none();
            let new_file_path = command.get_value_of("new-file").to_option();
            let delete_old = command.has("delete-old");

            archive_command(input_path_string, new_file_path, delete_old);
        }
        "unarchive" => {
            let input_path_string = command.get_value().throw_if_none();
            let new_file_path = command.get_value_of("new-file").to_option();
            let delete_old = command.has("delete-old");

            unarchive_command(input_path_string, new_file_path, delete_old);
        }
        "details" => {
            let input_path_string = command.get_value().throw_if_none();

            details_command(input_path_string)
        }
        "models" => models_command(),
        _ => cli.help(None),
    }
}
