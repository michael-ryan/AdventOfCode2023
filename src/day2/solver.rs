use crate::common;

#[derive(Clone, Copy, Debug)]
struct Handful {
    red: u32,
    green: u32,
    blue: u32,
}

impl Handful {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

pub fn solve() {
    println!("Day 2");
    let read_lines = common::read_file("src/day2/input.txt");
    println!("Part 1: {}", part1(&read_lines));
    println!("Part 2: {}", part2(&read_lines));
}

fn parse(lines: &Vec<String>) -> Vec<Vec<Handful>>{
    // actual filth

    let mut games: Vec<Vec<Handful>> = Vec::new();

    for line in lines {
        let mut handfuls: Vec<Handful> = Vec::new();
        for grab in line.split(':').collect::<Vec<&str>>()[1].split(';').map(|grab| grab.trim()) {
            let split_grab = grab.split(", ").collect::<Vec<&str>>();
            let mut new_handful = Handful{
                red: 0,
                green: 0,
                blue: 0,
            };
            for colour_quantity in split_grab {
                let split_colour_quantity = colour_quantity.split(' ').collect::<Vec<&str>>();
                let quantity = split_colour_quantity[0].parse::<u32>().unwrap();
                match split_colour_quantity[1] {
                    "red" => new_handful.red = quantity,
                    "green" => new_handful.green = quantity,
                    "blue" => new_handful.blue = quantity,
                    _ => panic!("unknown colour {}", split_colour_quantity[0]),
                }
            }
            handfuls.push(new_handful);
            dbg!(new_handful);
        }
        games.push(handfuls);
    }

    games
}

fn part1(read_lines: &Vec<String>) -> u32 {
    let games = parse(read_lines);

    let bag = Handful {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut possible_games: Vec<u32> = vec![];

    for (index, game) in games.iter().enumerate() {
        if is_game_possible(game, &bag) {
            possible_games.push(index as u32 + 1);
        }
    }

    possible_games.iter().sum()
}

fn is_game_possible(game: &Vec<Handful>, bag: &Handful) -> bool {
    for handful in game {
        if handful.red > bag.red || handful.green > bag.green || handful.blue > bag.blue {
            return false;
        }
    }

    true
}

fn part2(read_lines: &Vec<String>) -> u32 {
    let games = parse(read_lines);

    let mut bags: Vec<Handful> = vec![];

    for game in games {
        bags.push(compute_smallest_bag(&game));
    }

    bags.iter().map(|bag| bag.power()).sum()
}

fn compute_smallest_bag(game: &Vec<Handful>) -> Handful {
    let mut bag = Handful { red: 0, green: 0, blue: 0 };
    for handful in game {
        if handful.red > bag.red {
            bag.red = handful.red;
        }
        if handful.green > bag.green {
            bag.green = handful.green;
        }
        if handful.blue > bag.blue {
            bag.blue = handful.blue;
        }
    }

    bag
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_part1_correct() {
        let read_lines = common::read_file("src/day2/test.txt");

        assert_eq!(part1(&read_lines), 8);
    }

    #[test]
    fn is_part2_correct() {
        let read_lines = common::read_file("src/day2/test.txt");

        assert_eq!(part2(&read_lines), 2286);
    }
}