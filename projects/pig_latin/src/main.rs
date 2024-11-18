fn main() {
    println!("{}", convert_to_pig_latin("Hello world!"));
}

fn convert_to_pig_latin(input: &str) -> String {
    // store result
    let mut result = String::new();

    // work on individual words
    for word in input.split_whitespace() {
        // get first character
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();

        // check if vowel or not
        if is_vowel(first_char) {
            // append -hay to word
            result.push_str(word);
            result.push_str("-hay");
        } else {
            // move first char to end and append -ay
            for ch in chars {
                result.push(ch);
            }

            result.push('-');
            result.push(first_char);
            result.push_str("ay");
        }

        result.push(' ');
    }

    result.trim_end().to_string()
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false,
    }
}
