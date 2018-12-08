use std::fs;

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i64, durability: i64, flavor: i64, texture: i64, calories: i64
}

#[derive(Debug)]
struct Recipe {
    ingredients: Vec<Ingredient>
}

impl Recipe {
    fn len(&self) -> usize {
        self.ingredients.len()
    }

    fn score_with_multipliers(&self, multipliers: &Vec<i64>) -> i64 {
        if self.len() != multipliers.len() {
            panic!("Ouchie!");
        };
        let ing_with_multipliers: Vec<(&Ingredient, &i64)> = self.ingredients.iter().zip(multipliers).collect();
        let cap: i64 = ing_with_multipliers.iter().map(|(ing, &mult)| mult * ing.capacity).sum();
        let dur: i64 = ing_with_multipliers.iter().map(|(ing, &mult)| mult * ing.durability).sum();
        let fla: i64 = ing_with_multipliers.iter().map(|(ing, &mult)| mult * ing.flavor).sum();
        let tex: i64 = ing_with_multipliers.iter().map(|(ing, &mult)| mult * ing.texture).sum();
        if cap < 0 || dur < 0 || fla < 0 || tex < 0 {
            return 0;
        };
        cap * dur * fla * tex
    }
}

pub fn make_cookies(filename: String) -> i64 {
    let contents = read_file(filename);
    let recipe = parse_ingredients(contents);
    let result: i64 = find_high_score(recipe);
    println!("Found score! {}", result);
    result
}

fn read_file(filename: String) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn parse_ingredients(contents: String) -> Recipe {
    let lines = contents.lines();
    let ingredients: Vec<Ingredient> = lines
        .map(|line| parse_ingredient(line.to_string()))
        .collect();
    println!("Ingredients that are going into recipe: {:?}", ingredients);
    Recipe { ingredients }
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

fn find_high_score(recipe: Recipe) -> i64 {
    find_high_score_rec(&recipe, &mut vec![])
}

fn find_high_score_rec(recipe: &Recipe, multipliers: &mut Vec<i64>) -> i64 {
    let new_vec = |vec: &Vec<i64>, elt: i64| {
        let mut a_new_vec = vec.clone();
        a_new_vec.push(elt);
        a_new_vec
    };
    let count = multipliers.len();
    let recipe_length = recipe.len();
    if count >= recipe_length {
        panic!("I miscounted the number of ingredients!!");
    } else if count == recipe_length - 1 {
        let total: i64 = multipliers.iter().sum();
        multipliers.push(100 - total);
        recipe.score_with_multipliers(multipliers)
    } else {
        let total: i64 = multipliers.iter().sum();
        (0..=(100 - total)).map(|i|
            find_high_score_rec(recipe, &mut new_vec(multipliers, i))
        ).max().expect("Out of bounds or empty?")
    }
}

