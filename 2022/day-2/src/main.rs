use std::fs;

enum Weapon {
    Scissors,
    Paper,
    Rock,
}

impl Weapon {
    fn get_score(self) -> i32 {
        match self {
            Weapon::Rock => 1,
            Weapon::Paper => 2,
            Weapon::Scissors => 3,
        }
    }

    fn from_elf(value: String) -> Option<Weapon> {
        match &*value {
            "A" => Some(Weapon::Rock),
            "B" => Some(Weapon::Paper),
            "C" => Some(Weapon::Scissors),
            _ => None,
        }
    }

    fn from_you(value: String) -> Option<Weapon> {
        match &*value {
            "X" => Some(Weapon::Rock),
            "Y" => Some(Weapon::Paper),
            "Z" => Some(Weapon::Scissors),
            _ => None,
        }
    }

    fn battle(&self, other: &Weapon) -> State {
        match self {
            Weapon::Rock => match other {
                Weapon::Rock => State::Draw,
                Weapon::Paper => State::Loss,
                Weapon::Scissors => State::Win,
            },
            Weapon::Paper => match other {
                Weapon::Rock => State::Win,
                Weapon::Paper => State::Draw,
                Weapon::Scissors => State::Loss,
            },
            Weapon::Scissors => match other {
                Weapon::Rock => State::Loss,
                Weapon::Paper => State::Win,
                Weapon::Scissors => State::Draw,
            },
        }
    }
}

enum State {
    Draw,
    Win,
    Loss,
}

impl State {
    fn get_score(self) -> i32 {
        match self {
            State::Draw => 3,
            State::Win => 6,
            State::Loss => 0,
        }
    }
}

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let games = contents.split("\n");

    let mut score: i32 = 0;
    for game in games {
        let splits: Vec<&str> = game.split(" ").collect();

        if splits.len() != 2 {
            continue;
        }

        let elf = Weapon::from_elf(splits[0].to_string()).unwrap();
        let you = Weapon::from_you(splits[1].to_string()).unwrap();

        let state = you.battle(&elf);

        score += you.get_score() + state.get_score();
    }

    println!("{score}");
}
