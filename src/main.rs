use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let word_count = count_words(&contents);

    println!("Number of words: {}", word_count.len());
    println!("Top 10 most common words:");
    for (word, count) in top_words(&word_count, 10) {
        println!("{}: {}", word, count);
    }
}

fn count_words(text: &str) -> HashMap<String, usize> {
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let clean = word
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();

        if !clean.is_empty() {
            *word_count.entry(clean).or_insert(0) += 1;
        }
    }

    word_count
}

fn top_words(counts: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut sorted: Vec<_> = counts.iter().map(|(w, &c)| (w.clone(), c)).collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted.into_iter().take(n).collect()
}
