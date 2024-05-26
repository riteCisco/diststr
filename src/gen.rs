use std::collections::HashMap;

pub fn expandomatic(str: &str, flags: (bool, bool, bool, bool, bool)) -> Vec<String>{
    let mut master_vec: Vec<String> = Vec::new(); 
    master_vec.push(str.to_string());
    if flags.1 { //leet
        master_vec.extend(l33tify(str, flags.4));
    }
    if flags.2 {
        master_vec.extend(caseify(str, flags.4));
    }
    master_vec
}

/*

*/
pub fn l33tify(word: &str, flag: bool) -> Vec<String> {
    // Define leetspeak substitutions (can be expanded with more substitutions)
    let mut result: Vec<String> = Vec::new();
    let subs: HashMap<char, Vec<char>> = [
        ('a', vec!['4', '@']),
        ('e', vec!['3']),
        ('i', vec!['1', '!']),
        ('o', vec!['0']),
        ('s', vec!['$', '5']),
        ('t', vec!['7']),
    ]
    .iter()
    .cloned()
    .collect();
    if !flag {
        let word_chars: Vec<char> = word.chars().collect();
        genrec(&subs, &word_chars, 0, &mut String::new(), &mut result);
    }
    else {
        result.push(
            word
                .chars()
                .map(|c| {
                    subs.get(&c.to_ascii_lowercase())
                        .and_then(|sub_vec| sub_vec.first())
                        .map_or(c, |leet_char| *leet_char)
                })
                .collect()
        );
    } 
    result
}

fn caseify(input: &str, flag: bool) -> Vec<String> {
    let mut list = Vec::new();
    if !flag {
        genperm(input.chars().collect(), 0, &mut String::new(), &mut list);
    }
    else {
        list.push(input.to_lowercase());
        list.push(input.to_uppercase());
        list.push(input.chars()
            .map(|c| {
                if c.is_uppercase() {
                    c.to_lowercase().to_string()
                } else {
                    c.to_uppercase().to_string()
                }
            })
            .collect())
    }
    list
}

fn genperm(chars: Vec<char>, index: usize, current: &mut String, permutations: &mut Vec<String>) {
    if index == chars.len() {
        permutations.push(current.clone());
        return;
    }
    // Add the lowercase version of the current character and recurse
    current.push(chars[index].to_ascii_lowercase());
    genperm(chars.clone(), index + 1, current, permutations);
    current.pop();
    // Add the uppercase version of the current character and recurse
    current.push(chars[index].to_ascii_uppercase());
    genperm(chars, index + 1, current, permutations);
    current.pop();
}

fn genrec(
    subs: &HashMap<char, Vec<char>>,
    word: &[char],
    index: usize,
    current: &mut String,
    combinations: &mut Vec<String>,
) {
    if index == word.len() {
        combinations.push(current.clone());
        return;
    }
    let ch = word[index];
    // Always include the original character
    current.push(ch);
    genrec(subs, word, index + 1, current, combinations);
    current.pop();
    // Include leetspeak substitutions if they exist
    if let Some(sub_chars) = subs.get(&ch) {
        for &sub_ch in sub_chars {
            current.push(sub_ch);
            genrec(subs, word, index + 1, current, combinations);
            current.pop();
        }
    }
}

