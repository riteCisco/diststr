use std::env;
use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use colored::Colorize;
use std::collections::HashMap;
use std::time::Instant;
//use std::io::prelude::*;
//use cewl to gen wordlist?
//Variations based off of hashcat (one rule to rule them all)
/*
Generalization of Hamming distance 
that allows for different length 
strings, and (with Damerau) 
for transpositions
 */
/*fn sixfourify(&str) -> Vec<String> {

}*/

fn variety_calc(input: &str) -> f64 {
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

fn length_calc(word: &str) -> f64 {
    let space:f64 = 62.0;
    return space.powf(word.len() as f64);

}
fn l33tify(word: &str, flag: bool) -> Vec<String> {
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
 fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
 where P: AsRef<Path>, {
     let file = File::open(filename)?;
     Ok(io::BufReader::new(file).lines())
 }

fn expandomatic(str: &str, flags: (bool, bool, bool, bool, bool)) -> Vec<String>{
    let mut master_vec: Vec<String> = Vec::new(); 
    master_vec.push(str.to_string());
    if flags.1 { //leet
        master_vec.extend(l33tify(str, flags.4));
    }
    if flags.2 {
        master_vec.extend(caseify(str, flags.4));
    }

    master_vec
    /*let mut file = File::create("./genfile").expect("Error creating file");

    for line in masterVec {
        writeln!(file, "{}", line).expect("Failure to write to file");
    }*/
}

fn dldist(s1: &str, s2: &str) -> usize {
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

fn main() {
    let start = Instant::now();
    let args1: Vec<String> = env::args().collect();
    dbg!(&args1);
    let mut args: Vec<String> = args1;
    let mut argsze = args.len();
    let entry;
    let filename;
    let mut smallest_dist = 10000;
    let mut biggest_dist= 0;
    let mut smallest_word = String::new();
    let mut list_flag = (false, false, false, false, false);
    entry = args.get(1).unwrap().to_string();
    args.remove(1);
    argsze -= 1;
    println!("Entry: {entry}");
    filename = args.get(1).unwrap().to_string();
    args.remove(1);
    argsze = argsze - 1;
    println!("List Path: {filename}");
    argsze -= 1;
    while argsze != 0 {
        let ph = args.get(1).unwrap();
        if  ph == "-d" {
            list_flag.0 = true;
            args.remove(1);
            argsze -= 1;
            continue;
        } 
        if  ph == "-l" { //leet
            //println!("!");
            list_flag.1 = true;
            //here would be the inclusion of a weighted average of the highest DL Distance and the possibility of discovery based off of leetspeak changes
            args.remove(1);
            argsze -= 1;
            continue;
        } 
        if  ph == "-c" { //caps
            //println!("?");
            list_flag.2 = true;
            //here would be the inclusion of a weighted average including the usage of lower/uppercase, as the entire thing is case sensitive
            //TO-DO: Figure out if this is needed
            args.remove(1);
            argsze -= 1;
            continue;
        }
        if  ph == "-s" { //only dl distance 
            //println!("@");
            list_flag.3 = true;
            //stacked once best64 check (not a lot of rules, should be able to brute force)
            args.remove(1); 
            argsze -= 1;
            continue;
        }
        if  ph == "-li" { //light mode
            list_flag.4 = true;
            args.remove(1);
            argsze -= 1;
            continue;
        }
        eprintln!("Invalid flag {}", ph);
        std::process::exit(1);
    }
    if list_flag.0 {
        println!("Debug flag set to {}", list_flag.0);
        println!("Leet flag set to {}", list_flag.1); 
        println!("Upper/Lower flag set to {}", list_flag.2);
        println!("Simple flag set to {}", list_flag.3);
        println!("Light flag set to {}", list_flag.4);
    }
    //there's always space to add more, but these are the four "cool" ones. everything is possible imo except for dive, that looks like
    //it could be a real pain 
    //let mut file =  File::create(filename);
    //file.read_line(&mut ph);
    //println!("{entry}");
    let mut flag = true;
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let test_list = expandomatic(&line, list_flag);
            for test in test_list {
                let newdist = dldist(&test, &entry);
                //println!("{newdist}");
                if newdist < smallest_dist {
                    smallest_dist = newdist; //NEEDS A TIE BREAKER THAT DOES NOT ADD A BUNCH OF COMPLEXITY
                    smallest_word = test.clone();
                    if flag {
                        flag = false;
                        continue;
                    }
                    if list_flag.0 {
                        println!("New Winner! {} + {} with {}. Current time elapsed: {}", test.blue(), entry.blue(), newdist.to_string().blue(), start.elapsed().as_secs());
                    }
                }
                if newdist > biggest_dist {
                    biggest_dist = newdist;
                    if flag {
                        flag = false;
                        continue;
                    }
                    if list_flag.0 {
                        println!("New Loser! {} + {} with {}. Current time elapsed: {}", test.red(), entry.red(), newdist.to_string().red(), start.elapsed().as_secs());
                    }
                }
            }
        }
        println!("Base Answer: {}, with {}", smallest_word.green(), smallest_dist.to_string().green());
    }
    let mut flag_final = false;
    let mut final_value: f64;
    if smallest_dist < 5 {
        final_value = (((biggest_dist as f64) - (smallest_dist as f64)) / (biggest_dist as f64)) * 60.0; //if there's a close match, boost importance 
        flag_final = true;
    }
    else {
        final_value = (((biggest_dist as f64) - (smallest_dist as f64)) / (biggest_dist as f64)) * 30.0;
    }
    if list_flag.0 {
        println!("Distance calculated score: {}", final_value.to_string().blue());
    }
    if !list_flag.3 { 
        let length_score: f64;
        let mut length_val: f64;
        length_score = length_calc(&entry);
        let max_length_score: f64 = 7.0442343e35; // Defined max length score, 62^20 (perfect score would be length of 20)
        // Apply logarithmic scaling
        length_val = if length_score > 0.0 {
            //let log_base: f64 = 2.0;
            //let normalized_score = length_score.log(log_base);
            //let normalized_max = max_length_score.log(log_base);
            // Now scale the normalized score to the range [0, 35] //CHANGE HERE
            if !flag_final {
                (1.0 - (length_score / max_length_score)) * 50.0 //CHANGE HERE 
            }
            else {
                (1.0 - (length_score / max_length_score)) * 10.0 //CHANGE HERE 
            }
        } else {
            25.0
        }; 
        if length_val < 0.0 {
            length_val = 0.0;
        }
        let variety_score: f64;
        let variety_val: f64;
        variety_score = variety_calc(&entry);
        variety_val = (1.0 - variety_score) * 20.0;
        if list_flag.0 {
            println!("Length score: {}", length_score.to_string().blue());
            println!("Brute Force Calculation Score: {}", length_val.to_string().blue());
            println!("Character Diversity: {}", variety_score.to_string().blue());
            println!("Character Diversity Calculation Score: {}", variety_val.to_string().blue());
        }
        final_value += variety_val + length_val;
    }
    if (final_value as i32) < 50 {
        println!("You score is: {}. Looks good! This string is pretty complex!", (final_value as i32).to_string().green()); 
    }
    else if (final_value as i32) > 80 {
        if !flag_final {
            println!("You score is: {}. You should probably add more complexity.", (final_value as i32).to_string().red());
        }
        else {
            println!("You score is: {}. Your string was awfully close to a word on your wordlist, {}.", (final_value as i32).to_string().red(), smallest_word.red());
        }
    }
    else {
        println!("Your score is {}. It's decent, but you could do better.", (final_value as i32).to_string().yellow());
    }
    if list_flag.0 {
        println!("This process took a total of {} seconds", start.elapsed().as_secs().to_string().yellow());
    }
}
