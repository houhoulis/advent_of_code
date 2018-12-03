extern crate aoc_2015_15_cookie_recipe;

// #[cfg(test)]
// mod tests {
    use aoc_2015_15_cookie_recipe::*;

#[test]
fn test_make_cookies_input0() {
    let result = make_cookies(String::from("src/input0.txt"));
    assert_eq!(result, 62842880);
}

#[test]
fn test_make_cookies_input1() {
    let result = make_cookies(String::from("src/input1.txt"));
    assert_eq!(result, 13882464);
}

#[test]
fn test_stdlib() {
    let v: Vec<i32> = vec![1, 2, 3].into_iter().map(|x| x + 1).rev().collect();

    assert_eq!(v, [4, 3, 2]);
}
