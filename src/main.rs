pub mod calculator;
pub mod cli;
pub mod commands;
pub mod styles;
#[cfg(test)]
pub mod tests;
pub mod tokens;

use cli::{Arg, Cli, CmdOption, Command};
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
    let cli = Cli::new()
        .with_default_command("help")
        .with_commands(vec![
        Command::new("help", "Prints help information")
            .with_option(
                CmdOption::new("command", "COMMAND", "The command you want help with")
                    .optional()
            ),
        Command::new("version", "Prints version information"),
        Command::new("decode", "Converts 8xp/82p/83p to txt")
            .with_option(
                CmdOption::new("input", "INPUT", "The input path to an 8xp/82p/83p file")
            )
            .with_arg(
                Arg::new("output", "The output path to a txt file", "output", 'o')
                    .with_value_name("OUTPUT"),
            )
            .with_arg(
                Arg::new("display-mode", "The characters to translate the tokens to [pretty, accessible, ti] | Default: accessible", "display-mode", 'd')
                    .with_default_value("accessible")
                    .with_value_name("DISPLAY_MODE")
            )
            .with_arg(
                Arg::new("content", "Display the content of the input file", "content", 'c')
            )
            .with_arg(
                Arg::new("preview", "Display the decoded output", "preview", 'p')
            ),
        Command::new("encode", "Converts txt to 8xp")
            .with_option(
                CmdOption::new("input", "INPUT", "The input path to an txt file")
            )
            .with_arg(
                Arg::new("output", "The output path to an 8xp/82p/83p file", "output", 'o')
                    .with_value_name("OUTPUT"),
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
            ),
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
            ),
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
            ),
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
            ),
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
            ), 
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
            ),
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
            ),
        Command::new("details", "Displays information about an 8xp/82p/83p file")
            .with_option(
                CmdOption::new("input", "INPUT", "The input path to an 8xp/82p/83p file")
            ),
        Command::new("models", "Prints the supported TI calculator models"),
        ]);

    let command = cli.match_commands();

    match command.name.as_str() {
        "help" => {
            let command = command.get_option("command").to_option();
            cli.help(command.as_deref())
        }
        "version" => cli.version(),
        "decode" => {
            let input_path_string = command.get_option("input").throw_if_none();
            let output_path_string = command.get_arg("output").to_option();
            let display_mode = command
                .get_arg("display-mode")
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
            let input_path_string = command.get_option("input").throw_if_none();
            let output_path_string = command.get_arg("output").to_option();
            let encode_mode = command
                .get_arg("encode-mode")
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
