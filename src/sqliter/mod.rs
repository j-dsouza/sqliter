use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq)]
pub enum DataTypes {
    Integer,
    Float,
    String,
    Bool,
    Invalid,
}

pub struct Table {
    table_name: String,
    columns: Vec<Column>,
    // rows: Vec<HashMap<String, Row>>,
    primary_key: String,
    // index:
    // (or string?)
}

impl Table {
    pub fn new(
        table_name: String,
        columns: Vec<Column>,
        // rows: Vec<HashMap<String, Row>>,
        primary_key: String,
    ) -> Self {
        let table_name: String = "people".to_string();
        let id_col = Column::new("id".to_string(), DataTypes::Integer, true, true);
        let username_col = Column::new("username".to_string(), DataTypes::String, true, true);
        let email_col = Column::new("email".to_string(), DataTypes::String, true, true);
        let mut columns = Vec::new();
        columns.push(id_col);
        columns.push(username_col);
        columns.push(email_col);
        let primary_key: String = "id".to_string();

        Table {
            table_name: table_name,
            columns: columns,
            primary_key: primary_key,
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Table <table_name: {}, columns: {:?}, primary_key {}>",
            self.table_name, self.columns, self.primary_key,
        )
    }
}

#[derive(PartialEq)]
pub struct Column {
    column_name: String,
    data_type: DataTypes,
    is_pk: bool,
    is_nullable: bool,
}

impl Column {
    pub fn new(column_name: String, data_type: DataTypes, is_pk: bool, is_nullable: bool) -> Self {
        Column {
            column_name: column_name,
            data_type: data_type,
            is_pk: is_pk,
            is_nullable: is_nullable,
        }
    }
}

impl fmt::Debug for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Column <column_name: {}, is_pk: {}, is_nullable: {}>",
            self.column_name, self.is_pk, self.is_nullable,
        )
    }
}

pub struct Row {
    id: i32,
    username: String,
    email: String,
}
impl Row {}
