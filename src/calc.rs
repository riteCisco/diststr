use std::collections::HashMap;

/*
Calculate the diversity of characters within the string

Input: input - borrowed string

Output: A score assigned, the higher the score the more diverse the string
*/

pub fn variety_calc(input: &str) -> f64 {
    // Create a HashMap to store character frequencies.
    let mut frequencies = HashMap::new();

    // Iterate over each character in the string.
    for ch in input.chars() {
        // Increment the frequency count for this character.
        *frequencies.entry(ch).or_insert(0) += 1;
    }

    // Calculate the diversity score.
    // This example uses the number of unique characters divided by the total number of characters.
    // The score will be between 0 (no diversity) and 1 (maximum diversity).
    let unique_chars = frequencies.len();
    let total_chars = input.chars().count();
    
    // Avoid division by zero if the string is empty.
    if total_chars > 0 {
        (unique_chars as f64) / (total_chars as f64)
    } else {
        0.0
    }
}

/*
Calculate possible brute force usages of a string, assuming a character space of 62 (a-z, A-Z, 0-9)

Input: word - borrowed string

Output: the number of possible strings after subsitutions could be made
*/

pub fn length_calc(word: &str) -> f64 {
    let space:f64 = 62.0;
    return space.powf(word.len() as f64);
}


/*
Calculate modified dl-dist, with less emphasis on transpositions

Input: s1 - borrowed string
       s2 - borrowed string

Output: the distance between the two strings
*/
pub fn dldistcalc(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<_> = s1.chars().collect();
    let s2_chars: Vec<_> = s2.chars().collect();
    let s1_len = s1_chars.len();
    let s2_len = s2_chars.len();

    let mut distances = vec![vec![0; s2_len + 1]; s1_len + 1];

    for i in 0..=s1_len {
        distances[i][0] = i;
    }

    for j in 0..=s2_len {
        distances[0][j] = j;
    }

    for i in 1..=s1_len {
        for j in 1..=s2_len {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };

            distances[i][j] = *[
                distances[i - 1][j] + 1, // deletion
                distances[i][j - 1] + 1, // insertion
                distances[i - 1][j - 1] + cost, // substitution
            ]
            .iter()
            .min()
            .unwrap();

            if i > 1
                && j > 1
                && s1_chars[i - 1] == s2_chars[j - 2]
                && s1_chars[i - 2] == s2_chars[j - 1]
            {
                distances[i][j] = std::cmp::min(
                    distances[i][j],
                    distances[i - 2][j - 2] + 2, // transposition with higher cost
                );
            }
        }
    }

    distances[s1_len][s2_len]
}