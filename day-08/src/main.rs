#![feature(is_some_and)]

mod read;
mod lib;

fn main() {

    let input: Vec<Vec<u32>> = read::read_digits("input.txt".to_owned());
    let mut forest: lib::Forest = lib::Forest::new(input);

    // Part 1

    lib::check_visibility_outside_forest(&mut forest);
    let visible_trees: usize = forest.trees.iter()
        .filter(|&(_, tree)| tree.check_visible())
        .count();
    println!("\r🌲 Number of trees visible: '{}' (Part 1)", visible_trees);
    
    // Part 2

    lib::check_visibility_inside_forest(&mut forest);
    let best_scenic_score: (&lib::Coordinate, usize) = forest.trees.iter()
        .map(|(position, tree)| (position, tree.scenic_score()))
        .max_by_key(|&(_, scenic_score)| scenic_score)
        .unwrap();
    println!("\r🌲 Maximum scenic score registered for position '({}, {})': '{}' (Part 2)", best_scenic_score.0.x, best_scenic_score.0.y, best_scenic_score.1);
}