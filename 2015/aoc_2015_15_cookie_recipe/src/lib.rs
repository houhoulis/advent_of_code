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

    fn score_with_multipliers(&self, multipliers: &Vec<i64>, calorie_limit: Option<i64>) -> Option<i64> {
        if self.len() != multipliers.len() {
            panic!("Proportion count != ingredient count!");
        };
        let ing_with_multipliers: Vec<(&Ingredient, &i64)> = self.ingredients.iter().zip(multipliers).collect();
        if let Some(limit) = calorie_limit {
            let cal: i64 = ing_with_multipliers.iter().map(|(ing, &mult)| mult * ing.calories).sum();
            if cal != limit {
                return None;
            }
        };
        let cap: i64 = ing_with_multipliers.iter().map(|(ing, &mult)| mult * ing.capacity).sum();
        let dur: i64 = ing_with_multipliers.iter().map(|(ing, &mult)| mult * ing.durability).sum();
        let fla: i64 = ing_with_multipliers.iter().map(|(ing, &mult)| mult * ing.flavor).sum();
        let tex: i64 = ing_with_multipliers.iter().map(|(ing, &mult)| mult * ing.texture).sum();
        if cap < 0 || dur < 0 || fla < 0 || tex < 0 {
            return Some(0);
        };
        Some(cap * dur * fla * tex)
    }
}

pub fn make_cookies(filename: String) -> Option<i64> {
    let contents = read_file(filename);
    let recipe = parse_ingredients(contents);
    let result: Option<i64> = find_high_score(recipe, None);
    println!("Found score! {:?}", result);
    result
}

pub fn cookies_with_calories(filename: String, calorie_limit: i64) -> Option<i64> {
    let contents = read_file(filename);
    let recipe = parse_ingredients(contents);
    let result: Option<i64> = find_high_score(recipe, Some(calorie_limit));
    println!("Found score! {:?}", result);
    result
}

fn read_file(filename: String) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file.")
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

fn find_high_score(recipe: Recipe, calorie_limit: Option<i64>) -> Option<i64> {
    find_high_score_recursive(&recipe, &mut vec![], calorie_limit)
}

fn find_high_score_recursive(recipe: &Recipe, multipliers: &mut Vec<i64>, calorie_limit: Option<i64>) -> Option<i64> {
    let new_vec = |vec: &Vec<i64>, elt: i64| {
        let mut a_new_vec = vec.clone();
        a_new_vec.push(elt);
        a_new_vec
    };
    let count = multipliers.len();
    let recipe_length = recipe.len();
    if count >= recipe_length {
        panic!("Proportion count != ingredient count!");
    } else if count == recipe_length - 1 {
        let total: i64 = multipliers.iter().sum();
        multipliers.push(100 - total);
        recipe.score_with_multipliers(multipliers, calorie_limit)
    } else {
        let total: i64 = multipliers.iter().sum();
        (0..=(100 - total)).filter_map(|i|
            find_high_score_recursive(recipe, &mut new_vec(multipliers, i), calorie_limit)
        ).max()
    }
}
