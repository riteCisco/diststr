# INSTRUCTIONS TO RUN:

1.  Rust must be installed on your machine. If not installed, follow this site's instructions to install: https://www.rust-lang.org/tools/install
3.  To run (starting from diststr directory):
<pre><code>cd src</code></pre>
<pre><code>cargo run -- {domain name} {path to wordlist} {flags}</code></pre>

# FLAG INSTRUCTION

### These flags might add a significant amount (hours) of time to the process:
\-l -> Add on a list of leetcode variations to test

\-c -> Add on a list of upper/lowercase variations to test

 -o -> only levenstein distance, no other complexity calculations   
### Other:
-d -> Prints out debug messages and shows whenever there is a new biggest and shortest distance\
\
-li -> Is not an exhaustive list of variations, but broader applications of the rules.
* i.e: For the word "late", instead of going through the combinations l4te, lat3, and l4t3, the light flag will make it such that it only processes it as l4t3.

# RATIONALE 

- The Damerau–Levenshtein distance allows the transposition of two adjacent characters alongside insertion, deletion, substitution
    - See theory here: https://en.wikipedia.org/wiki/Damerau–Levenshtein_distance
    - There will be a VERY small weight placed on transposition, as even though I want to be able to account for that, it is more of a "if something is mistyped or if there is a human error" type change.
- Thus, it's possible to tell how DIFFERENT something is from a certain entry, and be able to extrapolate how many extra "rules" might need to be added in order to get there
- Instead of weighting the averages, offer to create new list to sort through, it's much simpler. 

# GUIDANCE REQUESTED

- The permutation and rule stuff. I don't know if I'm overthinking it at all
- Usage of algorithm. I trust the distance algorithm, just not sure if that's what the project is requiring or asking
- Rust optimization. This is the first time I've coded in Rust, and the reason why this script took a bit long was because I was learning the language for the first time. Please let me know if there's anything I can improve on! Any and all feedback is appreciated
- If the script is in any way wrong, please let me know. I want to make this script better!

# TO-DO

- Tie-breaking system -> Progress: Done 
- Implement wordlist as baseline https://github.com/danielmiessler/SecLists/tree/master/Discovery/DNS -> Progress: Done, using namelist.txt -> without modifications takes 5 seconds to find shortest distance between "4dministrator-cisco" and administrators, with leet modifications it takes an estimated 10 hours
- Implement more sophisticated flag system to add weights into the algorithm -> Progress: fixed, now is order agnostic and has a dedicated weighting system
- Optimize -> Progress: Threads! :()
- Create lightweight rules -> Progress: Done, created -li, simple mode
- swap the flags, make the distance optional and naive default 