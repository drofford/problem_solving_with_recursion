mod algorithms;
mod utils;

use algorithms::fibonacci;
use utils::get_i64;

fn main() {
    println!("Enter -1 to exit\n");
    loop {
        // Prompt the user for n.
        let n = get_i64("N: ");

        // If n < 0, break out of the loop.
        if n < 0 {
            break;
        }

        // Calculate the Fibonacci number.
        println!("fibonacci({}) = {}\n", n, fibonacci(n));
    }
}
