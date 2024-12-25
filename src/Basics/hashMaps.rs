use std::{collections::HashMap, hash::Hash, vec};

fn main() {
    let mut person: HashMap<&str, i32> = HashMap::new();

    person.insert("Victor", 100);
    person.insert("Chuks", 200);
    print!("the {:?}", person.get("Victor").unwrap());

    if person.contains_key("Victor") {
        println!("The value exist")
    } else {
        println!("The value does not exist");
    }
    //Or
    match person.get("Nouman") {
        Some(value) => println!("The value exist{}", value),
        None => println!("The value does not exist"),
    }

    for (name, age) in &person {
        println!("The person {} has an age of {}", name, age);
    }

    // only updates the mapping if there's no value with that key
    person.entry("Victor").or_insert(270);
    println!(" {:?}", person);

    let some_vec = vec![5, 5, 8, 8, 1, 0, 1, 5, 5, 5, 5];
    let mut freq_vec: HashMap<i32, u32> = HashMap::new();

    for i in &some_vec {
        let freq = freq_vec.entry(*i).or_insert(0);
        *freq += 1;
    }

    println!("{:?}", freq_vec);
}
