//!main.rs

use advent_of_code_2022::run;

fn main() {
    if let Err(err) = run() {
        println!("Error occured: {}", err);

        // look for source
        if let Some(source) = err.source() {
            println!("Source of error: {:?}", source);
        }
    }
}
