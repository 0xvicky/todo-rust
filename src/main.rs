use std::io;

#[derive(Debug)]
#[allow(dead_code)]
struct Todo {
    //struct Todo
    id: u32,
    item: String,
    is_done: bool,
}

fn update_item(list: &mut Vec<Todo>) {
    let mut id = String::new();
    io::stdin()
        .read_line(&mut id)
        .expect("Cannot read the id from user");
    //remove the item from the list with some logic
    let id = id.trim().parse::<u32>().expect("Failed to parse");
    for todo in list {
        if todo.id == id {
            todo.is_done = !todo.is_done;
        }
    }
}

fn remove_item(list: &mut Vec<Todo>) {
    //takes the id of the item
    let mut id = String::new();
    io::stdin()
        .read_line(&mut id)
        .expect("Cannot read the id from user");
    //remove the item from the list with some logic
    let id = id.trim().parse::<u32>().expect("Failed to parse");

    list.retain(|todo| todo.id != id);
}

fn add_item(list: &mut Vec<Todo>) {
    println!("Item adding started...");
    //intialise a item variable of type string
    let mut item = String::new();
    let id: u32 = list.len() as u32 + 1;
    //read the line from user and stores it
    io::stdin()
        .read_line(&mut item)
        .expect("Cannot read line from user");
    //create the item object using struct
    let item = Todo {
        id,
        item,
        is_done: false,
    };
    //push the item into the vector
    list.push(item);
}

fn display_items(todo_list: &mut Vec<Todo>) {
    //loop over the list and display the items with their id, name and status of done or not
    println!("Id. Name---> Status");
    for todo in todo_list {
        println!("{}. {}--->{}", todo.id, todo.item, todo.is_done);
    }
}

fn main() {
    let mut todo_list: Vec<Todo> = Vec::new();

    loop {
        println!("What you want to do?");
        println!("1. Add Item");
        println!("2. Remove Item");
        println!("3. Update Item");
        println!("4. Display Items");
        println!("5. Quit");

        let mut choice: String = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Cannot read input");

        let choice: u32 = choice.trim().parse::<u32>().expect("Invalid choice");

        match choice {
            1 => {
                add_item(&mut todo_list);
            }
            2 => {
                remove_item(&mut todo_list);
            }
            3 => {
                update_item(&mut todo_list);
            }
            4 => {
                display_items(&mut todo_list);
            }
            5 => {
                println!("GoodBye Baby");
                return;
            }
            _ => {
                println!("Invalid Input");
            }
        }

        println!("===============================================================");
        println!("===============================================================");
    }
}
