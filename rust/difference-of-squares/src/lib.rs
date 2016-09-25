pub fn sum_of_squares(value: i32) -> i32 {
    let numbers: Vec<i32> = (1..value+1).collect();

    numbers.iter()
        .fold(0, |acc, &x| acc + x * x)
}

pub fn square_of_sum(value: i32) -> i32 {
    let numbers: Vec<i32> = (1..value+1).collect();
    numbers.iter()
        .fold(0, |acc, x| acc + x)
        .pow(2)
}

pub fn difference(value: i32) -> i32 {
    square_of_sum(value) - sum_of_squares(value)
}