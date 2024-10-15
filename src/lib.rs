/*
    1. Verilen testi miktar ve para birimine ayır.
    2. "42 Lira" -> 42 ve "Lira"
    3. "42 Lira" -> (i32, String)
*/  


pub fn parse_money(input: &str) -> (i32, String) {
    let pieces: Vec<&str>  = input.split_whitespace().collect();
    // ["42", "lira"]
    let quantity: i32  = pieces[0].parse::<i32>().unwrap();
    let currency_unit: String = pieces[1].into();
    (quantity, currency_unit)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_integer_number() {
        let result: (i32, String) = parse_money("42 lira");
        assert_eq!((42, "lira".to_string()), result);
    }
}
