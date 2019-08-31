pub fn verse(n: i32) -> String {
    match n {
        0 => return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => return format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n, n),
        2 => return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n-1),
        3...99 => return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1),
        _ => return String::new(),
    }
}

pub fn sing(mut start: i32, end: i32) -> String {
    let mut result = String::new();
    while start >= end {
        if result.len() > 0 {
            result.push_str(&String::from("\n"));
        }
        result.push_str(&verse(start));
        start -= 1;
    }
    return result;
}
