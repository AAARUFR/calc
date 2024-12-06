use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<_> = env::args().collect();
    if args.len() == 3 && args[2] == "squared" {
        let Ok(x): Result<i32, _> = args[1].parse() else {
            return ExitCode::FAILURE;
        };
        println!("Result: {}", x * x);
        return ExitCode::SUCCESS;
    } else if args.len() < 4 {
        eprintln!("Usage: calc num1 operator num2 or calc num squared");
        return ExitCode::FAILURE;
    }

    let Ok(x): Result<i32, _> = args[1].parse() else {
        return ExitCode::FAILURE;
    };
    let Ok(y): Result<i32, _> = args[3].parse() else {
        return ExitCode::FAILURE;
    };

    let result;
    match args[2].as_str() {
        "plus" | "+" | "add" => result = x + y,
        "minus" | "subtract" | "-" => result = x - y,
        "times" | "x" | "multiplied" | "multiplied by" => result = x * y,
        "divided" | "divided by" | "/" => result = x / y,
        "squared" => result = x * x,
        _ => {
            eprintln!("Unknown operator: {}", args[2]);
            return ExitCode::FAILURE;
        }
    }

    println!("Result: {}", result);

    return ExitCode::SUCCESS;
}
