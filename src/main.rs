#[derive(Debug)]
#[allow(dead_code)]
struct Todo {
    //struct Todo
    id: i32,
    item: String,
    is_done: bool,
}

fn handle_is_done(todo: &mut Todo) {
    todo.is_done = !todo.is_done;
}

fn display_items(todo_list: &mut Vec<Todo>) {
    //loop over the list and display the items with their id, name and status of done or not
    println!("Id. Name Status");
    for todo in todo_list {
        println!("{}. {}--->{}", todo.id, todo.item, todo.is_done);
    }
}

fn main() {
    let mut todo_list: Vec<Todo> = Vec::new();

    let todo_1: Todo = Todo {
        id: 0,
        item: String::from("Buy Milk"),
        is_done: false,
    }; //creating a new vector of type Todo

    todo_list.push(todo_1);

    display_items(&mut todo_list);
}
