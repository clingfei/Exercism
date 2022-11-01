pub fn verse(n: u32) -> String {
    if n == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
    } else if n == 1 {
        return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string();
    } else if n == 2 {
        return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n - 1);
    } else {
        return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1);

    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut res = "".to_string();
    let mut i = start;
    res += &verse(start);
    while i > end && i > 0 {
        res += &"\n".to_string();
        res += &verse(i - 1);
        i -= 1;
    }
    res
}
