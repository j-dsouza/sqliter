use std::fmt;

pub enum SQLCommandType {
    Select,
    Insert,
    Unknown,
}

impl fmt::Display for SQLCommandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SQLCommandType::Select => f.write_str("select"),
            SQLCommandType::Insert => f.write_str("insert"),
            SQLCommandType::Unknown => f.write_str("Unknown command"),
        }
    }
}

impl SQLCommandType {
    pub fn new(command: String) -> SQLCommandType {
        let args: Vec<&str> = command.split_whitespace().collect();
        let cmd = args[0].to_owned();
        match cmd.as_ref() {
            "select" => SQLCommandType::Select,
            "insert" => SQLCommandType::Insert,
            _ => SQLCommandType::Unknown,
        }
    }
}

pub fn handle_sql_command(command: SQLCommandType) -> Result<String, String> {
    match command {
        SQLCommandType::Select => Ok(format!("{}", "Select statement")),
        SQLCommandType::Insert => Ok(format!("{}", "Insert statement")),
        SQLCommandType::Unknown => Err(format!(
            "{}",
            "Unknown command or invalid arguments. Enter '.help'",
        )),
    }
}
