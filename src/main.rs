use rand::Rng;

fn main() {
    // Extract a random line from the art of war
    let artofwar = include_str!("artofwar.txt");
    let n_quotes = artofwar.lines().count();
    let n = rand::thread_rng().gen_range(0..n_quotes);
    let mut quotes = artofwar.lines();
    let quote = quotes.nth(n).unwrap();
    // Format it for newlines in the right places
    let quote = &str::replace(quote, "\\", "\n");
    // Print it
    println!("Sun Tzu said: {}", quote);
}
