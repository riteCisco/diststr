use std::env;
use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use colored::Colorize;
use std::time::Instant;

mod calc;
use calc::*;

mod gen;
use gen::*;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
                let newdist = dldistcalc(&test, &entry);
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
                (1.0 - (length_score / max_length_score)) * 20.0 //CHANGE HERE 
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
