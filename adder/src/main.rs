fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    let mut some_word = "hello world";
    some_word = "";
    println!("the first word is: {}", some_word);
    // println!("the first word is: {}", okay);

    println!("the first word is: {}", word);
    // s.clear(); // error!

    let mut s = String::from("i world");
    let wrd = first_word(&s);
    println!("the first word is: {}", s.len());
    // println!("the first word is: {}", s);
    // let hello = String::from("hello world");
    let hello = "hello world";
	let slice = &hello[..];
    println!("the first word is: {}", slice);
    // s.clear(); // error!
    println!("the first word is: {}", wrd);
    println!("the first word is: {}", s);
}
