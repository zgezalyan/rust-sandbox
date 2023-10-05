use std::collections::HashSet;

fn canonical(word: &str) -> String {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

fn find_anagrams<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_canonical = canonical(word);
    
    possible_anagrams.iter()
        .filter(|&candidate| canonical(candidate) == word_canonical && candidate.to_lowercase() != word.to_lowercase())
        .cloned()
        .collect::<HashSet<&'a str>>()
}

fn main() {
    let word = "listen";
    let possible_anagrams = [
        "enlist",
        "inlets",
        "silent",
        "slinte", 
		"slothf",
		"tenlis",
    ];
    
    let result = find_anagrams(word, &possible_anagrams);
    println!("{:?}", result); // Should output: {"enlist", "inlets", "silent"}
}