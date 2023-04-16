use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("====== Day 1 ======");
    let day1_result = day_one();
    if day1_result.is_err() {
        println!("Day 1 failed");
    }
    println!("====== Day 2 ======");
    let day2_result = day_two();
    if day2_result.is_err() {
        println!("Day 2 failed");
    }
    println!("====== Day 3 ======");
    let day3_result = day_three();
    if day3_result.is_err() {
        println!("Day 3 failed");
    }
    println!("====== Day 4 ======");
    let day4_result = day_four();
    if day4_result.is_err() {
        println!("Day 4 failed");
    }
    Ok(())
}

#[derive(Debug)]
pub struct Elf {
    calories: i32,
    num: i32,
    rps: RPS
}

impl Default for Elf {
    fn default() -> Self {
        Elf { calories: 0, num: 0, rps: RPS::default() }
    }
}

#[derive(Debug)]
pub struct RPS {
    round_shapes: [RPSShape;3],
    round_outcomes: [RPSRoundOutcome; 3]
}

impl Default for RPS {
    fn default() -> Self {
        RPS { round_shapes: [RPSShape::NONE,RPSShape::NONE,RPSShape::NONE], round_outcomes: [RPSRoundOutcome::NONE,RPSRoundOutcome::NONE,RPSRoundOutcome::NONE] }
    }
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
pub enum RPSShape {
    ROCK,       // A, X
    PAPER,      // B, Y
    SCISSORS,   // C, Z
    NONE
}

// Set up a default enum value
impl Default for RPSShape {
    fn default() -> Self {
        RPSShape::NONE
    }
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
pub enum RPSRoundOutcome {
    WIN,
    LOSS,
    DRAW,
    NONE
}
impl Default for RPSRoundOutcome {
    fn default() -> Self {
        RPSRoundOutcome::NONE
    }
}

fn fetch_contents(day_input: &str) -> String {
    let mut f = File::open(format!("input/day_{}_input.txt", day_input));
    if f.is_err() {
        // TODO: Better handle the error
        return String::new();
    }

    let file = f.as_mut().unwrap();
    let mut contents = String::new();
    
    let result = file.read_to_string(&mut contents);
    if result.is_err() {
        print!("Failed to read file contents");
        return String::new();
    }

    return contents;
}

fn day_one() -> std::io::Result<()> {
    // Check our supplies
    
    let elf_supplies = fetch_contents("one");
    // Organize the groupings
    let organized_supplies:Vec<&str> = elf_supplies.split("\n\n").collect();

    let mut elves:Vec<Elf> = Vec::new();
    for (i, elf_pack) in organized_supplies.iter().enumerate() {

        // Itemize each of the calories
        let calories_raw: Vec<&str> = elf_pack.split("\n").collect();

        let mut calories: i32 = 0;        
        for cal_raw in calories_raw {
            let calorie:i32 = cal_raw.parse::<i32>().unwrap();
            calories += calorie
        }
        
        let elf = Elf{
            calories: calories,
            num: (i+1) as i32 ,
            rps: RPS::default(),
        };
        elves.append(&mut vec![elf]);
    }

    // Reverse sort: largest first
    elves.sort_by_key(|e: &Elf| -e.calories);

    println!("Elf with most cals to spare is: {:?} with {:?} cals", elves[0].num, elves[0].calories);
    Ok(())
}


fn get_total(rps: RPS) -> i32 {
    let mut total = 0;
    for (pos, outcome) in rps.round_outcomes.iter().enumerate() {
        let out = *outcome;

        if out == RPSRoundOutcome::WIN {
            total += 6;
        } else if out == RPSRoundOutcome::DRAW {
            total += 3;
        } else if out == RPSRoundOutcome::LOSS {
            total += 0;
        }

        let round_shape = rps.round_shapes[pos];
        if round_shape == RPSShape::ROCK {
            total += 1;
        } else if round_shape == RPSShape::PAPER {
            total += 2;
        } else if round_shape == RPSShape::SCISSORS {
            total += 3;
        }
    }
    return total;
}

fn determine_outcome(op_shape: RPSShape, u_shape: RPSShape) -> RPSRoundOutcome {
    if op_shape == u_shape {
        return RPSRoundOutcome::DRAW;
    } else if (u_shape == RPSShape::ROCK && op_shape == RPSShape::SCISSORS) 
    || (u_shape == RPSShape::SCISSORS && op_shape == RPSShape::PAPER) 
    || (u_shape == RPSShape::PAPER && op_shape == RPSShape::ROCK) {
        return RPSRoundOutcome::WIN;
    } else {
        return RPSRoundOutcome::LOSS;
    }
}

fn day_two() -> std::io::Result<()> {
    // Rock Paper Scissors -> R | S = W, S | P = W, P | R = W, X | Y = D
    let contents = fetch_contents("two");
    let rounds:Vec<&str> = contents.split("\n").collect();
    let mut me = Elf::default();

    for (pos, round_content) in rounds.iter().enumerate() {
        let choices: Vec<&str> = round_content.split(" ").collect();
        let op_choice = choices[0];
        let choice = choices[1];
        
        let shape: RPSShape;
        if choice == "X" {
            shape = RPSShape::ROCK;
        } else if choice == "Y" {
            shape = RPSShape::PAPER;
        } else {
            shape = RPSShape::SCISSORS;
        }

        let op_shape: RPSShape;
        if op_choice == "A" {
            op_shape = RPSShape::ROCK;
        } else if op_choice == "B" {
            op_shape = RPSShape::PAPER;
        } else {
            op_shape = RPSShape::SCISSORS;
        }

        me.rps.round_shapes[pos] = shape;
        me.rps.round_outcomes[pos] = determine_outcome(op_shape, shape)
    }
    
    println!("Using the guide the score will be: {}", get_total(me.rps));
    Ok(())
}

#[derive(Debug)]
#[derive(Default)]
pub struct Napsack {
    common_string: String,
    priority: u32,
}

fn determine_common_char(p1: &str, p2:&str) -> String {

    let mut t = String::new();
    let s: Vec<&str> = p1.split("").collect();
    for (_i, c) in s.iter().enumerate() {
        // Skip the empty characters
        if c == &"" {
            continue;
        }

        let p2s:Vec<&str> = p2.split("").collect();
        for (_j, c2) in p2s.iter().enumerate() {
            if c2 == &"" {
                continue;
            }
            
            if *c == *c2 {
                t = c.to_string();
            }
        }
    }

    return t;
}

fn determine_priority(character: &str) -> u32 {
    if character == "" {
        return 0
    }
    
    let mut v = 0;

    for c in character.chars() {
        let adjustment: u32;
        if c.is_ascii_lowercase() { 
            adjustment = 96;
        } else {
            adjustment = 38;
        }
        v = c as u32 - adjustment;
    }

    return v;
}
fn day_three() ->  std::io::Result<()> { 
    let contents = fetch_contents("three");
    let napsacks_contents:Vec<&str> = contents.split("\n").collect();

    let mut bags:[Napsack;6] = [Napsack::default(),Napsack::default(),Napsack::default(),Napsack::default(),Napsack::default(),Napsack::default()];
    for (pos, items) in napsacks_contents.iter().enumerate() {
        let n = items.len();
        let parts = items.split_at(n/2);
        let common_string = determine_common_char(parts.0, parts.1);

        let priority =determine_priority(common_string.as_str());
        bags[pos] = Napsack {
            common_string: common_string,
            priority: priority
        }
    }

    let mut total_priority = 0;
    for bag in bags {
        total_priority += bag.priority;
        println!("Priority item type: {}({})",bag.priority, bag.common_string)
    }

    println!("Total sum priority: {}", total_priority);

    Ok(())
}

fn day_four() -> std::io::Result<()> {
    // .234.....  2-4
    // .....678.  6-8
    //
    // .23......  2-3
    // ...45....  4-5
    //
    // ....567..  5-7
    // ......789  7-9
    //
    // .2345678.  2-8
    // ..34567..  3-7
    //
    // .....6...  6-6
    // ...456...  4-6
    //
    // .23456...  2-6
    // ...45678.  4-8

    Ok(())
}
