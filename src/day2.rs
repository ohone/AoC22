#[derive(PartialEq, Copy, Clone)]
pub enum Play{
    Rock,
    Paper,
    Scissors
}

impl Play {
    fn from_i8(value: i8) -> Play {
        match value {
            0 => Play::Rock,
            1 => Play::Paper,
            2 => Play::Scissors,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Copy, Clone)]
enum Outcome{
    Win,
    Draw,
    Loss
}

fn get_score(outcome: Outcome, play: Play) -> i32{
    let shape_score = match play{
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3
    };
    let outcome_score = match outcome{
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6
    };

    return shape_score + outcome_score;
}

fn required_move(play: Play, outcome: Outcome) -> Play {
    let move_index = play as i8;
    return match outcome{
        Outcome::Draw => play,
        Outcome::Win => Play::from_i8((move_index + 1).rem_euclid(3)),
        Outcome::Loss => Play::from_i8((move_index - 1).rem_euclid(3))
    }
}

fn play_to_outcome(play: &Play) -> Outcome{
    return match play {
        Play::Rock => Outcome::Loss,
        Play::Paper => Outcome::Draw,
        Play::Scissors => Outcome::Win
    }
}

fn parse_move(input: &str) -> Play {
    return match input{
        "A" | "X" => Play::Rock,
        "B" | "Y" => Play::Paper,
        "C" | "Z" => Play::Scissors,
        _ => {panic!("no!");}
    } 
}

fn outcome(plays: &(Play, Play)) -> Outcome{
    return match plays{
        (opp, me) if opp == me => Outcome::Draw,
        (Play::Rock, Play::Paper) => Outcome::Win,
        (Play::Paper, Play::Rock) => Outcome::Loss,
        (Play::Scissors, Play::Rock) => Outcome::Win,
        (Play::Rock, Play::Scissors) => Outcome::Loss,
        (Play::Paper, Play::Scissors) => Outcome::Win,
        (Play::Scissors, Play::Paper) => Outcome::Loss,
        _ => {panic!("nope!");}
    };
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Play,Play)> {
    return input
        .lines()
        .map(|line| {

            let mut moves = line
                .split(' ');
            let opponent = parse_move(moves.next().unwrap());
            let me = parse_move(moves.next().unwrap());
            return (opponent, me);
        })
        .collect();
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<(Play, Play)>) -> i32 {
    let mut total_score = 0;
    for game in input{
        let outcome = outcome(game);
        total_score += get_score(outcome, game.1);
    }

    return total_score;
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<(Play, Play)>) -> i32 {
    let mut total_score = 0;
    for game in input{
        let required_outcome = play_to_outcome(&game.1);

        let required_move = required_move(game.0, required_outcome);
        total_score += get_score(required_outcome, required_move);
    }

    return total_score;
}

#[cfg(test)]
mod tests {
    use crate::day2::Play;
    use super::{part1,part2};

    // (()) and ()() both result in floor 0.
    #[test]
    fn sample1() {
        let mut vec: Vec<(Play, Play)> = Vec::new();
        vec.push((Play::Rock, Play::Scissors));
        assert_eq!(part1(&vec), 3);
    }

    #[test]
    fn sample2() {
        let mut vec: Vec<(Play, Play)> = Vec::new();
        vec.push((Play::Rock, Play::Scissors));
        // win and counter rock
        // 6 + paper = 8
        assert_eq!(part2(&vec), 8);
    }

    #[test]
    fn sample3() {
        let mut vec: Vec<(Play, Play)> = Vec::new();
        vec.push((Play::Scissors, Play::Scissors));
        // win and counter rock
        // 6 + paper = 8
        assert_eq!(part2(&vec), 7);
    }
}
