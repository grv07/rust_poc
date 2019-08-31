pub fn square_of_sum(mut n: i32) -> i32 {
    let mut sum:i32 = 0;
    while n > 0{
        sum += n;
        n -= 1;
    }
    sum*sum
}

pub fn sum_of_squares(mut n: i32) -> i32 {
    let mut sum_sq:i32 = 0;
    while n > 0 {
        sum_sq += n*n;
        n -= 1;
    }
    sum_sq
}

pub fn difference(n: i32) -> i32 {
    square_of_sum(n) - sum_of_squares(n)
}
