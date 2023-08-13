mod hello;

use crate::hello::hello;

fn main() {
    println!("{}", hello());
    println!("{}", hello::sample::hello2());
}
