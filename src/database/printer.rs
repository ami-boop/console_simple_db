use std::io::{self, Write};

use super::{database::Database, error::DatabaseError, table::Table};

pub(crate) struct Printer;

impl Printer {
    pub(crate) fn clear_console() {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().expect("Failed flushing stdout");
    }

    pub(crate) fn print_welcome(database_name: &str) {
        println!("========================================");
        println!(" Simple DB Console");
        println!(" Database: {database_name}");
        println!("========================================");
        println!("Use menu numbers or type commands manually.");
        println!();
    }

    pub(crate) fn prompt(database_name: &str, current_table: Option<&str>) -> String {
        match current_table {
            Some(table_name) => format!("{database_name}[{table_name}]> "),
            None => format!("{database_name}> "),
        }
    }

    pub(crate) fn selection_prompt(database_name: &str) -> String {
        format!("{database_name}[select-table]> ")
    }

    pub(crate) fn print_help(current_table: Option<&str>) {
        println!("Available commands:");
        println!("  help            - show commands");
        println!("  tables          - print all tables");
        println!("  fields <table>  - print fields for table");
        println!("  use <table>     - select table for quick access");
        println!("  back            - clear selected table");
        println!("  exit | .exit    - close console");
        println!("  1..5            - run menu actions by number");
        println!();
        Self::print_options(current_table);
    }

    pub(crate) fn print_options(current_table: Option<&str>) {
        match current_table {
            Some(table_name) => {
                println!("Menu:");
                println!("  1. Show fields of current table");
                println!("  2. Show all tables");
                println!("  3. Select another table");
                println!("  4. Back");
                println!("  5. Exit");
                println!("Current table: {table_name}");
                println!("Commands: fields | tables | use <table> | back | exit");
            }
            None => {
                println!("Menu:");
                println!("  1. Show all tables");
                println!("  2. Show fields of a table");
                println!("  3. Select a table");
                println!("  4. Help");
                println!("  5. Exit");
                println!("Commands: tables | fields <table> | use <table> | help | exit");
            }
        }
        println!();
    }

    pub(crate) fn print_tables(database: &Database, current_table: Option<&str>) {
        let table_names = database.table_names();

        println!("----------------------------------------");
        println!("Tables in `{}`:", database.name());
        println!("----------------------------------------");

        if table_names.is_empty() {
            println!("No tables created yet.");
            println!();
            Self::print_options(current_table);
            return;
        }

        for (index, table_name) in table_names.iter().enumerate() {
            let table = database
                .get_table(table_name)
                .expect("table should exist while iterating");
            let field_names = table.field_names();
            let field_summary = if field_names.is_empty() {
                "no fields yet".to_string()
            } else {
                field_names.join(", ")
            };

            println!(
                "{}. {} | rows: {} | fields: {}",
                index + 1,
                table_name,
                table.row_count(),
                field_summary
            );
        }

        println!();
        Self::print_options(current_table);
    }

    pub(crate) fn print_fields(table_name: &str, table: &Table) {
        let field_names = table.field_names();

        println!("----------------------------------------");
        println!("Fields in table `{table_name}`:");
        println!("----------------------------------------");

        if field_names.is_empty() {
            println!("No fields found. Add rows or values to define schema.");
            println!();
            return;
        }

        for (index, field_name) in field_names.iter().enumerate() {
            println!("{}. {}", index + 1, field_name);
        }

        println!();
        println!("Total fields: {}", field_names.len());
        println!("Rows in table: {}", table.row_count());
        println!();
    }

    pub(crate) fn print_selected_table(table_name: &str) {
        println!("Selected table: {table_name}");
        println!("Tip: now you can call `fields` without a table name.");
        println!();
    }

    pub(crate) fn print_table_not_found(table_name: &str) {
        println!("Table `{table_name}` not found.");
        println!("Use `tables` to inspect available names.");
        println!();
    }

    pub(crate) fn print_table_selector(
        title: &str,
        table_names: &[&str],
        current_table: Option<&str>,
    ) {
        println!("----------------------------------------");
        println!("{title}");
        println!("----------------------------------------");

        if table_names.is_empty() {
            println!("No tables available.");
            println!();
            Self::print_options(current_table);
            return;
        }

        for (index, table_name) in table_names.iter().enumerate() {
            println!("  {}. {}", index + 1, table_name);
        }

        println!("  0. Cancel");
        println!();
        println!("Enter a table number.");
        println!();
    }

    pub(crate) fn print_invalid_table_selection(input: &str) {
        println!("Invalid table selection: `{input}`");
        println!("Choose a number from the list or enter `0` to cancel.");
        println!();
    }

    pub(crate) fn print_unknown_command(input: &str) {
        println!("Unknown command: `{input}`");
        println!("Use `help` to inspect supported commands.");
        println!();
    }

    pub(crate) fn print_error(error: &DatabaseError) {
        println!("Operation failed: {error:?}");
        println!();
    }

    pub(crate) fn print_exit() {
        println!("Console stopped.");
    }
}
