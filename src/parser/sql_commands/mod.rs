use crate::sqliter;

use std::fmt;

pub enum SQLCommandType {
    Select(String),
    Insert(String),
    Create(String),
    Unknown(String),
}

impl fmt::Display for SQLCommandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SQLCommandType::Select(_) => f.write_str("select"),
            SQLCommandType::Insert(_) => f.write_str("insert"),
            SQLCommandType::Create(_) => f.write_str("create"),
            SQLCommandType::Unknown(_) => f.write_str("Unknown command"),
        }
    }
}

impl SQLCommandType {
    pub fn new(command: String) -> SQLCommandType {
        let args: Vec<&str> = command.split_whitespace().collect();
        let cmd = args[0].to_owned();
        match cmd.as_ref() {
            "select" => SQLCommandType::Select(command),
            "insert" => SQLCommandType::Insert(command),
            "create" => SQLCommandType::Create(command),
            _ => SQLCommandType::Unknown(command),
        }
    }
}

pub fn handle_sql_command(command: SQLCommandType) -> Result<String, String> {
    match command {
        SQLCommandType::Select(x) => Ok(format!("{} {}", "Select statement:", x)),
        SQLCommandType::Insert(x) => Ok(format!("{} {}", "Insert statement:", x)),
        SQLCommandType::Create(x) => Ok(format!("{} {}", "Create statement:", x)),
        SQLCommandType::Unknown(x) => Err(format!(
            "{} {}",
            "Unknown command or invalid arguments. Enter '.help': ", x
        )),
    }
}

struct CreateStatement {
    table_name: String,
    columns: Vec<sqliter::Column>,
}

impl CreateStatement {
    // pub fn new(statement: SQLCommandType) -> Self {
    //     // CREATE TABLE test (test_1 INTEGER NULL, test_2 STRING NOT NULL)
    //     let mut columns: Vec<sqliter::Column> = Vec::new();

    //     match statement {
    //         SQLCommandType::Create(create_statement) => {
    //             let split_bracket = create_statement.split("(").collect();
    //             let before_bracket = split_bracket[0];
    //             let before_bracket_split_whitespace = before_bracket.split_whitespace().collect();
    //             let after_bracket = split_bracket[1].split(")").collect()[0];
    //             let after_bracket_split_comma = after_bracket.split(",").collect();

    //             for str in after_bracket_split_comma {
    //                 columns.push(string_to_column(str))
    //             }

    //         }
    //         _ => Err(format!(
    //             "{} {}",
    //             "Create statement not recognised:", statement
    //         )),
    //     }
    // }

    fn string_to_column(col_statement: &str) -> sqliter::Column {
        let split_col_statement: Vec<&str> = col_statement.split_whitespace().collect();
        sqliter::Column::new(
            split_col_statement[0].to_owned(),
            sqliter::DataTypes::Integer,
            false,
            false,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sqliter::*;

    #[test]
    fn string_to_column_test() {
        let sql_inputs = ["id INTEGER", "id INTEGER NULL"];

        let expected_res = [sqliter::Column::new(
            String::from("id"),
            DataTypes::Integer,
            false,
            false,
        )];

        for it in sql_inputs.iter().zip(expected_res.iter_mut()) {
            let (sql, exp) = it;
            let res: sqliter::Column = CreateStatement::string_to_column(sql);
            assert_eq!(sql, res)
        }
    }
}
