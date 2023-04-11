// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a hint.



// Step 1.
// Complete the `capitalize_first` function.
// // "hello" -> "Hello"
// pub fn capitalize_first(input: &str) -> String {
//     let mut c = input.chars();
//     match c.next() {
//         None => String::new(),
//         Some(first) => ???,
//     }
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    let mut out = match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string(),
    };
    // out += &c.collect::<String>();
    out += c.as_str();
    out
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // let mut out:Vec<String> = vec![];
    // for item in words.to_owned(){
    //     out.push(capitalize_first(item))
    // }
    // out
    words.to_owned().into_iter().map(capitalize_first).collect::<Vec<String>>()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    // let mut out = String::new();
    // let words = capitalize_words_vector(words);
    // words.concat()
    // words.to_owned().into_iter().map(capitalize_first).collect::<Vec<String>>().concat()
    words.to_owned().into_iter().map(capitalize_first).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
