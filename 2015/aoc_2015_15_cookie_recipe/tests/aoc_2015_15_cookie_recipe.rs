extern crate aoc_2015_15_cookie_recipe;

use aoc_2015_15_cookie_recipe::*;

#[test]
fn test_make_cookies_input0() {
    let result = make_cookies(String::from("src/input0.txt"));
    assert_eq!(result, Some(62842880));
}

#[test]
fn test_make_cookies_input1() {
    let result = make_cookies(String::from("src/input1.txt"));
    assert_eq!(result, Some(13882464));
}

#[test]
fn test_stdlib() {
    let v: Vec<i32> = vec![1, 2, 3].into_iter().map(|x| x + 1).rev().collect();

    assert_eq!(v, [4, 3, 2]);
}

#[test]
fn test_cookies_with_calories_input0_500() {
    let result = cookies_with_calories(String::from("src/input0.txt"), 500);
    assert_eq!(result, Some(57600000));
}

#[test]
fn test_cookies_with_calories_input1_500() {
    let result = cookies_with_calories(String::from("src/input1.txt"), 500);
    assert_eq!(result, Some(11171160));
}

#[test]
fn test_cookies_with_negative_calories() {
    let result = cookies_with_calories(String::from("src/input1.txt"), -1);
    assert_eq!(result, None);
}

/////////////////////////////////

fn add_to_inverse(int: i32) -> i32 {
    let inverse = 1 / int;
    int + inverse
}

#[test]
// #[should_panic(expected = "Guess value must be less than or equal to 100")]
#[should_panic(expected = "divide by zero")]
fn test_asserting_should_panic() {
    add_to_inverse(0);
}
