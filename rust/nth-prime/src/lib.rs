pub fn nth(n: usize) -> Result<usize, String> { 
  if n == 0 {
    return Err("Nope".to_string());
  }

  let mut primes = Vec::new();
  primes.push(2);
  
  let mut next = 3;
  while primes.len() < n {
    let divisible_by: Vec<usize> = primes.clone().into_iter().map(|n| next % n).filter(|&n| n == 0).collect();

    if divisible_by.is_empty() { 
      primes.push(next);
    }

    next = next + 2;
  }
  
  Ok(primes.pop().unwrap())
}
