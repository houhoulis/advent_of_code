use std::env;
use std::fs;

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i64, durability: i64, flavor: i64, texture: i64, calories: i64
}

impl Ingredient {
    fn multiply(&self, i: i64) -> Ingredient {
        Ingredient {
            name: format!("{} ({})", self.name, i),
            capacity: self.capacity * i,
            durability: self.durability * i,
            flavor: self.flavor * i,
            texture: self.texture * i,
            calories: self.calories * i
        }
    }
}

pub fn make_cookies(filename: String) -> i64 {
    let contents = read_file(filename);
    let ings = parse_ingredients(contents);
    let result: i64 = find_high_score(ings);
    println!("Found score! {}", result);
    result
}

fn read_file(filename: String) -> String {
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents
}

fn parse_ingredients(contents: String) -> Vec<Ingredient> {
    let lines = contents.lines();
    let ings: Vec<Ingredient> = lines
        .map(|line| parse_ingredient(line.to_string()))
        .collect();
    println!("hi {:?}", ings);
    ings
}

fn parse_ingredient(text: String) -> Ingredient {
    let words: Vec<&str> = text.split_whitespace().collect();

    let name = words[0].trim_end_matches(":").to_string();
    let capacity = words[2].trim_end_matches(",").parse::<i64>().unwrap();
    let durability = words[4].trim_end_matches(",").parse::<i64>().unwrap();
    let flavor = words[6].trim_end_matches(",").parse::<i64>().unwrap();
    let texture = words[8].trim_end_matches(",").parse::<i64>().unwrap();
    let calories = words[10].parse::<i64>().unwrap();

    Ingredient {
        name, capacity, durability, flavor, texture, calories
    }
}

fn find_high_score(ingredients: Vec<Ingredient>) -> i64 {
    let result = if ingredients.len() == 2 {
        find_high_score_2(ingredients)
    } else {
        find_high_score_4(ingredients)
    };
    result
}

fn score(ingredients: Vec<Ingredient>) -> i64 {
    let cap: i64 = ingredients.iter().map(|ing| ing.capacity).sum();
    let dur: i64 = ingredients.iter().map(|ing| ing.durability).sum();
    let fla: i64 = ingredients.iter().map(|ing| ing.flavor).sum();
    let tex: i64 = ingredients.iter().map(|ing| ing.texture).sum();
    if cap < 0 || dur < 0 || fla < 0 || tex < 0 {
        return 0;
    };
    cap * dur * fla * tex
}

fn find_high_score_2(ingredients: Vec<Ingredient>) -> i64 {
    (0..100).map(|i|
        score(vec![
            ingredients[0].multiply(i),
            ingredients[1].multiply(100 - i)
        ])
    ).max().expect("whatta")
}

fn find_high_score_4(ingredients: Vec<Ingredient>) -> i64 {
    let mut scores: Vec<i64> = Vec::new();
    for i in 0..=100 {
        for j in 0..=(100-i) {
            for k in 0..=(100-i-j) {
                let l = 100 - i - j - k;
                scores.push(score(vec![
                    ingredients[0].multiply(i),
                    ingredients[1].multiply(j),
                    ingredients[2].multiply(k),
                    ingredients[3].multiply(l),
                ]));
            }
        }
    };
    println!("Better be better than 25s all around ({})", score(vec![
        ingredients[0].multiply(25),
        ingredients[1].multiply(25),
        ingredients[2].multiply(25),
        ingredients[3].multiply(25),
    ]));

    scores.into_iter().max().expect("wowzers")
}
