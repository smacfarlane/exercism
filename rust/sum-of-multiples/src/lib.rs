#![feature(step_by)]

pub fn sum_of_multiples(max: i32, numbers: &Vec<i32>) -> i32 {
  let mut multiples: Vec<i32> = vec![];

  for &n in numbers {
    if n >= max {
      continue;
    }

    let mut result: Vec<_> = (n..max).step_by(n).collect();
    multiples.append(result.as_mut());      
  }

  multiples.sort();
  multiples.dedup();

  let result = multiples.iter().sum();


  result
}