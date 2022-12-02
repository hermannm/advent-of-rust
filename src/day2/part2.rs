pub fn solve_puzzle() -> Result<i32, String> {
    let input = include_str!("input.txt");

    input
        .lines()
        .map(|line| {
            let (enemy_choice, game_outcome) = parse_input_line(line)?;
            let our_choice = choose_from_outcome(&enemy_choice, &game_outcome);
            Ok(calculate_score(enemy_choice, our_choice))
        })
        .sum()
}

fn choose_from_outcome(enemy_choice: &GameChoice, game_outcome: &GameOutcome) -> GameChoice {
    use GameChoice::*;
    use GameOutcome::*;

    match (enemy_choice, game_outcome) {
        (Rock, Draw) | (Paper, Loss) | (Scissors, Win) => Rock,
        (Rock, Win) | (Paper, Draw) | (Scissors, Loss) => Paper,
        (Rock, Loss) | (Paper, Win) | (Scissors, Draw) => Scissors,
    }
}

fn parse_input_line(input_line: &str) -> Result<(GameChoice, GameOutcome), String> {
    let input_chars: Vec<char> = input_line.chars().collect();
    if input_chars.len() < 3 {
        return Err("Input line did not contain enough characters".to_string());
    }

    let enemy_choice = GameChoice::try_from(input_chars[0])?;
    let game_outcome = GameOutcome::try_from(input_chars[2])?;

    Ok((enemy_choice, game_outcome))
}

fn calculate_score(enemy_choice: GameChoice, our_choice: GameChoice) -> i32 {
    use GameChoice::*;

    let mut score = our_choice.score();

    score += match (enemy_choice, our_choice) {
        // Loss
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 0,
        // Draw
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        // Win
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
    };

    score
}

enum GameOutcome {
    Loss,
    Draw,
    Win,
}

impl TryFrom<char> for GameOutcome {
    type Error = String;

    fn try_from(outcome_char: char) -> Result<Self, Self::Error> {
        match outcome_char {
            'X' => Ok(GameOutcome::Loss),
            'Y' => Ok(GameOutcome::Draw),
            'Z' => Ok(GameOutcome::Win),
            _ => Err(format!("Invalid game outcome character: {outcome_char}")),
        }
    }
}

enum GameChoice {
    Rock,
    Paper,
    Scissors,
}

impl GameChoice {
    fn score(&self) -> i32 {
        match self {
            GameChoice::Rock => 1,
            GameChoice::Paper => 2,
            GameChoice::Scissors => 3,
        }
    }
}

impl TryFrom<char> for GameChoice {
    type Error = String;

    fn try_from(choice_char: char) -> Result<Self, Self::Error> {
        match choice_char {
            'A' => Ok(GameChoice::Rock),
            'B' => Ok(GameChoice::Paper),
            'C' => Ok(GameChoice::Scissors),
            _ => Err(format!("Invalid game choice character: {choice_char}")),
        }
    }
}
