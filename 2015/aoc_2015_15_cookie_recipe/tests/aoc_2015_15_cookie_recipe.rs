extern crate aoc_2015_15_cookie_recipe;

// #[cfg(test)]
// mod tests {
    use aoc_2015_15_cookie_recipe::*;

    // #[test]
    // fn it_works() {
    //     let result = foobar();
    //     assert_eq!(result, 4 + 2);
    // }

    // #[test]
    // fn whatevs() {
    //     let result = main();
    //     println!("{:?}", result);
    //     assert_eq!(2, 2);
    // }

    #[test]
    fn test_make_cookies() {
        // let fred : i32 = "1,".parse().unwrap();
        let result = make_cookies(String::from("src/input0.txt"));
        assert!(result > 1);
    }
// }

#[test]
fn test_stdlib() {
    let v: Vec<i32> = vec![1, 2, 3].into_iter().map(|x| x + 1).rev().collect();

    assert_eq!(v, [4, 3, 2]);
}