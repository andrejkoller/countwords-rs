use clap::{Arg, Command};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("WordCounter")
        .version("1.1")
        .author("Andrej Koller <andrejkoller@outlook.com>")
        .about("Counts word frequencies in text files and optionally exports results to CSV")
        .arg(
            Arg::new("files")
                .help("One or more text files to analyze")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("top")
                .short('t')
                .long("top")
                .help("Show only the N most frequent words")
                .default_value("10"),
        )
        .arg(
            Arg::new("ignore-stopwords")
                .long("ignore-stopwords")
                .help("Ignore common stopwords like 'the', 'and', 'is'")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Export results to a CSV file")
                .num_args(1),
        )
        .get_matches();

    let files: Vec<_> = matches.get_many::<String>("files").unwrap().collect();
    let top_n: usize = matches
        .get_one::<String>("top")
        .unwrap()
        .parse()
        .unwrap_or(10);
    let ignore_stopwords = matches.get_flag("ignore-stopwords");

    let output_path = matches.get_one::<String>("output");

    let mut combined_text = String::new();
    for filename in &files {
        let contents = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("Error reading file {}", filename));
        combined_text.push_str(&contents);
        combined_text.push('\n');
    }

    let word_count = count_words(&combined_text, ignore_stopwords);
    let top_words = top_words(&word_count, top_n);

    println!("Analyzed files: {:?}", files);
    println!("Unique words found: {}", word_count.len());
    println!("\nTop {} most frequent words:", top_n);
    for (word, count) in &top_words {
        println!("{:<15} {}", word, count);
    }

    if let Some(path) = output_path {
        export_to_csv(path, &word_count)?;
        println!("\n✅ Results successfully exported to {}", path);
    }

    Ok(())
}

fn count_words(text: &str, ignore_stopwords: bool) -> HashMap<String, usize> {
    let mut counts = HashMap::new();

    let stopwords: HashSet<&str> = [
        "the", "and", "is", "a", "an", "to", "in", "on", "for", "of", "with", "by", "at", "it",
        "as", "from", "this", "that", "be", "or", "was", "are", "not", "but",
    ]
    .iter()
    .cloned()
    .collect();

    for word in text.split_whitespace() {
        let clean = word
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();

        if clean.is_empty() {
            continue;
        }

        if ignore_stopwords && stopwords.contains(clean.as_str()) {
            continue;
        }

        *counts.entry(clean).or_insert(0) += 1;
    }

    counts
}

fn top_words(counts: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut sorted: Vec<_> = counts.iter().map(|(w, &c)| (w.clone(), c)).collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted.into_iter().take(n).collect()
}

fn export_to_csv(path: &str, counts: &HashMap<String, usize>) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(path)?;
    writer.write_record(&["word", "count"])?;

    for (word, count) in counts {
        writer.write_record(&[word, &count.to_string()])?;
    }

    writer.flush()?;
    Ok(())
}
