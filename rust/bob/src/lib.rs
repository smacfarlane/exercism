pub fn reply(input: &str) -> &str {

    if input.is_empty() == true {
        "Fine. Be that way!"
    } else if input.chars().last() == Some('?') {
        "Sure."
    } else if input.to_uppercase() == input {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}