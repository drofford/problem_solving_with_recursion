mod algorithms;
mod utils;

use algorithms::fibonacci_on_the_fly;
use utils::get_i64;

fn main() {

    let mut fill_on_the_fly_values: Vec<i64> = vec![0, 1];

    loop {
        // Prompt the user for n.
        let n = get_i64("N: ");

        // Calculate the Fibonacci number.
        println!("On the fly: {}", fibonacci_on_the_fly(&mut fill_on_the_fly_values, n));
        println!();
    }
}
