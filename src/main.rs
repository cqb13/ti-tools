pub mod cli;
pub mod commands;
pub mod calculator;
#[cfg(test)]
pub mod tests;
pub mod tokens;

use cli::{Arg, Cli, Command};
use commands::comment::comment_command;
use commands::decode::decode_command;
use commands::details::details_command;
use commands::encode::encode_command;
use commands::models::models_command;
use commands::rename::rename_command;

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
        Command::new("decode", "Converts 8xp to txt")
            .with_arg(
                Arg::new()
                .with_name("input")
                .with_value_name("INPUT")
                .with_help("The input path to an 8xp file")
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
                    .with_name("model")
                    .with_long("model")
                    .with_short('m')
                    .with_value_name("MODEL")
                    .with_help("The model of calculator (use models command to see the supported models) | Default: latest"),
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
        Command::new("encode", "Converts txt to 8xp")
            .with_arg(
                Arg::new()
                .with_name("input")
                .with_value_name("INPUT")
                .with_help("The input path to an 8xp file")
            )
            .with_arg(
                Arg::new()
                    .with_name("output")
                    .with_long("output")
                    .with_short('o')
                    .with_value_name("OUTPUT")
                    .with_help("The output path to a 8xp file"),
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
        Command::new("rename", "Renames the program name in a 8xp file")
            .with_arg(
                Arg::new()
                .with_name("input")
                .with_value_name("INPUT")
                .with_help("The input path to an 8xp file")
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
                    .with_name("model")
                    .with_long("model")
                    .with_short('m')
                    .with_value_name("MODEL")
                    .with_help("The model of calculator (use models command to see the supported models) | Default: latest"),
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
        Command::new("comment", "Write a custom comment to an 8xp file")
            .with_arg(
                Arg::new()
                .with_name("input")
                .with_value_name("INPUT")
                .with_help("The input path to an 8xp file")
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
                    .with_name("model")
                    .with_long("model")
                    .with_short('m')
                    .with_value_name("MODEL")
                    .with_help("The model of calculator (use models command to see the supported models) | Default: latest"),
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
        Command::new("details", "Displays information about an 8xp file")
            .with_arg(
                Arg::new()
                .with_name("input")
                .with_value_name("INPUT")
                .with_help("The input path to an 8xp file")
            )
            .with_arg(
                Arg::new()
                    .with_name("model")
                    .with_long("model")
                    .with_short('m')
                    .with_value_name("MODEL")
                    .with_help("The model of calculator (use models command to see the supported models) | Default: latest"),
            ),
        Command::new("models", "Prints the supported TI calculator models"),
    ]);

    let command = cli.match_commands();

    match command.name {
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
            let model = command
                .get_value_of("model")
                .to_option()
                .unwrap_or("latest".to_string());
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
                model,
                content,
                preview,
            );
        }
        "encode" => {
            let input_path_string = command.get_value().throw_if_none();
            let output_path_string = command.get_value_of("output").to_option();
            let model = command
                .get_value_of("model")
                .to_option()
                .unwrap_or("latest".to_string());
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
                model,
                encode_mode,
                content,
                preview,
            );
        }
        "rename" => {
            let input_path_string = command.get_value().throw_if_none();
            let name = command.get_value_of("name").throw_if_none();
            let model = command
                .get_value_of("model")
                .to_option()
                .unwrap_or("latest".to_string());
            let new_file_path = command.get_value_of("new-file").to_option();
            let delete_old = command.has("delete-old");

            rename_command(input_path_string, name, model, new_file_path, delete_old);
        }
        "comment" => {
            let input_path_string = command.get_value().throw_if_none();
            let comment = command.get_value_of("comment").throw_if_none();
            let model = command
                .get_value_of("model")
                .to_option()
                .unwrap_or("latest".to_string());
            let new_file_path = command.get_value_of("new-file").to_option();
            let delete_old = command.has("delete-old");

            comment_command(input_path_string, comment, model, new_file_path, delete_old)
        }
        "details" => {
            let input_path_string = command.get_value().throw_if_none();
            let model = command
                .get_value_of("model")
                .to_option()
                .unwrap_or("latest".to_string());

            details_command(input_path_string, model)
        }
        "models" => models_command(),
        _ => cli.help(None),
    }
}
