use std::env;

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: String,
    pub short: Option<char>,
    pub long: Option<String>,
    pub value_name: String,
    pub default: Option<String>,
    pub help: String,
    pub requires: Vec<String>,
}

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub args: Option<Vec<Arg>>,
}

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

impl Arg {
    pub fn new() -> Arg {
        Arg {
            name: String::new(),
            short: None,
            long: None,
            value_name: String::new(),
            default: None,
            help: String::new(),
            requires: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: &str) -> Arg {
        self.name = name.to_string();
        self
    }

    pub fn with_short(mut self, short: char) -> Arg {
        if short == 'h' {
            panic!("'h' short is reserved for the help command");
        }
        self.short = Some(short);
        self
    }

    pub fn with_long(mut self, long: &str) -> Arg {
        if long == "help" {
            panic!("'help' long is reserved for the help command");
        }
        self.long = Some(long.to_string());
        self
    }

    pub fn with_value_name(mut self, value_name: &str) -> Arg {
        self.value_name = value_name.to_string();
        self
    }

    pub fn default(mut self, default: &str) -> Arg {
        self.default = Some(default.to_string());
        self
    }

    pub fn with_help(mut self, help: &str) -> Arg {
        self.help = help.to_string();
        self
    }

    pub fn requires(mut self, requires: &str) -> Arg {
        self.requires.push(requires.to_string());
        self
    }
}

impl Command {
    pub fn new(name: &str, description: &str) -> Command {
        Command {
            name: name.to_string(),
            description: description.to_string(),
            args: None,
        }
    }

    /**
     * Adds an argument to the command
     */
    pub fn with_arg(mut self, arg: Arg) -> Command {
        if self.args.is_none() {
            self.args = Some(vec![]);
        }
        self.args.as_mut().unwrap().push(arg);
        self
    }

    /**
     * Adds arguments to the command
     */
    pub fn with_args(mut self, args: &Vec<Arg>) -> Command {
        if self.args.is_none() {
            self.args = Some(vec![]);
        }
        self.args.as_mut().unwrap().extend(args.iter().cloned());
        self
    }

    /**
     * Checks if the required arguments are present
     */
    fn check_if_required_args_are_present(&self, env_args: &Vec<String>, arg: &Arg) {
        for required in &arg.requires {
            let required_arg = self.find_arg(required).unwrap();
            if !env_args
                .iter()
                .any(|s| *s == format!("-{}", required_arg.short.unwrap()))
                && !env_args
                    .iter()
                    .any(|s| *s == format!("--{}", required_arg.long.as_ref().unwrap()))
            {
                println!(
                    "The argument \"{}\" requires the argument \"{}\"",
                    arg.name, required
                );
                std::process::exit(0);
            }
        }
    }

    fn find_arg(&self, arg_name: &str) -> Option<&Arg> {
        self.args
            .as_ref()
            .and_then(|args| args.iter().find(|&arg| arg.name == arg_name))
    }

    /**
     * Get the first string value after command name without a flag
     */
    pub fn get_value(&self) -> ArgValue {
        let args: Vec<String> = env::args().collect();
        if args.len() <= 2 {
            return ArgValue::Missing(self.name.to_string());
        }
        let mut value = String::new();
        for arg in &args[2..] {
            if arg.starts_with("-") {
                break;
            }
            value.push_str(arg);
            value.push_str(" ");
        }
        ArgValue::Present(value.trim().to_string())
    }

    /**
     * Check if a flag is present
     */
    pub fn has(&self, arg_name: &str) -> bool {
        self.args
            .as_ref()
            .and_then(|args| args.iter().find(|&arg| arg.name == arg_name))
            .map(|arg| {
                let args: Vec<String> = env::args().collect();
                let found = args.iter().any(|s| {
                    *s == format!("-{}", arg.short.unwrap())
                        || *s == format!("--{}", arg.long.as_ref().unwrap())
                });

                if found {
                    self.check_if_required_args_are_present(&args, arg);
                }

                found
            })
            .unwrap_or(false)
    }

    /**
     * Get the value of a flag
     */
    pub fn get_value_of(&self, arg_name: &str) -> ArgValue {
        self.args
            .as_ref()
            .and_then(|args| args.iter().find(|&arg| arg.name == arg_name))
            .and_then(|arg| {
                let args: Vec<String> = env::args().collect();
                let arg_index = args.iter().position(|s| {
                    *s == format!("-{}", arg.short.unwrap())
                        || *s == format!("--{}", arg.long.as_ref().unwrap())
                });

                let value = arg_index.and_then(|index| args.get(index + 1));
                value
                    .or_else(|| arg.default.as_ref())
                    .map(|s| s.to_string())
            })
            .map(|value| ArgValue::Present(value))
            .unwrap_or(ArgValue::Missing(arg_name.to_string()))
    }
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
                self.help(Some(command_name.to_string()));
                std::process::exit(0);
            }

            command
        }
    }

    pub fn version(&self) {
        println!("{} {}", self.name, self.version);
    }

    pub fn help(&self, command_name: Option<String>) {
        println!("{} {}", self.name, self.version);
        println!("{}", self.description);
        println!("Author: {}", self.author);
        println!("Github: {}", self.github);
        println!();
        println!("USAGE:");
        println!("    {} [COMMAND] [OPTIONS]", self.bin);
        println!();
        println!("COMMANDS:");
        if command_name.is_some() {
            let command = self
                .commands
                .iter()
                .find(|&command| command.name == *command_name.as_ref().unwrap())
                .unwrap_or_else(|| {
                    println!("Command not found: {}", command_name.unwrap());
                    std::process::exit(0);
                });

            self.command_help(command)
        } else {
            for command in &self.commands {
                self.command_help(&command)
            }
        }
        println!();
    }

    fn command_help(&self, command: &Command) {
        println!("    {:<12}", command.name);
        println!("        {}", command.description);

        if let Some(args) = &command.args {
            for arg in args {
                let short = arg
                    .short
                    .map(|s| format!("-{}", s))
                    .unwrap_or("".to_string());
                let long = arg
                    .long
                    .as_ref()
                    .map(|s| format!("--{}", s))
                    .unwrap_or("".to_string());
                let value = format!("<{}>", arg.value_name);
                let default = arg.default.as_ref().map(|s| format!(" (default: {})", s));
                println!(
                    "        {:<12} {:<12} {:<12} {:<12}{}",
                    short,
                    long,
                    value,
                    arg.help,
                    default.unwrap_or("".to_string())
                );
            }
        }
    }
}

#[derive(Debug)]
pub enum ArgValue {
    Missing(String),
    Present(String),
}

impl ArgValue {
    pub fn throw_if_none(&self) -> String {
        match self {
            ArgValue::Missing(name) => {
                println!("Missing required argument: {}", name);
                std::process::exit(0);
            }
            ArgValue::Present(value) => value.to_string(),
        }
    }

    pub fn to_option(&self) -> Option<String> {
        match self {
            ArgValue::Missing(_) => None,
            ArgValue::Present(value) => Some(value.to_owned()),
        }
    }
}
