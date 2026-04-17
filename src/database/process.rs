use std::io::{self, Write};

use serde_json::json;

use super::{database::Database, error::DatabaseError, printer::Printer};

enum Command {
    Help,
    Tables,
    Fields(Option<String>),
    Use(Option<String>),
    Back,
    Exit,
    Empty,
    Unknown(String),
}

#[derive(Clone, Copy)]
enum TableSelectionAction {
    ShowFields,
    UseTable,
}

pub(crate) fn process() {
    let mut database = Database::new("app", 32, false, 4);
    let mut current_table = None;
    let mut table_selection = None;

    Printer::clear_console();

    if let Err(error) = seed_demo_data(&mut database) {
        Printer::print_error(&error);
    }

    Printer::print_welcome(database.name());
    Printer::print_options(current_table.as_deref());

    loop {
        let prompt = match table_selection {
            Some(_) => Printer::selection_prompt(database.name()),
            None => Printer::prompt(database.name(), current_table.as_deref()),
        };
        print!("{prompt}");
        io::stdout().flush().expect("Failed flushing stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed reading line");

        let command = parse_command(input.trim(), current_table.as_deref());

        if matches!(&command, Command::Empty) {
            continue;
        }

        Printer::clear_console();

        if let Some(selection_action) = table_selection.take() {
            handle_table_selection(
                &database,
                input.trim(),
                selection_action,
                &mut current_table,
            );
            continue;
        }

        match command {
            Command::Help => Printer::print_help(current_table.as_deref()),
            Command::Tables => Printer::print_tables(&database, current_table.as_deref()),
            Command::Fields(table_name) => {
                let requested_table = table_name.or_else(|| current_table.clone());

                match requested_table {
                    Some(table_name) => match database.get_table(&table_name) {
                        Some(table) => {
                            Printer::print_fields(&table_name, table);
                            Printer::print_options(current_table.as_deref());
                        }
                        None => {
                            Printer::print_table_not_found(&table_name);
                            Printer::print_options(current_table.as_deref());
                        }
                    },
                    None => {
                        show_table_selector(
                            &database,
                            TableSelectionAction::ShowFields,
                            current_table.as_deref(),
                        );
                        table_selection = Some(TableSelectionAction::ShowFields);
                    }
                }
            }
            Command::Use(table_name) => {
                match table_name {
                    Some(table_name) => {
                        if database.get_table(&table_name).is_some() {
                            current_table = Some(table_name.clone());
                            Printer::print_selected_table(&table_name);
                            if let Some(table) = database.get_table(&table_name) {
                                Printer::print_fields(&table_name, table);
                            }
                            Printer::print_options(current_table.as_deref());
                        } else {
                            Printer::print_table_not_found(&table_name);
                            Printer::print_options(current_table.as_deref());
                        }
                    }
                    None => {
                        show_table_selector(
                            &database,
                            TableSelectionAction::UseTable,
                            current_table.as_deref(),
                        );
                        table_selection = Some(TableSelectionAction::UseTable);
                    }
                }
            }
            Command::Back => {
                current_table = None;
                Printer::print_options(current_table.as_deref());
            }
            Command::Exit => {
                Printer::print_exit();
                break;
            }
            Command::Empty => {}
            Command::Unknown(input) => {
                Printer::print_unknown_command(&input);
                Printer::print_options(current_table.as_deref());
            }
        }
    }
}

fn parse_command(input: &str, current_table: Option<&str>) -> Command {
    if input.is_empty() {
        return Command::Empty;
    }

    if let Some(command) = parse_menu_command(input, current_table) {
        return command;
    }

    let mut parts = input.split_whitespace();
    let command = parts.next().unwrap_or_default().to_ascii_lowercase();
    let argument = parts.collect::<Vec<_>>().join(" ");

    match command.as_str() {
        "help" => Command::Help,
        "tables" => Command::Tables,
        "fields" => {
            if argument.is_empty() {
                Command::Fields(None)
            } else {
                Command::Fields(Some(argument))
            }
        }
        "use" => {
            if argument.is_empty() {
                Command::Use(None)
            } else {
                Command::Use(Some(argument))
            }
        }
        "back" => Command::Back,
        "exit" | ".exit" => Command::Exit,
        _ => Command::Unknown(input.to_string()),
    }
}

fn parse_menu_command(input: &str, current_table: Option<&str>) -> Option<Command> {
    match (current_table, input) {
        (Some(_), "1") => Some(Command::Fields(None)),
        (Some(_), "2") => Some(Command::Tables),
        (Some(_), "3") => Some(Command::Use(None)),
        (Some(_), "4") => Some(Command::Back),
        (Some(_), "5") => Some(Command::Exit),
        (None, "1") => Some(Command::Tables),
        (None, "2") => Some(Command::Fields(None)),
        (None, "3") => Some(Command::Use(None)),
        (None, "4") => Some(Command::Help),
        (None, "5") => Some(Command::Exit),
        _ => None,
    }
}

fn show_table_selector(
    database: &Database,
    action: TableSelectionAction,
    current_table: Option<&str>,
) {
    let title = match action {
        TableSelectionAction::ShowFields => "Select a table to print its fields:",
        TableSelectionAction::UseTable => "Select a table to make it current:",
    };
    let table_names = database.table_names();
    Printer::print_table_selector(title, &table_names, current_table);
}

fn handle_table_selection(
    database: &Database,
    input: &str,
    action: TableSelectionAction,
    current_table: &mut Option<String>,
) {
    if matches!(input, "0" | "back" | "cancel") {
        Printer::print_options(current_table.as_deref());
        return;
    }

    let table_names = database.table_names();
    let Some(table_name) = resolve_table_name(&table_names, input) else {
        Printer::print_invalid_table_selection(input);
        Printer::print_table_selector(selection_title(action), &table_names, current_table.as_deref());
        return;
    };

    match action {
        TableSelectionAction::ShowFields => {
            if let Some(table) = database.get_table(&table_name) {
                Printer::print_fields(&table_name, table);
                Printer::print_options(current_table.as_deref());
            } else {
                Printer::print_table_not_found(&table_name);
                Printer::print_options(current_table.as_deref());
            }
        }
        TableSelectionAction::UseTable => {
            *current_table = Some(table_name.clone());
            Printer::print_selected_table(&table_name);
            if let Some(table) = database.get_table(&table_name) {
                Printer::print_fields(&table_name, table);
            }
            Printer::print_options(current_table.as_deref());
        }
    }
}

fn selection_title(action: TableSelectionAction) -> &'static str {
    match action {
        TableSelectionAction::ShowFields => "Select a table to print its fields:",
        TableSelectionAction::UseTable => "Select a table to make it current:",
    }
}

fn resolve_table_name(table_names: &[&str], input: &str) -> Option<String> {
    if let Ok(index) = input.parse::<usize>() {
        return table_names
            .get(index.checked_sub(1)?)
            .map(|table_name| (*table_name).to_string());
    }

    table_names
        .iter()
        .find(|table_name| **table_name == input)
        .map(|table_name| (*table_name).to_string())
}

fn seed_demo_data(database: &mut Database) -> Result<(), DatabaseError> {
    database.create_table("users".to_string())?;
    database.create_table("orders".to_string())?;

    let users = database
        .get_table_mut("users")
        .ok_or(DatabaseError::NotFound)?;
    let first_user = users.insert_row();
    users.set_value(first_user, "name".to_string(), json!("Alice"))?;
    users.set_value(first_user, "email".to_string(), json!("alice@example.com"))?;
    users.set_value(first_user, "role".to_string(), json!("admin"))?;

    let second_user = users.insert_row();
    users.set_value(second_user, "name".to_string(), json!("Bob"))?;
    users.set_value(second_user, "email".to_string(), json!("bob@example.com"))?;
    users.set_value(second_user, "role".to_string(), json!("editor"))?;

    let orders = database
        .get_table_mut("orders")
        .ok_or(DatabaseError::NotFound)?;
    let first_order = orders.insert_row();
    orders.set_value(first_order, "number".to_string(), json!("ORD-001"))?;
    orders.set_value(first_order, "amount".to_string(), json!(1250))?;
    orders.set_value(first_order, "status".to_string(), json!("paid"))?;

    Ok(())
}
