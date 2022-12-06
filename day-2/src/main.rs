use std::fs;

// Stores the decoded score for each move (Rock, Paper, Scissors)
struct Move {
    score: i32,
}

impl Move {
    pub fn new(encoded_move: char) -> Self {
        let decoded_score;
        match encoded_move {
            'A' | 'X' => decoded_score = 1,
            'B' | 'Y' => decoded_score = 2,
            'C' | 'Z' => decoded_score = 3,
            _ => decoded_score = 0,
        }

        Move {
            score: decoded_score,
        }
    }
}

// Stores the decoded score for each outcome (win, loss, draw)
struct Outcome {
    score: i32,
}

impl Outcome {
    pub fn new(encoded_move: char) -> Self {
        let decoded_score;
        match encoded_move {
            'X' => decoded_score = 0,
            'Y' => decoded_score = 3,
            'Z' => decoded_score = 6,
            _ => decoded_score = 0,
        }

        Outcome {
            score: decoded_score,
        }
    }
}

fn main() {
    // Part 1
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut part1_total = 0;
    for line in content.lines() {
        //each line is one round of the game
        let round: Vec<_> = line.chars().collect();
        // decrypt the opponents move
        let opponent = Move::new(round[0]);
        // decrypt your move
        let you = Move::new(round[2]);
        // look up score for that round result
        let round_score = result_score(&you, opponent);
        // add round result score to the move score
        part1_total += round_score + you.score;
    }
    println!("Part 1 Total Score: {}", part1_total);

    //Part 2
    let mut part2_total = 0;
    for line in content.lines() {
        let round: Vec<_> = line.chars().collect();
        // decypt opponents move
        let opponent = Move::new(round[0]);
        // decrypt outcome
        let outcome = Outcome::new(round[2]);
        // look up move score based on the requested outcome
        let move_score = move_score(opponent, &outcome);
        // add round result and move score to total
        part2_total += move_score + outcome.score;
    }

    println!("Part 2 Total: {}", part2_total);
}

fn result_score(you: &Move, opponent: Move) -> i32 {
    // Draw
    if you.score == opponent.score {
        return 3;
    }
    // Rock beats Scissors
    if you.score == 1 && opponent.score == 3 {
        return 6;
    }
    if you.score == 3 && opponent.score == 1 {
        return 0;
    }
    // Paper beats Rock, Scissors beats Paper
    if you.score > opponent.score {
        return 6;
    }
    if you.score < opponent.score {
        return 0;
    }
    // Oops.
    return 10000000;
}

fn move_score(opponent: Move, outcome: &Outcome) -> i32 {
    if outcome.score == 0 {
        match opponent.score {
            1 => return 3,
            2 => return 1,
            3 => return 2,
            _ => return 0,
        }
    }

    if outcome.score == 3 {
        return opponent.score;
    }

    if outcome.score == 6 {
        match opponent.score {
            1 => return 2,
            2 => return 3,
            3 => return 1,
            _ => return 0,
        }
    }
    // Oops.
    return 10000000;
}
