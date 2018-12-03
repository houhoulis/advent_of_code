pub fn foobar() -> i32 {
    println!("Hello, world!");
    6
}

use std::env;

pub fn main() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    println!("passed in: {:?}", args);
    args
    // let arg: String = args.first();
}

use std::fs;

pub fn make_cookies(filename : String) -> i32 {
    let contents = read_file(filename);
    let ings = parse_ingredients(contents);
    3
}

fn read_file(filename : String) -> String {
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n------------------------\n{}\n", contents);
    contents
}

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32
}

fn parse_ingredients(contents : String) -> Vec<Ingredient> {
    let ing = Ingredient {
        name: String::from("Hi"), capacity: 2, durability: 2, flavor: 2, texture: 2, calories: 2
    };
    let lines = contents.lines();
    // for element in lines {
    //     println!("raw line: {}", element);
    // }
    let ings : Vec<Ingredient> = lines.map(|line| parse_ingredient(line.to_string()) )
                    .collect();
    println!("hi {:#?}", ings);
    ings
}

fn parse_ingredient(text : String) -> Ingredient {
    let words : Vec<&str> = text.split_whitespace().collect();

    let name = words[0].trim_end_matches(":").to_string();
    let capacity = words[2].trim_end_matches(",").parse::<i32>().unwrap();
    let durability = words[4].trim_end_matches(",").parse::<i32>().unwrap();
    let flavor = words[6].trim_end_matches(",").parse::<i32>().unwrap();
    let texture = words[8].trim_end_matches(",").parse::<i32>().unwrap();
    let calories = words[10].parse::<i32>().unwrap();

    Ingredient {
        name, capacity, durability, flavor, texture, calories
    }
}
