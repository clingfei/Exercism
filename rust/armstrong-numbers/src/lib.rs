pub fn is_armstrong_number(mut num: u32) -> bool {
    let origin = num;
    let mut vec = Vec::new();
    while num > 0 {
        vec.push(num % 10);
        num /= 10;
    }
    let mut sum = 0;
    let mut i = 0;
    while i < vec.len() {
        sum += vec[i].pow(vec.len() as u32);
        i += 1;
    }
    return sum == origin;
}
