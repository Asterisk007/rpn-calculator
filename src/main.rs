/*
    Reverse Polish Notation calculator
    Daniel Ellingson, aka Asterisk007
*/
use std::io;

fn main() {
    let mut stack: Vec<f64> = Vec::new();
    println!("Reverse Polish Notation Calculator\nType 'q!' then enter to quit. '?' then enter for commands.");
    let mut input = String::new();
    while input != String::from("q!") {
        // Clear input for next calculation
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = String::from(input.trim());
        for item in input.split(" ") {
            if String::from(item).parse::<f64>().is_ok() {
                let num = String::from(item).parse::<f64>().unwrap();
                stack.push(num);
            } else {
                match item {
                    "+" | "-" | "*" | "/" => {
                        let mut result: f64;
                        if stack.len() >= 2 {
                            result = stack.pop().unwrap();
                            match item {
                                "+" => {
                                    result = result + stack.pop().unwrap();
                                }
                                "-" => {
                                    result = result - stack.pop().unwrap();
                                }
                                "*" => {
                                    result = result * stack.pop().unwrap();
                                }
                                "/" => {
                                    result = result / stack.pop().unwrap();
                                }
                                _ => {
                                    panic!("Something is definitely wrong on line {}", line!());
                                }
                            }
                            stack.push(result);
                        } else {
                            eprintln!("Input {} more number(s).", 2-stack.len());
                        }
                    }
                    ":" => {
                        // Print the entire stack
                        if stack.len() > 0 {
                            for i in 0..stack.len() {
                                if i < stack.len()-1 {
                                    print!("{} ", stack[i]);
                                } else {
                                    print!("{}\n", stack[i]);
                                }
                            }
                        } else {
                            eprintln!("No numbers on the stack.");
                        }
                    }
                    "." => {
                        // Print the topmost element of the stack.
                        if stack.len() > 0 {
                            println!("{}", stack[stack.len()-1]);
                        } else {
                            eprintln!("No numbers on the stack.");
                        }
                    }
                    "c" => {
                        // Clear stack.
                        stack.resize(0, 0.0);
                    }
                    "?" => {
                        // Help text
                        let help = [
                            "All input should be space-separated. Press enter to confirm input.",
                            "Numerical input (whole or floating point) -> push a number to the stack",
                            "+, -, *, / -> pop two numbers from the stack, perform the requested operation, then push the result onto the stack.",
                            ". -> print topmost element of the stack.",
                            ": -> print the stack",
                            "c -> clear the stack",
                            "? -> print this help text",
                            "q! -> exit program"
                        ];
                        for text in help.iter() {
                            println!("{}", text);
                        }

                    }
                    "q!" => {}
                    _ => println!("Unrecognized symbol: {}", item)
                }
            }
        }
    }
}
