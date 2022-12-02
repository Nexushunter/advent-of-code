

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Elf {
    calories: i32,
    num: i32
}

fn main() -> std::io::Result<()> {
    // Check our supplies
    let mut f = File::open("src/puzzle_input.txt")?;
    let mut elf_supplies = String::new();
    
    f.read_to_string(&mut elf_supplies)?;

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
        };
        elves.append(&mut vec![elf]);
    }

    // Reverse sort: largest first
    elves.sort_by_key(|e: &Elf| -e.calories);

    println!("Elf with most cals to spare is: {:?} with {:?} cals", elves[0].num, elves[0].calories);
    Ok(())
}