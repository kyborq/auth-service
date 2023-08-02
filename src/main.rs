mod token;
mod utils;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    println!("Hello, world!");
}
