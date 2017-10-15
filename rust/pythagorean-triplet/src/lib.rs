pub fn find() -> Option<usize> {
  let target = 1000;

  for a in 1..(target) {
    for b in (a)..(target-a) {
      let c = target - a - b;

      if a * a + b * b == c * c {
        return Some(a * b * c);
      }
    }
  }

  None
}