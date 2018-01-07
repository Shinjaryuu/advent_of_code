use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::Read;

fn get_input(filename:&str) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    return Ok(contents);
}

fn is_anagram(s1:&str, s2:&str) -> bool {
    let mut st1 : Vec<char> = s1.chars().collect();
    let mut st2 : Vec<char> = s2.chars().collect();
    st1.sort_unstable();
    st2.sort_unstable();
    return st1 == st2;
}

fn is_anagram_passphrase(s:&str) -> bool {
    let mut h = HashSet::new();
    for word in s.split_whitespace(){
        let h2 = h.clone();
        for w in h2 {
            if is_anagram(word,w) {
                return false;
            }
        }
        h.insert(word);
    }
    return true;
}

fn is_passphrase(s:&str) -> bool {
    let mut h = HashSet::new();
    for word in s.split_whitespace(){
        if h.contains(word){
            return false;
        }
        h.insert(word);
    }
    return true;
}

fn count_passphrases(s : &str) -> u32 {
    return s.lines().map(|p| if is_passphrase(p){1} else{0}).sum()
    //let mut counter : u32 = 0;
    //for phrase in s.split("\n"){
    //    if is_passphrase(phrase) {
    //        counter += 1;
    //    }
    //}
    //return counter;
}

fn count_anagram_passphrases(s : &str) -> u32 {
    return s.lines().map(|p| if is_anagram_passphrase(p){1} else{0}).sum()

}

fn main() {
    let input_file = "Input";
    let input = match get_input(input_file){
        Ok(s) => s,
        Err(_) => panic!("Failed to read file!"),
    };

    println!("{} passphrases.", count_passphrases(&input));
    println!("{} anagram passphrases.", count_anagram_passphrases(&input));
}
