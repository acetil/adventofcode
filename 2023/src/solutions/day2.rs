type Turn = (u32, u32, u32);
type Game = (u32, Vec<Turn>);

static NUM_RED: u32 = 12;
static NUM_BLUE: u32 = 14;
static NUM_GREEN: u32 = 13;

pub fn parse_turn (turn: &str) -> Turn {
    let mut num_red = 0u32;
    let mut num_blue = 0u32;
    let mut num_green = 0u32;

    for mut i in turn.split(",") {
        i = i.trim();

        let mut s = i.split(" ");
        let num: u32 = s.next().unwrap().parse().unwrap();
        let colour = s.next().unwrap();

        if colour == "red" {
            num_red = num;
        } else if colour == "blue" {
            num_blue = num;
        } else {
            num_green = num;
        }
    }

    (num_red, num_blue, num_green)
}

pub fn parse_game (line: &str) -> Game {
    let mut split = line.split(":");
    let id: u32 = split.next()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse()
        .unwrap();

    return (id, split.next().unwrap().split(";").map(parse_turn).collect())
}

pub fn check_game_p1 (game: &Game) -> bool {
    for &(r, b, g) in &game.1 {
        if r > NUM_RED || b > NUM_BLUE || g > NUM_GREEN {
            return false;
        }
    }

    return true;
}

pub fn calc_power (game: &Game) -> u32 {
    let num_red = game.1.iter().map(|i| i.0).max().unwrap();
    let num_blue = game.1.iter().map(|i| i.1).max().unwrap();
    let num_green = game.1.iter().map(|i| i.2).max().unwrap();

    return num_red * num_blue * num_green;
}

pub fn solve (input: &str) {
    let games = input.lines()
        .map(parse_game)
        .collect::<Vec<Game>>();

    let sum: u32 = games.iter()
        .filter(|g| check_game_p1(*g))
        .map(|g| g.0)
        .sum();
    println!("Part 1: {sum}");

    let power: u32 = games.iter().map(calc_power).sum();
    println!("Part 2: {power}");
}