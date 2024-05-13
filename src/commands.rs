use crate::manager::BillManager;
use console::{style, Term};
use dialoguer::{Confirm, Input, Select};

fn get_input(prompt: &str) -> String {
    Input::new().with_prompt(prompt).interact_text().unwrap()
}

pub fn init_app(manager: &mut BillManager) {
    let term = Term::stdout();
    let theme = dialoguer::theme::ColorfulTheme::default();
    loop {
        term.clear_screen().unwrap();
        println!("{}", style("Bill Management System").bold().underlined());

        let choices = [
            "Add Bill",
            "View Bills",
            "Edit Bill",
            "Remove Bill",
            "About",
            "Exit",
        ];
        let selection = Select::with_theme(&theme)
            .with_prompt("Choose an action")
            .default(0)
            .items(&choices)
            .interact_on_opt(&term)
            .unwrap();

        if let Some(index) = selection {
            term.clear_screen().unwrap();
            match index {
                0 => add_bill(manager),
                1 => manager.view_bills(),
                2 => edit_bill(manager),
                3 => remove_bill(manager),
                4 => show_about(),
                5 => break,
                _ => unreachable!(),
            }
            println!("\nPress Enter to return to the menu...");
            let _ = std::io::stdin().read_line(&mut String::new());
        } else {
            break; // Exit if the user escapes the menu without selecting
        }
    }
}

fn add_bill(manager: &mut BillManager) {
    let description = get_input("Enter the description of the bill");

    let amount = loop {
        let input: String = get_input("Enter the amount of the bill");

        match input.parse::<f64>() {
            Ok(val) => break val,
            Err(_) => println!("Please enter a valid number for the amount."),
        }
    };

    manager.add_bill(description, amount);
    println!("Bill added successfully!");
}

fn remove_bill(manager: &mut BillManager) {
    let id = loop {
        let input: String = get_input("Enter the ID of the bill to remove");

        match input.parse::<usize>() {
            Ok(val) => {
                if manager.remove_bill(val) {
                    println!("Bill removed successfully!");
                    break;
                } else {
                    println!("No bill found with ID {}. Please try a different ID.", val);
                }
            }
            Err(_) => println!("Please enter a valid ID."),
        }
    };
}

fn edit_bill(manager: &mut BillManager) {
    let id = loop {
        let input: String = get_input("Enter the ID of the bill to edit");

        match input.parse::<usize>() {
            Ok(val) => {
                if manager.edit_bill(val, None, None) {
                    break val;
                } else {
                    println!("No bill found with ID {}. Please enter a valid ID.", val);
                }
            }
            Err(_) => println!("Please enter a valid number for the ID."),
        }
    };

    let description: String = Input::new()
        .with_prompt("Enter new description (or leave empty to keep current)")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let new_description = if description.trim().is_empty() {
        None
    } else {
        Some(description.trim().to_string())
    };

    let new_amount = loop {
        let input: String = Input::new()
            .with_prompt("Enter new amount (or leave empty to keep current)")
            .allow_empty(true)
            .interact_text()
            .unwrap();

        if input.trim().is_empty() {
            break None;
        } else {
            match input.trim().parse::<f64>() {
                Ok(val) => break Some(val),
                Err(_) => println!("Please enter a valid number for the amount."),
            }
        }
    };

    if !manager.edit_bill(id, new_description, new_amount) {
        println!("Failed to edit the bill. Please try again.");
    } else {
        println!("Bill edited successfully.");
    }
}

fn show_about() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    println!("{}", style("Bill Management System").bold().underlined());
    println!("A simple command-line application to manage bills.");
    println!("Created by Mohamed Kandil");
    println!("Version 0.1.0");
    println!("Licensed under MIT License");
}
