mod read;
mod lib;

use crate::lib::SnackCalories;
use crate::lib::Elf;
use crate::lib::convert_input_to_correct_format;

fn main() {

    let input = read::read_lines("input.txt".to_owned());
    let input_formatted : Vec<Vec<SnackCalories>> = convert_input_to_correct_format(input);

    let elfes : Vec<Elf> = input_formatted.into_iter()
        .map(|snacks| Elf::new(snacks))
        .collect();

    let mut sum_calories_by_elf : Vec<SnackCalories> = elfes.iter()
        .map(|elf| elf.get_total_snack_calories())
        .collect();
    sum_calories_by_elf.sort_by(|a, b| b.cmp(a));
    
    // Part 1
    let maximum_one_elf_sum: SnackCalories = *sum_calories_by_elf.first().unwrap();
    println!("\r🍗 Number of calories of top 1 Elves: '{}' (Part 1)", maximum_one_elf_sum);
    
    // Part 2
    let maximum_three_elf_sum: SnackCalories = sum_calories_by_elf[0..3]
        .iter()
        .sum();
    println!("\r🍗 Number of calories of top 3 Elves: '{}' (Part 2)", maximum_three_elf_sum);
}