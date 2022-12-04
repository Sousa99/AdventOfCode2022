mod read;
mod lib;

fn main() {

    let input = read::read_lines("input.txt".to_owned());
    let input_formatted : Vec<(char, char)> = input.into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|line| (*line.get(0).unwrap(), *line.get(2).unwrap()))
        .collect();
    
    // Part 1
    let score_prediction_play: lib::Score = lib::play_game_setting_play(&input_formatted);
    println!("\r✂️  Prediction for score achieved by setting play: '{}' (Part 1)", score_prediction_play);
    
    // Part 2
    let score_prediction_outcome: lib::Score = lib::play_game_setting_outcome(&input_formatted);
    println!("\r✂️  Prediction for score achieved by setting outcome: '{}' (Part 2)", score_prediction_outcome);
}