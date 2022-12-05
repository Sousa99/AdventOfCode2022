mod read;
mod lib;

fn main() {

    let input = read::read_lines("input.txt".to_owned());
    let input_formatted : Vec<lib::PairResponsabilities> = input.into_iter()
        .map(|line| {

            let mut split_elfs: Vec<String> = line.split(',')
                .map(|split| split.to_owned())
                .rev()
                .collect();
            
            let first_split: String = split_elfs.pop().unwrap();
            let second_split: String = split_elfs.pop().unwrap();

            let mut first_split_boundaries: Vec<lib::ResponsabilityBoundary> = first_split.split('-')
                .map(|boundary| boundary.parse().unwrap())
                .rev()
                .collect();
            let mut second_split_boundaries: Vec<lib::ResponsabilityBoundary> = second_split.split('-')
                .map(|boundary| boundary.parse().unwrap())
                .rev()
                .collect();

            let first_interval: lib::ResponsabilityInterval = lib::ResponsabilityInterval::new(first_split_boundaries.pop().unwrap(), first_split_boundaries.pop().unwrap());
            let second_interval: lib::ResponsabilityInterval = lib::ResponsabilityInterval::new(second_split_boundaries.pop().unwrap(), second_split_boundaries.pop().unwrap());

            return (first_interval, second_interval);

        }).collect();
    
    // Part 1
    let number_total_overlaps: usize = input_formatted.iter()
        .filter(|(first_interval, second_interval)| lib::detect_total_overlap(first_interval, second_interval))
        .count();
    println!("\r🏭 Number of total overlaps in elves work: '{}' (Part 1)", number_total_overlaps);
    
    // Part 2
    let number_partial_overlaps: usize = input_formatted.iter()
        .filter(|(first_interval, second_interval)| lib::detect_partial_overlap(first_interval, second_interval))
        .count();
    println!("\r🏭 Number of partial overlaps in elves work: '{}' (Part 2)", number_partial_overlaps);
}