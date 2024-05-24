//use std::env::Args;
use std::env;
use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use colored::Colorize;

//use std::io::prelude::*;
//use cewl to gen wordlist
//Variations based off of hashcat (one rule to rule them all)
/*
Generalization of Hamming distance 
that allows for different length 
strings, and (with Damerau) 
for transpositions
 */

 fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
 where P: AsRef<Path>, {
     let file = File::open(filename)?;
     Ok(io::BufReader::new(file).lines())
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
                distances[i - 1][j] + 1,    // deletion
                distances[i][j - 1] + 1,    // insertion
                distances[i - 1][j - 1] + cost,  // substitution
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
                    distances[i - 2][j - 2] + cost,
                );
            }
        }
    }

    distances[s1_len][s2_len]
}
fn main() {
    let _args1: Vec<String> = env::args().collect();
    dbg!(&_args1);
    let mut _args: Vec<String> = _args1;
    let mut argsze = _args.len();
    let entry;
    let filename;
    let mut smallest_dist = 10000;
    let mut biggest_dist= 0;
    let mut smallest_word = String::new();
    entry = _args.get(1).unwrap().to_string();
    _args.remove(1);
    argsze = argsze - 1;
    println!("Entry: {entry}");
    filename = _args.get(1).unwrap().to_string();
    _args.remove(1);
    argsze = argsze - 1;
    println!("List Path: {filename}");
    //let mut file =  File::create(filename);
    //file.read_line(&mut ph);
    //println!("{entry}");
    let mut flag = true;
    let mut result;
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            result = line.to_string();
            let newdist = dldist(&result, &entry);
            //println!("{newdist}");
            if newdist < smallest_dist {
                smallest_dist = newdist; //NEEDS A TIE BREAKER THAT DOES NOT ADD A BUNCH OF COMPLEXITY
                smallest_word = result.clone();
                if flag {
                    flag = false;
                    continue;
                }
                println!("New Winner! {} + {} with {}", result.blue(), entry.blue(), newdist.to_string().blue());
            }
            if newdist > biggest_dist {
                biggest_dist = newdist;
                if flag {
                    flag = false;
                    continue;
                }
                println!("New Loser! {} + {} with {}", result.red(), entry.red(), newdist.to_string().red());
            }
        }
    }
    println!("Base Answer: {}, with {}", smallest_word.green(), smallest_dist.to_string().green());
    //println!("Damerau-Levenshtein distance: {}", dldist(s1, s2));
    //the challenge isn't the distance itself, the addition of how many changes it would take is what I want
    //println!("{}", _args.get(1).unwrap());
    //Levenstein is a HUGE keyspace. Assume naive distance
    //The bigger the distance is, the better. This assumes that  
    if argsze >= 2 && _args.get(1).unwrap() == "-l" { //leet
        println!("!");
        //here would be the inclusion of a weighted average of the highest DL Distance and the possibility of discovery based off of leetspeak changes
        _args.remove(1);
        argsze = argsze - 1;
    } 
    if argsze >= 2 && _args.get(1).unwrap() == "-c" { //caps
        println!("?");
        //here would be the inclusion of a weighted average including the usage of lower/uppercase, as the entire thing is case sensitive
        //TO-DO: Figure out if this is needed
        _args.remove(1);
        argsze = argsze - 1;
    }
    if argsze >=2 && _args.get(1).unwrap() == "-64" { //stacked best64 
        println!("@");
        //stacked once best64 check (not a lot of rules, should be able to brute force)
       _args.remove(1); 
       argsze = argsze - 1;
    }
    if argsze >= 2 && _args.get(1).unwrap() == "-d" { //dive.rules
        println!("why");
        //why. there's no true pattern, so this might also need to be brute forced
        _args.remove(1);
    }
    //there's always space to add more, but these are the four "cool" ones. everything is possible imo except for dive, that looks like
    //it could be a real pain
}
