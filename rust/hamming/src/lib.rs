pub fn hamming_distance(left: &str, right: &str) -> Result<usize, usize> {
    match calculate_distance(left, right) {
        None => Err(0),
        Some(n) => Ok(n)
    }
}

fn calculate_distance(left: &str, right: &str) -> Option<usize> {
    if left.len() != right.len() {
        None
    } else {
        let iter = left.chars().zip(right.chars());

        let length = iter.filter(|&(a, b)| a != b).count();

        Some(length)
    }
}