use std::io;

#[derive(Debug, PartialEq)]
enum TodoStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug)]
struct Todo {
    name: String,
    status: TodoStatus,
}

impl Todo {
    fn new(name: &str) -> Self {
        Todo {
            name: name.to_string(),
            status: TodoStatus::Todo,
        }
    }
}

fn parse_order(order: &str) -> Result<(usize, char), &'static str> {
    if let Some('P') | Some('D') = order.chars().last() {
        let (num_str, cmd) = order.split_at(order.len() - 1);
        num_str
            .trim()
            .parse::<usize>()
            .map(|i| (i, cmd.chars().next().unwrap()))
            .map_err(|_| "Invalid number")
    } else {
        Err("Invalid command")
    }
}

fn main() {
    println!(
        "Welcome to Todo CLI, to create a todo just write it, 
    to mark it as in progress just write TodoNumberP like 1P and to mark it 
    as done it's similar just change the P to a D"
    );

    let mut todos: Vec<Todo> = Vec::new();

    loop {
        println!("Input:");
        let mut order = String::new();

        io::stdin().read_line(&mut order).expect("Failed read line");
        let order = order.trim();

        match parse_order(order) {
            Ok((i, 'P')) if i > 0 && i <= todos.len() => {
                if todos[i - 1].status == TodoStatus::InProgress {
                    println!("This task is already in progress.");
                } else {
                    todos[i - 1].status = TodoStatus::InProgress;
                }
            }
            Ok((i, 'D')) if i > 0 && i <= todos.len() => {
                if todos[i - 1].status == TodoStatus::Done {
                    println!("This task is already marked as done.");
                } else {
                    todos[i - 1].status = TodoStatus::Done;
                }
            }
            Ok(_) => {
                println!("Invalid index");
            }
            Err(_) if order.len() > 1 => todos.push(Todo::new(order)),
            Err(e) => {
                println!("{}", e);
            }
        }

        println!("{:?}", todos);
    }
}
