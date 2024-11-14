fn main() {
    println!("Hello, world!");
}

// Easy to follow solve
pub fn merge_alternatley(word1: String, word2: String) -> String {
    let mut merged_words: String = String::with_capacity(word1.len()+word2.len());
    for item in word1.chars().zip(word2.chars()).into_iter(){
        merged_words.push(item.0);
        merged_words.push(item.1);
    }

    if word1.len() > word2.len() {
        merged_words.push_str(&word1[word2.len()..]);
    }
    if word1.len() < word2.len() {
        merged_words.push_str(&word2[word1.len()..]);
    }

    merged_words
}
