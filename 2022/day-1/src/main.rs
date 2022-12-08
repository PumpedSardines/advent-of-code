use std::fs;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let parts = contents.split("\n\n");

    let mut elves = vec![];

    for part in parts {
        let calories = part
            .split("\n")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .reduce(|t, v| t + v)
            .unwrap();

        elves.push(calories);
    }

    elves.sort();
    elves.reverse();

    println!("Biggest: {:?}", elves[0]);
    println!("Biggest three: {:?}", (0..3).fold(0, |t, v| t + elves[v]));
}
