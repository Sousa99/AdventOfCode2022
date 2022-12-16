mod read;
mod lib;

fn main() {

    let input: Vec<String> = read::read_lines("input.txt".to_owned());
    let rope_2: lib::Rope = lib::Rope::new(&input, 2);
    let rope_10: lib::Rope = lib::Rope::new(&input, 10);

    // Part 1

    let diff_positions_tail_2: usize = rope_2.get_tail_number_different_positions();
    println!("\r🌉 Number of positions visited by the 'tail' in a rope with '2' knots: '{}' (Part 1)", diff_positions_tail_2);
    
    // Part 2

    let diff_positions_tail_10: usize = rope_10.get_tail_number_different_positions();
    println!("\r🌉 Number of positions visited by the 'tail' in a rope with '10' knots: '{}' (Part 2)", diff_positions_tail_10);
    
}