pub mod calculator;
pub mod cli;
pub mod commands;
pub mod styles;

use cli::{Arg, Cli, CmdOption, Command};
use commands::convert::convert_command;
use commands::details::details_command;
use commands::edit::archive::archive_command;
use commands::edit::comment::comment_command;
use commands::edit::lock::lock_command;
use commands::edit::rename::rename_command;
use commands::edit::unarchive::unarchive_command;
use commands::edit::unlock::unlock_command;
use commands::models::models_command;
use commands::search::search_command;

fn main() {
    let cli = Cli::new()
        .with_command(
            Command::new("help", "Prints help information")
                .with_option(
                    CmdOption::new("command", "COMMAND", "The command you want help with")
                        .optional()
                )
        )
        .with_command(
            Command::new("version", "Prints version information")
        )
        .with_command(
            Command::new("convert", "Converts between 8xp/83p/82p, json, and txt")
                .with_option(
                    CmdOption::new("input", "INPUT", "The input path to an 8xp, 83p, 82p, json or txt file")
                )
                .with_arg(
                    Arg::new("output", "The output path to an 8xp, 83p, json, or txt file", "output", 'o')
                        .with_value_name("OUTPUT"),
                )
                .with_arg(
                    Arg::new("display-mode", "The characters to translate the tokens to [pretty, accessible, ti] | Default: accessible", "display-mode", 'd')
                        .with_default_value("accessible")
                        .with_value_name("DISPLAY_MODE")
                )
                .with_arg(
                    Arg::new("encode-mode", "The mode used to parse tokens [min, max, smart] | Default: smart", "encode-mode", 'e')
                        .with_default_value("smart")
                        .with_value_name("ENCODE_MODE")
                )
                .with_arg(
                    Arg::new("content", "Display the content of the input file", "content", 'c')
                )
                .with_arg(
                    Arg::new("preview", "Display the decoded output", "preview", 'p')
                )
                .with_arg(
                    Arg::new("mass", "Changes input required from file to directory for mass file decoding", "mass", 'm')
                )
        )
        .with_command(
            Command::new("search", "Retrieves a description for a token")
            .with_option(
                CmdOption::new("token", "TOKEN", "The token to search for")
            )
            .with_arg(
                Arg::new("type", "The type of token to search for [accessible, pretty, byte] | Default: accessible", "type", 't')
                    .with_value_name("TYPE")
                    .with_default_value("accessible")
            )
        )
        .with_command(
            Command::new("rename", "Renames the program name in a 8xp/82p/83p file")
            .with_option(
                CmdOption::new("input", "INPUT", "The input path to an 8xp/82p/83p file")
            )
            .with_arg(
                Arg::new("name", "The new program name (8 or less alphabetic characters)", "name", 'n')
                    .with_value_name("NAME")
            )
            .with_arg(
                Arg::new("new-file", "Save the renamed program to a new file", "new-file", 'f')
                    .with_value_name("NEW_FILE")
            )
            .with_arg(
                Arg::new("delete-old", "Delete the old file", "delete-old", 'd')
                    .requires("new-file")
            )
        )
        .with_command(
            Command::new("comment", "Write a custom comment to an 8xp/82p/83p file")
            .with_option(
                CmdOption::new("input", "INPUT", "The input path to an 8xp/82p/83p file")
            )
            .with_arg(
                Arg::new("comment", "The new program comment (42 or less characters)", "comment", 'c')
                    .with_value_name("COMMENT")
            )
            .with_arg(
                Arg::new("new-file", "Save the program with the updated comment to a new file", "new-file", 'f')
                    .with_value_name("NEW_FILE")
            )
            .with_arg(
                Arg::new("delete-old", "Delete the old file", "delete-old", 'd')
                    .requires("new-file")
            )
        )
        .with_command(
            Command::new("lock", "Lock an 8xp/82p/83p file")
            .with_option(
                CmdOption::new("input", "INPUT", "The input path to an 8xp/82p/83p file")
            )
            .with_arg(
                Arg::new("new-file", "Save the locked program to a new file", "new-file", 'f')
                    .with_value_name("NEW_FILE")
            )
            .with_arg(
                Arg::new("delete-old", "Delete the old file", "delete-old", 'd')
                    .requires("new-file")
            )
        )
        .with_command(
            Command::new("unlock", "unlock an 8xp/82p/83p file")
            .with_option(
                CmdOption::new("input", "INPUT", "The input path to an 8xp/82p/83p file")
            )
            .with_arg(
                Arg::new("new-file", "Save the unlocked program to a new file", "new-file", 'f')
                    .with_value_name("NEW_FILE")
            )
            .with_arg(
                Arg::new("delete-old", "Delete the old file", "delete-old", 'd')
                    .requires("new-file")
            )
        )
        .with_command(
            Command::new("archive", "Set the program to be sent to Archive")
            .with_option(
                CmdOption::new("input", "INPUT", "The input path to an 8xp/82p/83p file")
            )
            .with_arg(
                Arg::new("new-file", "Save the archived program to a new file", "new-file", 'f')
                    .with_value_name("NEW_FILE")
            )
            .with_arg(
                Arg::new("delete-old", "Delete the old file", "delete-old", 'd')
                    .requires("new-file")
            )
        )
        .with_command(
            Command::new("unarchive", "Set the program to be sent to RAM")
            .with_option(
                CmdOption::new("input", "INPUT", "The input path to an 8xp/82p/83p file")
            )
            .with_arg(
                Arg::new("new-file", "Save the un-archived program to a new file", "new-file", 'f')
                    .with_value_name("NEW_FILE")
            )
            .with_arg(
                Arg::new("delete-old", "Delete the old file", "delete-old", 'd')
                    .requires("new-file")
            )
        )
        .with_command(
            Command::new("details", "Displays information about an 8xp/82p/83p file")
            .with_option(
                CmdOption::new("input", "INPUT", "The input path to an 8xp/82p/83p file")
            )
        )
        .with_command(
            Command::new("models", "Prints the supported TI calculator models")
        );

    let command = cli.match_commands();

    match command.name.as_str() {
        "help" => {
            let command = command.get_option("command").to_option();
            cli.help(command.as_deref())
        }
        "version" => cli.version(),
        "convert" => {
            let input_path_string = command.get_option("input").throw_if_none();
            let output_path_string = command.get_arg("output").to_option();
            let display_mode = command.get_arg("display-mode").throw_if_none();
            let encode_mode = command.get_arg("encode-mode").throw_if_none();
            let content = command.has("content");
            let preview = command.has("preview");
            let mass = command.has("mass");
            
            convert_command(
                input_path_string,
                output_path_string,
                display_mode,
                encode_mode,
                content,
                preview,
                mass,
            );
        }
        "search" => {
            let token = command.get_option("token").throw_if_none();
            let token_type = command.get_arg("type").throw_if_none();
    
            search_command(token, token_type)
        }
        "rename" => {
            let input_path_string = command.get_option("input").throw_if_none();
            let name = command.get_arg("name").throw_if_none();
            let new_file_path = command.get_arg("new-file").to_option();
            let delete_old = command.has("delete-old");

            rename_command(input_path_string, name, new_file_path, delete_old);
        }
        "comment" => {
            let input_path_string = command.get_option("input").throw_if_none();
            let comment = command.get_arg("comment").throw_if_none();
            let new_file_path = command.get_arg("new-file").to_option();
            let delete_old = command.has("delete-old");

            comment_command(input_path_string, comment, new_file_path, delete_old);
        }
        "lock" => {
            let input_path_string = command.get_option("input").throw_if_none();
            let new_file_path = command.get_arg("new-file").to_option();
            let delete_old = command.has("delete-old");

            lock_command(input_path_string, new_file_path, delete_old);
        }
        "unlock" => {
            let input_path_string = command.get_option("input").throw_if_none();
            let new_file_path = command.get_arg("new-file").to_option();
            let delete_old = command.has("delete-old");

            unlock_command(input_path_string, new_file_path, delete_old);
        }
        "archive" => {
            let input_path_string = command.get_option("input").throw_if_none();
            let new_file_path = command.get_arg("new-file").to_option();
            let delete_old = command.has("delete-old");

            archive_command(input_path_string, new_file_path, delete_old);
        }
        "unarchive" => {
            let input_path_string = command.get_option("input").throw_if_none();
            let new_file_path = command.get_arg("new-file").to_option();
            let delete_old = command.has("delete-old");

            unarchive_command(input_path_string, new_file_path, delete_old);
        }
        "details" => {
            let input_path_string = command.get_option("input").throw_if_none();

            details_command(input_path_string)
        }
        "models" => models_command(),
        _ => cli.help(None),
    }
}
