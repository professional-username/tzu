use rand::Rng;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value)]
    pattern: String,
}

fn main() {
    let pattern = Cli::from_args().pattern;
    // If no pattern is provided, extract a random line from the art of war
    if pattern == "" {
        let line_n = pick_random_line();
        let quote = get_line(line_n);
        // Format it for newlines in the right places
        let quote = &str::replace(quote, "\\", "\n");
        println!("Sun Tzu said: {}", quote);
    }
    // Otherwise, print all lines with the relevant quote
    else {
        find_lines(pattern)
    }
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

fn find_lines(pattern: String) {
    let artofwar = include_str!("artofwar.txt");
    for line in artofwar.lines() {
        if line.contains(&pattern) {
            // Format lines for newlines etc and print them
            let quote = &str::replace(line, "\\", "\n");
            println!("Sun Tzu said: {}\n", quote);
        }
    }
}

#[cfg(test)]
mod tests {

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

    #[test]
    fn test_default_struct_args() {
        use structopt::StructOpt;
        let pattern = crate::Cli::from_args().pattern;
        assert!(pattern == "")
    }

    #[test]
    fn test_find_lines() {
        crate::find_lines("War".to_string());
    }
}
