// Have to import hash maps.
use std::collections::HashMap;

fn main() {
    // Like normal, there is an associated new() function.
    // Use insert() to add values.
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);
    println!("{:?}", scores);

    // There is also this hideous way to construct a HashMap from two vectors
    // containing keys and values using iterators, zip, and a collect() method.
    // Not even Go is this ugly.
    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let initial_scores = vec![10, 50];
    // The HashMap<_, _> is necessary because collect() can result in different
    // types depending on usage. At least this doesn't need to be mut.
    let scores: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);

    // HashMap always owns its keys and values. Copy objects like i32 are
    // copied and not Copy objects like String are moved.
    // Types for the ke and value can also be implicit.
    let mut scores = HashMap::new();
    let key = "answer".to_string();
    let value = 42;
    scores.insert(key, value);
    // println!("{} {}", key, value); // key can't be used here. it was moved.
    println!("{}", value); // value is ok. It is Copy since it is an i32.

    let mut scores = HashMap::new();
    let key = "answer".to_string();
    let value = 42;
    scores.insert(&key, value); // using &String as the key type.
    println!("{} {}", key, value); // No move from key

    // Retrieving values is similar in that it uses references. References are
    // forced by the get() method signature.
    let score = scores.get(&key); // score is of type Option<i32>
    println!("{} -> {:?}", key, score);
    // .get call below is because scores is HashMap<String, i32> not str, plus
    // needing the reference.
    println!("{} -> {:?}", "missing", scores.get(&"missing".to_string()));

    // Iterating over the key-value pairs in a HashMap can be done with a loop
    // and a reference.
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // insert also updates values.
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Blue".to_string(), 25);
    println!("{:?}", scores);

    // inserting only if a key has no value is done via entry() and or_insert().
    println!("{:?}", scores.entry("Blue".to_string()));
    println!("{:?}", scores.entry("Yellow".to_string()));
    scores.entry("Blue".to_string()).or_insert(50);
    scores.entry("Yellow".to_string()).or_insert(50);
    println!("{:?}", scores);

    // updating based on the old value uses entry() and or_insert() as well.
    // or_insert returns a &mut Value so it can be changed.
    let text = "a b c d a b d q";
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", counts);
}
