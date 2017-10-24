// return Ok(x) where x is the number of steps required to reach 1
//
pub fn collatz(n: u64) -> Result<u64, &'static str> {
    if n <= 0 {
        return Err("Must be greater than 0")
    }

    let mut count = 0; 
    let mut value = n;
    let is_even = |x: u64| x & 1 == 0;

    while value != 1 {
        match is_even(value) {
            true => { value = value / 2 }
            false => { value = (value * 3) + 1 }
        }
        count = count +1;
    }

    
    Ok(count)
}
