fn main() {
    let s1 = String::from("Hello");
    println!("The length of s1 is {}", get_len(&s1));
    
    let sentence = String::from("This is a full sentence.");
    let resp = get_first_word(&sentence);
    println!("{resp}");
}

fn get_len(s: &String) -> usize {
    s.len()
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}