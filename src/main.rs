use rand::Rng;

fn main() {
    // Extract a random line from the art of war
    let line_n = pick_random_line();
    let quote = get_line(line_n);
    // Format it for newlines in the right places
    let quote = &str::replace(quote, "\\", "\n");
    // Print it
    println!("Sun Tzu said: {}", quote);
}

fn get_line(line_number: usize) -> &'static str {
    let artofwar = include_str!("artofwar.txt");
    let mut quotes = artofwar.lines();
    let quote = quotes.nth(line_number).unwrap();
    quote
}

fn pick_random_line() -> usize {
    let artofwar = include_str!("artofwar.txt");
    let n_quotes = artofwar.lines().count();
    let n = rand::thread_rng().gen_range(0..n_quotes);
    n
}

#[cfg(test)]
mod tests {
    //mod lib

    #[test]
    fn test_get_line() {
        let line = crate::get_line(12);
        let expected_line = "All warfare is based on deception.";
        assert_eq!(line, expected_line);
    }

    #[test]
    fn test_pick_random_line() {
        let line_n = crate::pick_random_line();
        assert!(line_n <= 364);
    }
}
