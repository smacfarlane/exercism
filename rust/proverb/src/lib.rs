pub fn build_proverb(input: Vec<&str>) -> String {
    let mut response = Vec::new();
    let items = input.len();

    if items == 0 {
        return String::new()
    }

    for items in input.windows(2) {
        let line = format!("For want of a {} the {} was lost.", items.first().unwrap(), items.last().unwrap());
        response.push(line);
    }

    let last_line = match items {
        1 => "And all for the want of a nail.",
        2 => "And all for the want of a nail.",
        _ => "And all for the want of a horseshoe nail.",
    };
    response.push(last_line.to_string());

    response.join("\n")
}

