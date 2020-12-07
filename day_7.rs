use crate::input::read_input_until_empty;
use std::collections::HashMap;
use std::borrow::Borrow;

#[derive(Clone, PartialEq, Eq)]
struct Bag {
    color: String,
    contain: Option<HashMap<String, u8>>,
}

pub fn day_7(part: i8) {
    let bags: Vec<Bag> = read_input();

    if part == 1 {
        part_one(bags);
    } else {
        part_two(bags);
    }
}

fn part_two(bags: Vec<Bag>) {
    println!("Total: {}", search_bag_count(&bags, &"shiny gold".to_string()));
}

fn search_bag_count(bags: &Vec<Bag>, color: &String) -> usize {
    let mut bag_count = 0;
    for bag in bags {
        if *color == bag.color {
            for rule in bag.contain.as_ref().unwrap().borrow(){
                bag_count += (*rule.1) as usize * (search_bag_count(bags, rule.0) + 1);
            }
            return bag_count;
        }
    }

    return bag_count;
}

fn part_one(bags: Vec<Bag>) {
    let mut all_containing: Vec<&Bag> = Vec::new();
    search_bags_containing(&bags, &mut all_containing, &"shiny gold".to_string());

    println!("Containing: {}", all_containing.len());
}

fn search_bags_containing<'a>(bags: &'a Vec<Bag>, bags_containing: &mut Vec<&'a Bag>, containing: &String) {
    for bag in bags {
        if bag.contain.is_none() {
            continue;
        }
        if bags_containing.contains(&bag) {
            continue;
        }

        for rules in bag.contain.as_ref().unwrap().borrow() {
            if rules.0.contains(containing) {
                bags_containing.push(bag);

                search_bags_containing(bags, bags_containing, &bag.color);
            }
        }
    }
}

fn read_input() -> Vec<Bag> {
    let input = read_input_until_empty();
    let mut bags = Vec::new();

    for line in input {
        let mut rule_bags: HashMap<String, u8> = HashMap::new();

        let split: Vec<&str> = line.split(" bags contain ").collect();
        let color: &str = split[0];
        let contain: Vec<&str> = split[1].split(", ").collect();

        for rule in contain {
            if rule.contains("no other bags") {
                // bags.push(Bag{color: color.to_string(), contain: None});
                break;
            }

            let rule_split: Vec<&str> = rule.split(" ").collect();
            let rule_number: u8 = rule_split[0].parse().unwrap();
            let mut rule_color = String::new();
            for i in 1..rule_split.len() - 1 {
                if rule_color.is_empty() {
                    rule_color += rule_split[i];
                } else {
                    rule_color += &*(" ".to_owned() + rule_split[i]);
                }
            }

            rule_bags.insert(rule_color, rule_number);
        }

        bags.push(Bag{
            color: color.parse().unwrap(),
            contain: Some(rule_bags)
        });
    }

    return bags;
}