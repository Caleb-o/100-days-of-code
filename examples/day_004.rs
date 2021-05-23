use std::str;

fn main()
{
    // A reference to a string that is allocated in read only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    println!("Words in reverse:");
    for word in pangram.split_whitespace().rev()
    {
        println!("> {}", word);
    }

    // Copy chars into a vector, sort and remove duplicates 
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    let mut string: String = String::new();
    for c in chars
    {
        string.push(c);
        string.push_str(", ");
    }

    // The trimmed string is a slice to the original string, hence no new
    // allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // Heap allocate a string
    let alice: String = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);

    // Using quotes
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // Byte strings
    let bytestring: &[u8; 21] = b"this is a byte string";

    // Byte arrays do not implement Display
    println!("A byte string: {:?}", bytestring);

    // Raw byte strings work just like byte strings
    let raw_bytestrings = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestrings);

    // Converting a byte array to 'str' can fail
    if let Ok(my_str) = str::from_utf8(raw_bytestrings)
    {
        println!("And the same as text: '{}'", my_str);
    }
}
