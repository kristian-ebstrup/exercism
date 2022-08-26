use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // initiate the output HashSet
    let mut anagrams: HashSet<&str> = HashSet::new();

    // initiate HashMaps for counting characters
    let mut characters: HashMap<String, i8> = HashMap::new();
    let mut possible_characters: HashMap<String, i8> = HashMap::new();

    // create a variable for the lowercase word for comparisons 
    let lowercase_word: String = word.to_string().to_lowercase();

    // deconstruct the input word into its constituent characters
    for c in lowercase_word.chars().into_iter() {
        // convert to string
        let c_string: String = c.to_string();

        // check if key already exists; else, insert key and value = 1
        if characters.contains_key(&c_string) {
            characters.insert(c_string.clone(), characters.get(&c_string).unwrap() + 1);   
        }
        else {
            characters.insert(c_string.clone(), 1);
        }
    }

    for possible_word in possible_anagrams.iter() {
        // check if equal length
        if word.len() == possible_word.len() && lowercase_word != possible_word.to_string().to_lowercase() {

            // clear the HashMap
            possible_characters.clear();

            // deconstruct the possible word
            for c in possible_word.chars().into_iter() {
                // convert to lowercase
                let c_string: String = c.to_lowercase().to_string();

                // check if key already exists; else, insert key and value = 1
                if possible_characters.contains_key(&c_string) {
                    possible_characters.insert(c_string.clone(), possible_characters.get(&c_string).unwrap() + 1);   
                }
                else {
                    possible_characters.insert(c_string.clone(), 1);
                }
            }

            // compare to the characters HashSet
            
            if characters == possible_characters { anagrams.insert(possible_word); }
        }
    }

    return anagrams;
}
