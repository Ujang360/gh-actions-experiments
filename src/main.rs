pub fn hello(to: &str) -> String {
    format!("Hello, {}!", to)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_to_is_as_expected() {
        let to = "Kresna";
        let expected_result = format!("Hello, {}!", to);

        assert_eq!(hello(to), expected_result);
    }
}

fn main() {
    println!("{}", hello("world!"));
}
