use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "../input.txt";
    let input = File::open(path)?;
    let buf = BufReader::new(input);
    let mut elf = 0;
    let mut calories_total: i32 = 0;
    let mut elf_max: i32 = 0;
    let mut items: i32 = 0;
    let mut lines: i32 = 0;
    let mut calories: Vec<i32> = Vec::with_capacity(260);
    for line in buf.lines(){
        lines += 1;
        let unwrap = line?;
        println!("Elf #{elf}  with {items} item(s) that has {unwrap} calories with a total of {calories_total}, line {lines}");
        if unwrap.len() == 0 {
            if elf != 0 {
                if calories_total > calories[elf_max as usize] {
                    elf_max = elf;
                }
            }
            calories.push(calories_total);
            println!("Elf #{elf} brought {items} items with a total of {calories_total} calories");
            calories_total = 0;
            items = 0;
            elf += 1;
        } else {
            items += 1;
            calories_total += unwrap.parse::<i32>()?;
        }
    }
    let cal = calories[elf_max as usize];
    elf_max += 1;
    let cal_len = calories.len();
    println!("The elf with that brought the most calories is {elf_max} with {cal} calories and {cal_len} elves, total of {lines} lines");
    Ok(())
}

enum ElfCond {
    Done,

}



fn getMaxElf(elf: i32, value: String, store: i32) -> (i32, i32) {
    if value.len() == 0 {
        (elf, value)
    }
    let ret = store + unwrap.parse::<i32>()?;
    (elf, ret)
}
