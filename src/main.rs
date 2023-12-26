mod greetings {
    pub fn hello() -> &'static str {
        "Hello, World!"
    }
}

fn main() {
    println!("{}", greetings::hello());
}

#[cfg(test)]
mod tests {
    use super::greetings;

    #[test]
    fn test_hello() {
        assert_eq!(greetings::hello(), "Hello, World!");
    }
}