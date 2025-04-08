use integer::Int;

fn main() {
    let a = Int::from_str("196");
    let b = Int::from_str("100");
    println!("{}", a.pow(b).to_str());
}
