use std::fmt;

pub enum MetaCommandType {
    Exit,
    Help,
    Unknown,
}

impl fmt::Display for MetaCommandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MetaCommandType::Exit => f.write_str(".exit"),
            MetaCommandType::Help => f.write_str(".help"),
            MetaCommandType::Unknown => f.write_str("Unknown command"),
        }
    }
}

impl MetaCommandType {
    pub fn new(command: String) -> MetaCommandType {
        let args: Vec<&str> = command.split_whitespace().collect();
        let cmd = args[0].to_owned();
        match cmd.as_ref() {
            ".exit" => MetaCommandType::Exit,
            ".help" => MetaCommandType::Help,
            _ => MetaCommandType::Unknown,
        }
    }
}

pub fn handle_meta_command(command: MetaCommandType) -> Result<String, String> {
    match command {
        MetaCommandType::Exit => std::process::exit(0),
        MetaCommandType::Help => Ok(format!(
            "{}{}{}",
            "Special commands:\n",
            ".help - Display this message\n",
            ".exit - Quits this application"
        )),
        MetaCommandType::Unknown => Err(format!(
            "{}",
            "Unknown command or invalid arguments. Enter '.help'"
        )),
    }
}
