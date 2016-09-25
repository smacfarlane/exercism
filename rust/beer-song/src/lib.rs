pub fn verse(beers: i32) -> String {
    match beers {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n", beers, beers),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", beers, beers, beers - 1)
    }
}

pub fn sing(first_round: i32, last_round: i32) -> String {
    let mut song: Vec<String> =  Vec::new();

    for beer in (last_round..first_round+1).rev() {
        song.push(verse(beer));
    }

    song.join("\n")
}
