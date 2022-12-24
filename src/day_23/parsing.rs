use super::elves::{Elf, Elves};

impl From<&str> for Elves {
    fn from(input: &str) -> Self {
        let mut elves = Vec::<Elf>::new();

        for (y, line) in (0i64..).zip(input.lines().rev()) {
            for (x, character) in (0i64..).zip(line.chars()) {
                if character == '#' {
                    elves.push(Elf::new(x, y));
                }
            }
        }

        Elves(elves)
    }
}
