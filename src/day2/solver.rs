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

    let mut quantity = 0;
    let mut done = false;
    let mut new_handful = Handful{
        red: 0,
        green: 0,
        blue: 0,
    };
    for line in lines {
        let mut handfuls: Vec<Handful> = Vec::new();
        for (index, item) in line.split_whitespace().collect::<Vec<&str>>()[2..].iter().enumerate() {
            if index % 2 == 0 {
                quantity = item.parse().unwrap();
            } else {
                let mut colour = *item;
                if colour.ends_with(';') {
                    done = true;
                }
                colour = colour.trim_end_matches(',').trim_end_matches(';');

                match colour {
                    "red" => new_handful.red = quantity,
                    "green" => new_handful.green = quantity,
                    "blue" => new_handful.blue = quantity,
                    _ => panic!("Found unknown colour: {}", colour),
                }
    
                if done {
                    done = false;
                    handfuls.push(new_handful);
                    new_handful = Handful{
                        red: 0,
                        green: 0,
                        blue: 0,
                    }
                }
            }
        }
        
        done = false;
        handfuls.push(new_handful);
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

    dbg!(games.len());

    let mut bags: Vec<Handful> = vec![];

    for game in games {
        bags.push(compute_smallest_bag(&game));
    }

    bags.iter().map(|bag| bag.power()).sum()
}

fn compute_smallest_bag(game: &Vec<Handful>) -> Handful {
    dbg!(game);
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
    dbg!(bag);
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