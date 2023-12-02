use super::util;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;

const DATAFILE: &str = "./data/d02pt1.txt";

#[derive(Debug)]
struct Game {
    id: u32,
    samples: Vec<Sample>,
}

#[derive(Debug)]
struct Sample {
    red: u32,
    green: u32,
    blue: u32,
}

enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn parse_line(input: &str) -> Result<Game, Box<dyn std::error::Error + '_>> {
    let (remainder, game) = parse_game(input)?;
    if !remainder.is_empty() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Remainder remaining after parsing game: {}", remainder))))
    }
    Ok(game)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (remainder, (_space, id)) =
        separated_pair(tag("Game"), tag(" "), nom::character::complete::u32)(input)?;
    let (remainder, _) = tag(": ")(remainder)?;
    let (remainder, samples) = separated_list0(tag("; "), parse_sample)(remainder)?;
    Ok((remainder, Game {
        id,
        samples,
    }))
}

fn parse_sample(input: &str) -> IResult<&str, Sample> {
    let (remaining, v) = separated_list0(tag(", "), parse_color)(input)?;
    let mut sample = Sample {
        red: 0,
        green: 0,
        blue: 0,
    };
    v.iter().for_each(|c| match c {
        Color::Red(x) => {
            sample.red = *x;
        }
        Color::Green(x) => {
            sample.green = *x;
        }
        Color::Blue(x) => {
            sample.blue = *x;
        }
    });
    Ok((remaining, sample))
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (remaining, (x, color)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        alt((tag("red"), tag("green"), tag("blue"))),
    )(input)?;
    Ok((
        remaining,
        match color {
            "red" => Color::Red(x),
            "green" => Color::Green(x),
            "blue" => Color::Blue(x),
            _ => unreachable!(),
        },
    ))
}

pub fn pt1() -> Result<(), Box<dyn std::error::Error>> {
    let lines = util::parse_in_lines(DATAFILE)?;
    let bag = Sample {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut sum = 0;
    'outer: for line in lines {
        let game = parse_line(&line).unwrap();
        for sample in game.samples {
            if sample.red > bag.red || sample.green > bag.green || sample.blue > bag.blue {
                continue 'outer;
            }
        }
        sum += game.id;
    }
    println!("Sum of game IDs: {}", sum);
    Ok(())
}

pub fn pt2() -> Result<(), Box<dyn std::error::Error>> {
    let lines = util::parse_in_lines(DATAFILE)?;
    let mut sum = 0;
    for line in lines {
        let mut viable = Sample {
            red: 0,
            green: 0,
            blue: 0,
        };
        let game = parse_line(&line).unwrap();
        for sample in game.samples {
            if viable.red < sample.red {
                viable.red = sample.red;
            }
            if viable.green < sample.green {
                viable.green = sample.green;
            }
            if viable.blue < sample.blue {
                viable.blue = sample.blue;
            }
        }
        sum += viable.red * viable.green * viable.blue;
    }
    println!("Sum of game IDs: {}", sum);
    Ok(())
}
