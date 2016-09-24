pub fn raindrops(input: i32) -> String {
    let mut reply = String::new();

    let dictionary = vec![
      RainLookup { number: 3, word: "Pling".to_string() },
      RainLookup { number: 5, word: "Plang".to_string() },
      RainLookup { number: 7, word: "Plong".to_string() }
    ];

    for item in dictionary {
        if input % item.number == 0 {
            reply.push_str(&item.word);
        }
    }

    if reply.is_empty() == true {
        reply = input.to_string();
    }

    reply
}

pub struct RainLookup {
    number: i32,
    word: String
}
