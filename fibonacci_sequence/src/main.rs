use std::io;

fn main() {
    println!("How many Fibonacci elements do you want?");

    let elements = get_user_input();

    if elements != 0 {
        if elements < 9 {
            let mut results: Vec<i32> = Vec::new();

            for _iteration in 0..elements {
                calcule_next_fibonacci_number(&mut results);
            }

            println!("{:?}", results);
            return;
        }

        println!("The last Fibonacci number is: {}", fibonacci(elements));

        println!("\nprogram finished. Have a great day")
    }
}

fn calcule_next_fibonacci_number(result: &mut Vec<i32>) {
    match result.len() {
        0 => result.push(0),
        1 => result.push(1),
        _ => {
            let prev_index = result.len() - 2;
            let current_index = result.len() - 1;
            result.push(result[prev_index] + result[current_index])
        }
    };
}

//RECURSIVE!!!!!
fn fibonacci(number: i32) -> i32 {
     match number {
        0 => 0,
        1 => 1,
        _ => {
            let prev_index = number - 2;
            let current_index = number - 1;

            fibonacci(prev_index) + fibonacci(current_index)
        }
    }
}

fn get_user_input() -> i32 {
    let mut elements = String::new();
    io::stdin()
        .read_line(&mut elements)
        .expect("Failed to read the line");

    match elements.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Is required to insert a valid number.");
            0
        }
    }
}
