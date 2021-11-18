mod parser;
mod sqliter;

use std::io;
use std::io::Write;
use text_io::read;

fn main() {
    loop {
        print!("sqliter> ");
        io::stdout().flush();
        let statement: String = read!("{}\n");
        match parser::get_command_type(&statement) {
            parser::CommandType::MetaCommandType(statement) => {
                let meta_command = match parser::meta_commands::handle_meta_command(statement) {
                    Ok(response) => println!("{}", response),
                    Err(err) => eprintln!("{}", err),
                };
            }
            parser::CommandType::SQLCommandType(statement) => {
                let sql_command = match parser::sql_commands::handle_sql_command(statement) {
                    Ok(response) => println!("{}", response),
                    Err(err) => eprintln!("{}", err),
                };
            }
        }
    }
}
