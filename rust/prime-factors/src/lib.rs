pub fn factors(input: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut target = input;
    let mut factor = 2;

    while factor <= target {
        if target % factor == 0 {
            factors.push(factor);
            target = target / factor;
        } else {
            factor = factor + 1;
        }
    }

    factors
}
