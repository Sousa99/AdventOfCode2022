mod read;
mod lib;

fn main() {

    let input = read::read_lines("input.txt".to_owned());
    let input_formatted : Vec<(lib::Compartment, lib::Compartment)> = input.into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|items| {

            let split_point: usize = items.len() / 2;
            let (first_compartment, second_compartment): (&[char], &[char]) = items.split_at(split_point);
            return (first_compartment.to_owned(), second_compartment.to_owned());

        }).collect();
    
    // Part 1
    let sum_priorities: lib::PriorityScore = input_formatted.iter()
        .map(|(first_compartment, second_compartment)| lib::priority_rugsack(first_compartment, second_compartment))
        .sum();
    println!("\r🎁 Priority rugsacks combined: '{}' (Part 1)", sum_priorities);
    
    // Part 2
    let sum_priorities_group: lib::PriorityScore = input_formatted.chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>()
        .into_iter()
        .map(|group_rugsacks| lib::priority_group_badge(group_rugsacks))
        .sum();
    println!("\r🎁 Priority group rugsacks combined: '{}' (Part 2)", sum_priorities_group);
}