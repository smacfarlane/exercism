//
// See Rust Language Specific Instructions
// below normal exercise description.
//

pub fn encode(input: u64) -> String {
    if input == 0 {
        return "zero".to_string();
    }

    let mut result = Vec::new();

    for (index, item) in break_apart(input).iter().enumerate() {
        if let Some(word) = chunk_to_word(*item) {
            let counter = index_to_counter(index);
            let phrase = format!("{} {}", word, counter).trim().to_string();
            result.push(phrase);
        }
    }   
    result.reverse();
    result.join(" ")
}


fn break_apart(input: u64) -> Vec<u64> {
    let mut value = input;
    let mut chunks = Vec::new();

    while value > 0 {
        let chunk = value % 1000;
        value = (value - chunk) / 1000;

        chunks.push(chunk)
    }

    chunks
}

fn chunk_to_word(input: u64) -> Option<String> {
    match value_to_word(input) {
        Some(word) => Some(word),
        None => {
            let ones = input % 10; 
            let tens = (input - ones) % 100;
            let hundreds = (input - tens - ones) / 100;
            
            let mut phrase = Vec::new();
            if let Some(hundreds) = value_to_word(hundreds) {
                phrase.push(format!("{} hundred", hundreds));
            }

            if let Some(tens) = value_to_word(tens) {
                if let Some(ones) = value_to_word(ones) {
                    phrase.push(format!("{}-{}", tens, ones));
                } else {
                    phrase.push(tens);
                }
            } else if let Some(ones) = value_to_word(ones) {
                phrase.push(ones)
            }

            if phrase.is_empty() {
                None
            } else {
                Some(phrase.join(" ").to_string())
            }
        }
    }

}

fn value_to_word(input: u64) -> Option<String> {
    match input {
       1 => Some("one".to_string()),
       2 => Some("two".to_string()),
       3 => Some("three".to_string()),
       4 => Some("four".to_string()),
       5 => Some("five".to_string()),
       6 => Some("six".to_string()),
       7 => Some("seven".to_string()),
       8 => Some("eight".to_string()),
       9 => Some("nine".to_string()),
       10 => Some("ten".to_string()),
       11 => Some("eleven".to_string()),
       12 => Some("twelve".to_string()),
       13 => Some("thirteen".to_string()),
       14 => Some("fourteen".to_string()),
       15 => Some("fifteen".to_string()),
       16 => Some("sixteen".to_string()),
       17 => Some("seventeen".to_string()),
       18 => Some("eighteen".to_string()),
       19 => Some("nineteen".to_string()),
       20 => Some("twenty".to_string()),
       30 => Some("thirty".to_string()),
       40 => Some("forty".to_string()),
       50 => Some("fifty".to_string()),
       60 => Some("sixty".to_string()),
       70 => Some("seventy".to_string()),
       80 => Some("eighty".to_string()),
       90 => Some("ninety".to_string()),
       _ => None
   }
}

fn index_to_counter(input: usize) -> String {
    match input {
        1 => "thousand",
        2 => "million",
        3 => "billion",
        4 => "trillion",
        5 => "quadrillion",
        6 => "quintillion",
        _ => ""
    }.to_string()
}

