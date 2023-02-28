use std::collections::HashSet;
use std::io;
use substring::Substring;

fn identify_compound_words(words: &HashSet<String>, word: &str) -> bool {
    if words.len() >= 3 {
        for i in 0..word.len() {
            if words.contains(word.substring(0, i)) && words.contains(word.substring(i, word.len())) {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let mut words:HashSet<String> = HashSet::new();
    
    println!("Enter the word count:");
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let int_num = num.trim().parse().unwrap();

    println!("Enter the words:");
    for _ in 0..int_num {
        let mut word = String::new();
        io::stdin().read_line(&mut word).unwrap();

        if word.len() < 1 {
            break;
        } 
        words.insert(String::from(word.trim()));
    }

    let mut compounds = Vec::new();
    for word in words.iter() {
        if identify_compound_words(&words, word) {
            compounds.push(word);
        }
    }
    compounds.sort();
    println!("{:?}", compounds);
}