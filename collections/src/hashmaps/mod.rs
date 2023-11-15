use std::collections::HashMap;

pub fn do_a_thing() {
    let mut scores = HashMap::new();

    // These inserts take ownership to the hashmap
    scores.insert(String::from("Blue"), 10);
    // Inserting again with a different value will overwrite
    scores.insert(String::from("Blue"), 25);
    scores.insert(String::from("Yellow"), 50);
    // But types with a Copy trait get copied

    // Using an Entry will insert if the value doesn't yet exist in the map
    let entry = scores.entry(String::from("Yellow")).or_insert(50);
    // We can update the entry by deref
    *entry += 1;
    // This works because "or_insert" returns a mutable reference
    // NOTE: this would be a problem if we tried to create an immutable reference to the hashmap

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
