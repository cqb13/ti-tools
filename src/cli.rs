use crate::prints;
use std::env;

pub struct Cli {
    pub name: String,
    pub bin: String,
    pub description: String,
    pub author: String,
    pub github: String,
    pub version: String,
    pub commands: Vec<Command>,
    pub default_command: Option<String>,
}

impl Cli {
    pub fn new() -> Cli {
        Cli {
            name: env!("CARGO_PKG_NAME").to_string(),
            bin: env!("CARGO_PKG_NAME").to_string(),
            description: env!("CARGO_PKG_DESCRIPTION").to_string(),
            author: env!("CARGO_PKG_AUTHORS").to_string(),
            github: env!("CARGO_PKG_REPOSITORY").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            commands: Vec::new(),
            default_command: None,
        }
    }

    pub fn with_name(mut self, name: &str) -> Cli {
        self.name = name.to_string();
        self
    }

    pub fn with_bin(mut self, bin: &str) -> Cli {
        self.bin = bin.to_string();
        self
    }

    pub fn with_description(mut self, description: &str) -> Cli {
        self.description = description.to_string();
        self
    }

    pub fn with_author(mut self, author: &str) -> Cli {
        self.author = author.to_string();
        self
    }

    pub fn with_github(mut self, github: &str) -> Cli {
        self.github = github.to_string();
        self
    }

    pub fn with_version(mut self, version: &str) -> Cli {
        self.version = version.to_string();
        self
    }

    pub fn with_commands(mut self, commands: Vec<Command>) -> Cli {
        self.commands = commands;
        self
    }

    /**
     * The default command to run if just the program name is called
     * The default_command should be the name of the desired command
     * If it is a command that should not be normally run, don't add it to cli command list, just match for name in the main function
     *
     * If a default command is not set, the auto generated help command will be run
     */
    pub fn with_default_command(mut self, default_command: &str) -> Cli {
        self.default_command = Some(default_command.to_string());
        self
    }

    pub fn match_commands(&self) -> &Command {
        let args: Vec<String> = env::args().collect();
        if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {}

        if args.len() <= 1 {
            if self.default_command.is_some() {
                self.commands
                    .iter()
                    .find(|&command| command.name == *self.default_command.as_ref().unwrap())
                    .unwrap_or_else(|| {
                        println!(
                            "Failed to find set default command: {}",
                            self.default_command.as_ref().unwrap()
                        );
                        std::process::exit(0);
                    })
            } else {
                self.help(None);
                std::process::exit(0);
            }
        } else {
            let command_name = &args[1];
            let command = self
                .commands
                .iter()
                .find(|&command| command.name == *command_name)
                .unwrap_or_else(|| {
                    println!("Command not found: {}", command_name);
                    std::process::exit(0);
                });

            if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
                self.help(Some(command_name));
                std::process::exit(0);
            }

            command
        }
    }

    pub fn help(&self, command_name: Option<&str>) {
        cli_help(self, command_name);
    }

    pub fn version(&self) {
        println!("{} {}", self.name, self.version);
    }
}

pub struct Command {
    pub name: String,
    pub description: String,
    pub options: Vec<CmdOption>,
    pub args: Vec<Arg>,
}

impl Command {
    pub fn new(name: &str, description: &str) -> Command {
        Command {
            name: name.to_string(),
            description: description.to_string(),
            options: Vec::new(),
            args: Vec::new(),
        }
    }

    pub fn with_option(mut self, option: CmdOption) -> Command {
        self.options.push(option);
        self
    }

    pub fn with_arg(mut self, arg: Arg) -> Command {
        self.args.push(arg);
        self
    }

    fn check_if_required_args_are_present(&self, env_args: &Vec<String>, arg: &Arg) {
        if arg.requires.is_some() {
            for required in arg.requires.as_ref().unwrap() {
                let required_arg = self.find_arg(required).unwrap();
                if !env_args
                    .iter()
                    .any(|s| *s == format!("-{}", required_arg.short))
                    && !env_args
                        .iter()
                        .any(|s| *s == format!("--{}", required_arg.long))
                {
                    println!(
                        "The argument \"{}\" requires the argument \"{}\"",
                        arg.name, required
                    );
                    std::process::exit(0);
                }
            }
        }
    }

    fn find_arg(&self, arg_name: &str) -> Option<&Arg> {
        self.args.iter().find(|&arg| arg.name == arg_name)
    }

    /**
     * Check if a flag is present
     */
    pub fn has(&self, arg_name: &str) -> bool {
        self.args
            .iter()
            .find(|&arg| arg.name == arg_name)
            .map(|arg| {
                let args: Vec<String> = env::args().collect();
                let found = args
                    .iter()
                    .any(|s| *s == format!("-{}", arg.short) || *s == format!("--{}", arg.long));

                if found {
                    self.check_if_required_args_are_present(&args, arg);
                }

                found
            })
            .unwrap_or(false)
    }

    /**
     * Get the value of the option in its location
     */
    pub fn get_option(&self, option_name: &str) -> Value {
        self.options
            .iter()
            .enumerate()
            .find(|(_, option)| option.name == option_name)
            .and_then(|(index, option)| {
                let args: Vec<String> = env::args().collect();
                if args.len() <= 2 + (index) {
                    return Some(Value::Missing(format!(
                        "{} could not be found in its location ({})",
                        option_name, index
                    )));
                }

                let value = args[1 + (index + 1)].to_string();

                if value == "--" && option.required {
                    return Some(Value::Missing(format!(
                        "{} is a required value and must be specified",
                        option_name
                    )));
                }

                return Some(Value::Present(value));
            })
            .unwrap()
    }

    /**
     * Get the value of a flag
     */
    pub fn get_arg(&self, arg_name: &str) -> Value {
        self.args
            .iter()
            .find(|&arg| arg.name == arg_name)
            .and_then(|arg| {
                let args: Vec<String> = env::args().collect();
                let arg_index = args.iter().position(|s| {
                    *s == format!("-{}", arg.short) || *s == format!("--{}", arg.long)
                });

                let value = arg_index.and_then(|index| args.get(index + 1));
                value
                    .or_else(|| arg.default_value.as_ref())
                    .map(|s| s.to_string())
            })
            .map(|value| Value::Present(value))
            .unwrap_or(Value::Missing(format!("{} could not be found", arg_name)))
    }
}

pub struct CmdOption {
    name: String,
    value_name: String,
    description: String,
    required: bool,
}

impl CmdOption {
    pub fn new(name: &str, value_name: &str, description: &str) -> CmdOption {
        CmdOption {
            name: name.to_string(),
            value_name: value_name.to_string(),
            description: description.to_string(),
            required: true,
        }
    }

    pub fn optional(mut self) -> CmdOption {
        self.required = false;
        self
    }
}

pub struct Arg {
    pub name: String,
    pub description: String,
    pub short: char,
    pub long: String,
    pub value_name: Option<String>,
    pub default_value: Option<String>,
    pub requires: Option<Vec<String>>,
    pub required: bool,
}

impl Arg {
    pub fn new(name: &str, description: &str, long: &str, short: char) -> Arg {
        Arg {
            name: name.to_string(),
            description: description.to_string(),
            short,
            long: long.to_string(),
            value_name: None,
            default_value: None,
            requires: None,
            required: false,
        }
    }

    pub fn with_value_name(mut self, value_name: &str) -> Arg {
        self.value_name = Some(value_name.to_string());
        self
    }

    pub fn with_default_value(mut self, default_value: &str) -> Arg {
        self.default_value = Some(default_value.to_string());
        self
    }

    pub fn with_description(mut self, description: &str) -> Arg {
        self.description = description.to_string();
        self
    }

    pub fn requires(mut self, requires: &str) -> Arg {
        if self.requires.is_none() {
            self.requires = Some(vec![requires.to_string()])
        } else {
            self.requires.as_mut().unwrap().push(requires.to_string());
        }
        self
    }

    pub fn required(mut self) -> Arg {
        self.required = true;
        self
    }
}

#[derive(Debug)]
pub enum Value {
    Missing(String),
    Present(String),
}

impl Value {
    pub fn throw_if_none(&self) -> String {
        match self {
            Value::Missing(message) => {
                println!("Missing Input Value: {}", message);
                std::process::exit(1);
            }
            Value::Present(value) => value.to_string(),
        }
    }

    pub fn to_option(&self) -> Option<String> {
        match self {
            Value::Missing(_) => None,
            Value::Present(value) => Some(value.to_owned()),
        }
    }
}

fn cli_help(cli: &Cli, command_name: Option<&str>) {
    if command_name.is_none() {
        println!("{} {}", cli.name, cli.version);
        println!("{}", cli.description);
        println!("Author: {}", cli.author);
        prints!("Github: [color:cyan]{}", cli.github);
        println!();
        prints!("[style:bold]USAGE:");
        println!("    {} [COMMAND] [OPTIONS]", cli.bin);
        println!();
        prints!("[style:bold]COMMANDS:");

        for command in &cli.commands {
            command_help(command, 1);
        }
    } else {
        let command_name: Vec<&str> = command_name.unwrap().split(":").collect();

        let command = cli
            .commands
            .iter()
            .find(|command| command.name == command_name[0]);

        if command.is_none() {
            cli.help(None);
            return;
        }

        let command = command.unwrap();

        command_help(command, 0)
    }
}

fn command_help(command: &Command, indent: u8) {
    prints!(
        "[style:bold]{}{} - {}",
        "    ".repeat(indent as usize),
        command.name,
        command.description
    );
    for option in &command.options {
        println!(
            "    {}{:<12}  {:<26}  {:>12} {}",
            "    ".repeat((indent) as usize),
            option.name,
            format!("<{}>", option.value_name),
            if option.required {
                " (required)"
            } else {
                "(optional)"
            },
            option.description,
        )
    }
    for arg in &command.args {
        println!(
            "    {}-{:<12} --{:<12} {:<14}{:<12}{}",
            "    ".repeat((indent) as usize),
            arg.short,
            arg.long,
            if arg.value_name.is_some() {
                format!("<{}>", arg.value_name.as_ref().unwrap())
            } else {
                "".to_string()
            },
            if arg.required { " (required)" } else { "" },
            arg.description,
        );
    }

    println!();
}
