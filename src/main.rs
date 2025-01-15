use std::io; 
mod messages; 
mod math;
fn main() {

    messages::print_title();

    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("Input failed \n");
    let initial_value = input.trim();
    let input:i64 = match input
                            .trim()
                            .parse() {
                                Ok(num) => math::fibonacci(num),
                                Err(_) => math::fibonacci(-1),
                            };
    
    messages::print_output(initial_value, input )
}


