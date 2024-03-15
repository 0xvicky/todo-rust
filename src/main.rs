use std::io::{self};

#[derive(Debug)] //

struct Todo {
    //struct Todo
    id: u32,
    item: String,
    is_done: bool,
} //todo stuct containing property of a todo item.

impl Todo {
    fn handle_todo_status(&mut self) {
        self.is_done = !self.is_done;
    }
    fn handle_todo_edit(&mut self) {
        if let Ok(updated_item) = handle_string_input() {
            self.item = updated_item;
        } else {
            println!("Error in updating the todo!!");
        }
    }
} //method on todo struct that handle the change in status of todo

fn handle_input() -> Result<u32, String> {
    let mut choice = String::new();
    println!("Enter your choice or Id:");
    io::stdin()
        .read_line(&mut choice)
        .map_err(|e| format!("Error:{}", e))?;

    let choice = choice
        .trim()
        .parse::<u32>()
        .map_err(|e| format!("Error parsing:{}", e))?;

    Ok(choice)
}

fn handle_string_input() -> Result<String, String> {
    let mut item = String::new();
    println!("Enter your todo:");
    //read the line from user and stores it
    io::stdin()
        .read_line(&mut item)
        .map_err(|e| format!("Error occured while taking string input:{}", e))?;
    //create the item object using struct

    Ok(item)
}

//update function, that takes the list as param and id from user and update the status of the todo
fn update_item_status(list: &mut Vec<Todo>) {
    if let Ok(id) = handle_input() {
        for todo in list {
            if todo.id == id {
                todo.handle_todo_status();
            }
        }
    } else {
        println!("Error updating");
    }
}

fn update_item(list: &mut Vec<Todo>) {
    if let Ok(id) = handle_input() {
        for todo in list {
            if todo.id == id {
                todo.handle_todo_edit();
            }
        }
    }
}
//Remove function helps to remove the item from the list, by taking list and id of the todo from the user and used the retain method to filter the unwanted todo
fn remove_item(list: &mut Vec<Todo>) {
    //takes the id of the item
    if let Ok(id) = handle_input() {
        list.retain(|todo| todo.id != id);
    } else {
        println!("Error removing");
    }
}

//Add method helps to add the item to the todo vector, we are taking the list as the param
fn add_item(list: &mut Vec<Todo>) {
    println!("Item adding started...");
    //intialise a item variable of type string
    if let Ok(item) = handle_string_input() {
        let id: u32 = list.len() as u32 + 1;
        //read the line from user and stores it
        //create the item object using struct
        let item = Todo {
            id,
            item,
            is_done: false,
        };
        //push the item into the vector
        list.push(item);
    } else {
        println!("Error occured while adding item");
    }
}

//Display the items in the list
fn display_items(todo_list: &mut Vec<Todo>) {
    //loop over the list and display the items with their id, name and status of done or not
    println!("Id. Name---> Status");
    for todo in todo_list {
        println!("{}. {}--->{}", todo.id, todo.item, todo.is_done);
    }
}

fn main() {
    let mut todo_list: Vec<Todo> = Vec::new(); //initialised a new vector that of type Todo struct

    loop {
        println!("What you want to do?");
        println!("1. Add Item");
        println!("2. Remove Item");
        println!("3. Update Item Status");
        println!("4. Display Items");
        println!("5. Update Item");
        println!("6. Quit");

        if let Ok(choice) = handle_input() {
            match choice {
                1 => {
                    add_item(&mut todo_list);
                }
                2 => {
                    remove_item(&mut todo_list);
                }
                3 => {
                    update_item_status(&mut todo_list);
                }
                4 => {
                    display_items(&mut todo_list);
                }
                5 => {
                    update_item(&mut todo_list);
                }
                6 => {
                    println!("GoodBye Baby");
                    return;
                }
                _ => {
                    println!("Invalid Input");
                }
            }
            println!("===============================================================");
            println!("===============================================================");
        } else {
            println!("Invalid Choice");
            continue;
        }
    }
}

//struct method implementation for toogling the status
//Error handling should be done with Result instead of expect
