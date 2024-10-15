use idiomatic_rust::*;

fn main() {
    let p = "15 $".parse::<Money>().unwrap();
    println!("{p:#?}")
}