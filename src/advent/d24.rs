use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::multi::{many1, separated_list1};
use nom::sequence::{separated_pair, tuple};
use nom::IResult;

#[derive(Debug, Clone)]
struct Hailstone {
    p: (f64, f64, f64),
    v: (f64, f64, f64),
    a: f64,
    b: f64,
    c: f64,
}

fn parse_trip(input: &str) -> IResult<&str, (f64, f64, f64)> {
    let (rem, v) = separated_list1(tuple((tag(","), many1(tag(" ")))), i64)(input)?;
    if v.len() != 3 {
        return Err(nom::Err::Error(nom::error::ParseError::from_error_kind(
            input,
            nom::error::ErrorKind::SeparatedList,
        )));
    } else {
        Ok((rem, (v[0] as f64, v[1] as f64, v[2] as f64)))
    }
}

fn parse_hailstone(input: &str) -> IResult<&str, Hailstone> {
    let (rem, (p, v)) = separated_pair(
        parse_trip,
        tuple((many1(tag(" ")), tag("@"), many1(tag(" ")))),
        parse_trip,
    )(input)?;
    // p.0 = x + t * v.0
    // p.1 = y + t * v.1
    // => (p.0 - x) / v.0 = (p.1 - y) / v.1
    // => (p.0 - x) * v.1 = (p.1 - y) * v.0
    // => p.0 * v.1 - v.1 * x = p.1 * v.0 - v.0 * y
    // => v.1 * x - v.0 * y = p.0 * v.1 - p.1 * v.0
    // given ax + by = c
    // => a = v.1, b = -v.0, c = p.0 * v.1 - p.1 * v.0
    Ok((
        rem,
        Hailstone {
            p,
            v,
            a: v.1,
            b: -v.0,
            c: p.0 * v.1 - p.1 * v.0,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Hailstone>> {
    let (rem, v) = separated_list1(tag("\n"), parse_hailstone)(input)?;
    Ok((rem, v))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(&path)?.trim().parse()?;
    let (_, hailstones) = parse_input(&input).unwrap();
    let area_start: f64 = 200000000000000.0;
    let area_end: f64 = 400000000000000.0;

    let mut intersecting_points = 0;
    for (i, hs1) in hailstones.iter().enumerate() {
        for hs2 in hailstones[(i + 1)..].iter() {
            let div = hs1.a * hs2.b - hs1.b * hs2.a;
            // "parallel". This is not 100% correct as inaccuracies in float storage may have some values pass this test
            if div == 0.0 {
                continue;
            }
            let x = (hs1.c * hs2.b - hs2.c * hs1.b) / div;
            let y = (hs2.c * hs1.a - hs1.c * hs2.a) / div;
            // gather all checks
            let v = vec![
                // check if intersection in square
                area_start > x || x > area_end,
                area_start > y || y > area_end,
                // check if intersection not in past
                // if intersection in future then the sign of v.0 and v.1 must be equal to the sign of
                // the intersection at x and y equivalently
                (x - hs1.p.0) * hs1.v.0 < 0.0,
                (x - hs2.p.0) * hs2.v.0 < 0.0,
                (y - hs1.p.1) * hs1.v.1 < 0.0,
                (y - hs2.p.1) * hs2.v.1 < 0.0,
            ];
            if v.iter().any(|&e| e) {
                continue;
            }
            intersecting_points += 1;
        }
    }

    println!("Intersecting Points: {}", intersecting_points);

    Ok(())
}

pub fn pt2(_path: String) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
