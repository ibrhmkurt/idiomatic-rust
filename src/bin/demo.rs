use idiomatic_rust::*;

fn main() -> Result<Money, MoneyError> {
    "15 $".parse::<Money>()
}