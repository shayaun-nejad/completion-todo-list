use std::io::{self, Write};

const CAPACITY: usize = 3; // Set the desired capacity here

fn main() {
    let mut todo_list: Vec<String> = Vec::new();

    loop {
        print_menu();
        let user_input = get_input();

        match user_input.trim() {
            "1" => {
                if todo_list.len() < CAPACITY {
                    println!("Enter a new to-do item:");
                    let new_item = get_input();
                    todo_list.push(new_item);
                } else {
                    println!("You cannot add a new item until there is space in the list.");
                }
            }
            "2" => {
                if !todo_list.is_empty() {
                    println!("Enter the index of the item you want to remove (1-based):");
                    let index_str = get_input();
                    if let Ok(index) = index_str.trim().parse::<usize>() {
                        if index > 0 && index <= todo_list.len() {
                            todo_list.remove(index - 1);
                            println!("Item removed.");
                        } else {
                            println!("Invalid index. Please try again.");
                        }
                    } else {
                        println!("Invalid input. Please try again.");
                    }
                } else {
                    println!("Your to-do list is empty.");
                }
            }
            "3" => {
                if !todo_list.is_empty() {
                    println!("To-do list items:");
                    for (index, item) in todo_list.iter().enumerate() {
                        println!("{}. {}", index + 1, item);
                    }
                } else {
                    println!("Your to-do list is empty.");
                }
            }
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
            }        
        }
    }
}

fn print_menu() {
    println!("\nCompletion-based To-Do List with Capacity");
    println!("1. Add a new item");
    println!("2. Remove an item");
    println!("3. View all items");
    println!("4. Exit");
    print!("Choose an option: ");
    io::stdout().flush().unwrap();
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}
