use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct FoodItem {
    calories: u32,
}

#[derive(Debug)]
struct Elf {
    food_items: Vec<FoodItem>,
}

impl Elf {
    fn total_calories(&self) -> u32 {
        self.food_items.iter().map(|item| item.calories).sum()
    }
}

fn main() {
    // read file and print line
    let file = File::open("src/sample.txt").unwrap();
    let reader = BufReader::new(file);
    let mut elves = vec![];
    let mut elf = Elf {
        food_items: vec![]
    };
    for (_, line) in reader.lines().enumerate() {
        // check if empty line
        let val = line.unwrap();
        if val.is_empty() {
            elves.push(elf);
            elf = Elf {
                food_items: vec![]
            };
            continue;
        } else {
            let line = val;
            // Show the line and its number.
            println!("{}", line);
            let food_item = FoodItem {
                calories: line.parse().unwrap(),
            };
            elf.food_items.push(food_item)
        }
    }
    elves.push(elf);
    
    // sort elves by highest calorie count
    elves.sort_by(|a, b| b.total_calories().cmp(&a.total_calories()));


    // print highest calorie count
    println!("highest calorie count: {}", elves[0].total_calories());

    let top_three = elves[0].total_calories() +
     elves[1].total_calories() +
     elves[2].total_calories();
    
    // print top 3
    println!("Top 3 elves calorie count is {}", top_three.to_string());

}