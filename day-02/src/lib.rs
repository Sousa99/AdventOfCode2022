
// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================

pub type Score = usize;

#[derive(PartialEq, Eq, Clone, Copy)]
enum GameChoice {
    Rock,
    Paper,
    Scissors
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum GameResult {
    Win,
    Lose,
    Draw
}

// =============================================== AUXILIARY FUNCTIONS ===============================================

fn map_other_player_choice(play_char: &char) -> GameChoice {
    
    return match play_char {
        'A' => GameChoice::Rock,
        'B' => GameChoice::Paper,
        'C' => GameChoice::Scissors,
        _ => panic!("🚨 Symbol not recognized as an other player's game choice!")
    }
}

fn map_self_player_choice(play_char: &char) -> GameChoice {
    
    return match play_char {
        'X' => GameChoice::Rock,
        'Y' => GameChoice::Paper,
        'Z' => GameChoice::Scissors,
        _ => panic!("🚨 Symbol not recognized as a self player's game choice!")
    }
}

fn map_outcome(outcome_char: &char) -> GameResult {
    
    return match outcome_char {
        'X' => GameResult::Lose,
        'Y' => GameResult::Draw,
        'Z' => GameResult::Win,
        _ => panic!("🚨 Symbol not recognized as a game result!")
    }
}

fn play_match(self_play: GameChoice, opponent_play: GameChoice) -> GameResult {

    return match (self_play, opponent_play) {
        (GameChoice::Rock, GameChoice::Scissors) => GameResult::Win,
        (GameChoice::Paper, GameChoice::Rock) => GameResult::Win,
        (GameChoice::Scissors, GameChoice::Paper) => GameResult::Win,
        (GameChoice::Scissors, GameChoice::Rock) => GameResult::Lose,
        (GameChoice::Rock, GameChoice::Paper) => GameResult::Lose,
        (GameChoice::Paper, GameChoice::Scissors) => GameResult::Lose,

        (GameChoice::Rock, GameChoice::Rock) | (GameChoice::Paper, GameChoice::Paper) | (GameChoice::Scissors, GameChoice::Scissors) => GameResult::Draw
    }
}

fn predict_match(outcome: GameResult, opponent_play: GameChoice) -> GameChoice {

    return match (outcome, opponent_play) {
        (GameResult::Win, GameChoice::Rock) => GameChoice::Paper,
        (GameResult::Win, GameChoice::Paper) => GameChoice::Scissors,
        (GameResult::Win, GameChoice::Scissors) => GameChoice::Rock,
        (GameResult::Lose, GameChoice::Rock) => GameChoice::Scissors,
        (GameResult::Lose, GameChoice::Paper) => GameChoice::Rock,
        (GameResult::Lose, GameChoice::Scissors) => GameChoice::Paper,

        (GameResult::Draw, opponent_play) => opponent_play
    }
}

fn compute_round_score(play: GameChoice, outcome: GameResult) -> Score {

    let mut round_score = 0;

    // Intrepert user's play as points
    match play {
        GameChoice::Rock => round_score = round_score + 1,
        GameChoice::Paper => round_score = round_score + 2,
        GameChoice::Scissors => round_score = round_score + 3,
    }

    // Intrepert match outcome as points
    match outcome {
        GameResult::Win => round_score = round_score + 6,
        GameResult::Draw => round_score = round_score + 3,
        GameResult::Lose => round_score = round_score + 0,
    }

    return round_score;

}

pub fn play_game_setting_play(plays: &Vec<(char, char)>) -> Score {

    let mut final_score: Score = 0;
    for (play_other_char, play_self_char) in plays.iter() {

        let play_other: GameChoice = map_other_player_choice(play_other_char);
        let play_self: GameChoice = map_self_player_choice(play_self_char);
        let outcome: GameResult = play_match(play_self, play_other);

        let round_score: Score = compute_round_score(play_self, outcome);
        final_score = final_score + round_score;
    
    } 


    return final_score;
}

pub fn play_game_setting_outcome(plays: &Vec<(char, char)>) -> Score {

    let mut final_score: Score = 0;
    for (play_other_char, play_outcome_char) in plays.iter() {

        let play_other: GameChoice = map_other_player_choice(play_other_char);
        let outcome: GameResult = map_outcome(play_outcome_char);
        let play_self: GameChoice = predict_match(outcome, play_other);

        let round_score: Score = compute_round_score(play_self, outcome);
        final_score = final_score + round_score;
    
    } 


    return final_score;
}

// ================================================= IMPLEMENTATIONS =================================================

