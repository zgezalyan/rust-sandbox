extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn reverse_string(s: &str) -> String {
    s.graphemes(true).rev().collect::<String>()
}

fn main() {
    let original = "uüu";
    let reversed = reverse_string(original);
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);
}
