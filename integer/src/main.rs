use integer::Int;
use std::env::args;
fn main() {
    let b = args();
    let a = Int::from_str("1");
    let b = Int::from_str("781271215");
    println!("{}", (b / a).to_str());
}
