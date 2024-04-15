pub enum TokenType {
    DotCommands(DotCommands),       // start with '.'
    RunTypes(RunCommands),          // run different files
    Configurations(Conifgurations), // configure the CPU
}

pub enum DotCommands {
    Quit,
    PrintCommands,
}

pub enum RunCommands {
    Default,
    UserDefined(String),
}

pub enum Conifgurations {
    Default,
    SetInstructionTime(u64),
    Memory,
}

pub struct Token {
    pub command_type: TokenType,
}

impl Token {
    pub fn create(command: &str) -> Result<Self, &str> {
        if command.starts_with('.') {
            match Token::analayze_dot_command(command) {
                Ok(content) => {
                    return Ok(Self {
                        command_type: TokenType::DotCommands(content),
                    })
                }
                Err(error) => return Err(error),
            }
        }

        let commands = command.split(' ').collect::<Vec<&str>>();
        if commands[0] == "run" {
            if commands.len() < 2 {
                return Err("please enter a program to run");
            } else if commands.len() > 2 {
                return Err("program name cannot contain spaces");
            }
            match Token::analayze_run_command(commands[1]) {
                Ok(content) => {
                    return Ok(Self {
                        command_type: TokenType::RunTypes(content),
                    })
                }
                Err(error) => return Err(error),
            }
        }

        match Token::analyze_config_command(commands) {
            Ok(content) => {
                return Ok(Self {
                    command_type: TokenType::Configurations(content),
                })
            }
            Err(error) => return Err(error),
        }
    }

    fn analayze_dot_command(command: &str) -> Result<DotCommands, &str> {
        let tokens = command.split(' ').collect::<Vec<&str>>();
        let len = tokens.len() - 1;

        for (i, token) in tokens.into_iter().enumerate() {
            if token == ".q" {
                if i < len {
                    return Err("Syntax Error: please enter .q to exit");
                }
                return Ok(DotCommands::Quit);
            } else if token == ".help" {
                if i < len {
                    return Err("Syntax Error: please enter .help to print all commands");
                }
                return Ok(DotCommands::PrintCommands);
            }
        }

        Err("this is not a command, please enter '.help' to list available commands")
    }

    fn analayze_run_command(program: &str) -> Result<RunCommands, &str> {
        //currently this will only match the program name with existing programs and will return whether the program is default or not
        if program == "-df" {
            return Ok(RunCommands::Default);
        }
        Ok(RunCommands::UserDefined(program.to_owned()))
    }

    fn analyze_config_command(config: Vec<&str>) -> Result<Conifgurations, &str> {
        if config[0] != "configure" {
            return Err("No such command");
        }
        if config[1] == "-It" {
            if config.len() != 3 {
                return Err("Impropare use of config command");
            }
            if let Ok(time) = config[2].parse::<u64>() {
                return Ok(Conifgurations::SetInstructionTime(time));
            } else {
                return Err("Instruction time needs to be an unsigned integer");
            }
        }

        Err("This type of configuration does not exist")
    }
}
