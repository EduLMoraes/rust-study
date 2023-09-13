#[warn(dead_code)]
pub fn to_int(string: String) -> i32{
    let int = string.trim().parse::<i32>().unwrap();
    return int
}
