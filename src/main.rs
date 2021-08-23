use rand::Rng;

fn main() {
    // Extract a random line from the art of war
    let path = "artofwar.txt";
    let content = std::fs::read_to_string(&path).expect("could not read file");
    let n_quotes = content.lines().count();
    let n = rand::thread_rng().gen_range(0..n_quotes);
    let mut quotes = content.lines();
    let quote = quotes.nth(n).unwrap();
    // Format it for newlines in the right places
    let quote = &str::replace(quote, "\\", "\n");
    // Print it
    println!("Sun Tzu said: {}", quote);
}
