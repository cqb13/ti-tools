use std::env;

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: &'static str,
    pub short: Option<char>,
    pub long: Option<&'static str>,
    pub value_name: &'static str,
    pub default: Option<String>,
    pub help: &'static str,
    pub requires: Vec<&'static str>,
}

#[derive(Debug)]
pub struct Command<'a> {
    pub name: &'a str,
    pub short: Option<char>,
    pub description: &'a str,
    pub args: Option<Vec<Arg>>,
}

pub struct Cli<'a> {
    pub name: String,
    pub bin: String,
    pub description: String,
    pub author: String,
    pub github: String,
    pub version: String,
    pub commands: Vec<Command<'a>>,
    pub default_command: Option<String>,
}

impl Arg {
    pub fn new() -> Arg {
        Arg {
            name: "",
            short: None,
            long: None,
            value_name: "",
            default: None,
            help: "",
            requires: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: &'static str) -> Arg {
        self.name = name;
        self
    }

    pub fn with_short(mut self, short: char) -> Arg {
        self.short = Some(short);
        self
    }

    pub fn with_long(mut self, long: &'static str) -> Arg {
        self.long = Some(long);
        self
    }

    pub fn with_value_name(mut self, value_name: &'static str) -> Arg {
        self.value_name = value_name;
        self
    }

    pub fn default(mut self, default: &'static str) -> Arg {
        self.default = Some(default.to_string());
        self
    }

    pub fn with_help(mut self, help: &'static str) -> Arg {
        self.help = help;
        self
    }

    pub fn requires(mut self, requires: &'static str) -> Arg {
        self.requires.push(requires);
        self
    }
}

impl<'a> Command<'a> {
    pub fn new(name: &'a str, description: &'a str) -> Command<'a> {
        Command {
            name,
            short: None,
            description,
            args: None,
        }
    }

    pub fn with_short(mut self, short: char) -> Command<'a> {
        self.short = Some(short);
        self
    }

    /**
     * Adds an argument to the command
     */
    pub fn with_arg(mut self, arg: Arg) -> Command<'a> {
        if self.args.is_none() {
            self.args = Some(vec![]);
        }
        self.args.as_mut().unwrap().push(arg);
        self
    }

    /**
     * Adds arguments to the command
     */
    pub fn with_args(mut self, args: &Vec<Arg>) -> Command<'a> {
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
                    .any(|s| *s == format!("--{}", required_arg.long.unwrap()))
            {
                println!(
                    "The argument \"{}\" requires the argument \"{}\"",
                    arg.name, required
                );
                std::process::exit(0);
            }
        }
    }

    fn find_arg(&self, arg_name: &'static str) -> Option<&Arg> {
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
    pub fn has(&self, arg_name: &'static str) -> bool {
        self.args
            .as_ref()
            .and_then(|args| args.iter().find(|&arg| arg.name == arg_name))
            .map(|arg| {
                let args: Vec<String> = env::args().collect();
                let found = args.iter().any(|s| {
                    *s == format!("-{}", arg.short.unwrap())
                        || *s == format!("--{}", arg.long.unwrap())
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
    pub fn get_value_of(&self, arg_name: &'static str) -> ArgValue {
        self.args
            .as_ref()
            .and_then(|args| args.iter().find(|&arg| arg.name == arg_name))
            .and_then(|arg| {
                let args: Vec<String> = env::args().collect();
                let arg_index = args.iter().position(|s| {
                    *s == format!("-{}", arg.short.unwrap())
                        || *s == format!("--{}", arg.long.unwrap())
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

impl<'a> Cli<'a> {
    pub fn new() -> Cli<'a> {
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

    pub fn with_name(mut self, name: &'a str) -> Cli<'a> {
        self.name = name.to_string();
        self
    }

    pub fn with_bin(mut self, bin: &'a str) -> Cli<'a> {
        self.bin = bin.to_string();
        self
    }

    pub fn with_description(mut self, description: &'a str) -> Cli<'a> {
        self.description = description.to_string();
        self
    }

    pub fn with_author(mut self, author: &'a str) -> Cli<'a> {
        self.author = author.to_string();
        self
    }

    pub fn with_github(mut self, github: &'a str) -> Cli<'a> {
        self.github = github.to_string();
        self
    }

    pub fn with_version(mut self, version: &'a str) -> Cli<'a> {
        self.version = version.to_string();
        self
    }

    pub fn with_commands(mut self, commands: Vec<Command<'a>>) -> Cli {
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
    pub fn with_default_command(mut self, default_command: &str) -> Cli<'a> {
        self.default_command = Some(default_command.to_string());
        self
    }

    pub fn match_commands(&self) -> &Command<'a> {
        let args: Vec<String> = env::args().collect();
        if args.len() <= 1 {
            if self.default_command.is_some() {
                self.commands
                    .iter()
                    .find(|&command| command.name == self.default_command.as_ref().unwrap())
                    .unwrap_or_else(|| {
                        println!(
                            "Failed to find set default command: {}",
                            self.default_command.as_ref().unwrap()
                        );
                        std::process::exit(0);
                    })
            } else {
                self.help();
                std::process::exit(0);
            }
        } else {
            let command_name = &args[1];
            self.commands
                .iter()
                .find(|&command| {
                    command.name == command_name
                        || (command.short
                            == Some(
                                command_name
                                    .replace("-", "")
                                    .to_lowercase()
                                    .chars()
                                    .next()
                                    .unwrap(),
                            )
                            && command_name.len() == 1)
                })
                .unwrap_or_else(|| {
                    println!("Command not found: {}", command_name);
                    std::process::exit(0);
                })
        }
    }

    pub fn version(&self) {
        println!("{} {}", self.name, self.version);
    }

    pub fn help(&self) {
        println!("{} {}", self.name, self.version);
        println!("{}", self.description);
        println!("Author: {}", self.author);
        println!("Github: {}", self.github);
        println!();
        println!("USAGE:");
        println!("    {} [COMMAND] [OPTIONS]", self.bin);
        println!();
        println!("COMMANDS:");
        for command in &self.commands {
            println!("    {:<12} -{}", command.name, command.short.unwrap_or(' '));
            println!("        {}", command.description);

            if let Some(args) = &command.args {
                for arg in args {
                    let short = arg
                        .short
                        .map(|s| format!("-{}", s))
                        .unwrap_or("".to_string());
                    let long = arg
                        .long
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
        println!();
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
