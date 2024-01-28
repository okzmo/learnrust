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

        match order.chars().last() {
            Some('P') => match order[..order.len() - 1].trim().parse::<usize>() {
                Ok(i) if i > 0 && i <= todos.len() => {
                    if todos[i - 1].status == TodoStatus::InProgress {
                        println!("This task is already in progress.");
                    } else {
                        todos[i - 1].status = TodoStatus::InProgress
                    }
                }
                _ => {
                    println!("Invalid index")
                }
            },
            Some('D') => match order[..order.len() - 1].trim().parse::<usize>() {
                Ok(i) if i > 0 && i <= todos.len() => todos[i - 1].status = TodoStatus::Done,
                _ => {
                    println!("Invalid index")
                }
            },
            _ if order.chars().count() > 1 => {
                let task = Todo {
                    name: String::from(order),
                    status: TodoStatus::Todo,
                };

                todos.push(task);
            }
            _ => {
                println!("Invalid input")
            }
        }

        println!("{:?}", todos);
    }
}
