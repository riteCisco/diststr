# Instructions to run **FRANKENSTEIN PROTOTYPE**:

1. Rust must be installed on your machine (preferably added to path)
    a. If not installed, use this cli command to install: https://www.rust-lang.org/tools/install
2. To run (starting from diststr directory):
3.     cd src
4.     cargo run -- {domain name} {path to wordlist} {Other flags}
# FLAG INSTRUCTION ***NOT IMPLEMENTED (mainly cause I don't know if it's completely necessary or not)***
- -l -> Chance if being leetspoken
- -c -> Chance of being found if testing upper/lowercase variation
- -64 -> Compared against Best64 rule list
- -d -> Compared against dive rule list

# Rationale
- The Damerau–Levenshtein distance allows the transposition of two adjacent characters alongside insertion, deletion, substitution
    - See theory here: https://en.wikipedia.org/wiki/Damerau–Levenshtein_distance
- Thus, it's possible to tell how DIFFERENT something is from a certain entry, and be able to extrapolate how many extra "rules" might need to be added in order to get there
- I was thinking to add some of the funny rule stuff to minimize keyspace, but the thought process is that if I'm only measuring the DISTANCE it might not be necessary to try to counter the PERMUTATIONS specifically
    - Waiting for more guidance on that

# Guidance Requested
- The permutation and rule stuff. I don't know if I'm overthinking it at all
- Usage of algorithm. I trust the distance algorithm, just not sure if that's what the project is requiring or asking
- Rust optimization. This is the first time I've coded in Rust, and the reason why this script took a bit long was because I was learning the language for the first time. Please let me know if there's anything I can improve on! Any and all feedback is appreciated
- If the script is completely wrong, please let me know. I want to make this script better!
