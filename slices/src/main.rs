fn main() {
    let s =
        String::from("Slices let you reference a contiguous sequence of elements in a collection.");
    println!("{}", s);
    println!("A slice is a kind of reference, so it does not have ownership.");
    let _word: &str = first_word(&s);
    //A string slice is a reference to a contiguous sequence of the elements of a String
    let s: String = String::from("hello world");
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    println!("{}", hello);
    println!("{}", world)
}

fn first_word(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        println!("i: {}, item: {}", i, item as char);
        if item == b' ' {
            return &s[0..i];
        }
    }
    return s;
}
