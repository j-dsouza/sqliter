pub mod meta_commands;
pub mod sql_commands;

use meta_commands::MetaCommandType;
use sql_commands::SQLCommandType;

pub enum CommandType {
    MetaCommandType(MetaCommandType),
    SQLCommandType(SQLCommandType),
}

pub fn get_command_type(command: &String) -> CommandType {
    match command.starts_with(".") {
        true => CommandType::MetaCommandType(MetaCommandType::new(command.to_owned())),
        false => CommandType::SQLCommandType(SQLCommandType::new(command.to_owned())),
    }
}
