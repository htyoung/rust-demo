pub(crate) fn ownership_entry() {
    let s = String::from("hello world");
    takes_ownership(s);
    //println!("{}", s);
    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

pub(crate) fn ownership_entry1(some_string: String) {
    let s1=gives_ownership();
    let s2=String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

pub(crate) fn ownership_entry2() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is '{len}'");
}

pub(crate) fn ownership_entry3() {
    let mut s = String::from("hello");
    charge(&mut s);
    println!("{}", s);
}

pub(crate) fn ownership_entry4() {
    let mut s2 = String::from("hello");
    let word = first_word(&s2);
    //s2.clear();
    println!("the first word is: {word}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn charge(s: &mut String) {
    s.push_str(", world");
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn gives_ownership() -> String{
    let s1 = String::from("yours");
    s1
}

fn takes_and_gives_back(s: String) -> String {
    s
}
