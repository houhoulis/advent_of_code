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

    fn score_with_multipliers(&self, multipliers: Vec<i64>) -> i64 {
        if self.len() != multipliers.len() {
            panic!("Ouchie!");
        };
        let ing_with_multipliers: Vec<(&Ingredient, i64)> = self.ingredients.iter().zip(multipliers).collect();
        let cap: i64 = ing_with_multipliers.iter().map(|(ing, mult)| mult * ing.capacity).sum();
        let dur: i64 = ing_with_multipliers.iter().map(|(ing, mult)| mult * ing.durability).sum();
        let fla: i64 = ing_with_multipliers.iter().map(|(ing, mult)| mult * ing.flavor).sum();
        let tex: i64 = ing_with_multipliers.iter().map(|(ing, mult)| mult * ing.texture).sum();
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
    if recipe.len() == 2 {
        find_high_score_2(recipe)
    } else {
        find_high_score_4(recipe)
    }
}

fn find_high_score_2(recipe: Recipe) -> i64 {
    (0..100).map(|i|
        recipe.score_with_multipliers(vec![i, 100 - i])
    ).max().expect("No max score?!")
}

fn find_high_score_4(recipe: Recipe) -> i64 {
    let mut scores: Vec<i64> = Vec::new();
    for i in 0..=100 {
        for j in 0..=(100 - i) {
            for k in 0..=(100 - i - j) {
                let l = 100 - i - j - k;
                scores.push(recipe.score_with_multipliers(vec![i, j, k, l]))
            }
        }
    };
    println!("Better be better than 25s all around! ({})", recipe.score_with_multipliers(vec![25, 25, 25, 25]));

    scores.into_iter().max().expect("Wowzers! No/bad scores for max()??!")
}
