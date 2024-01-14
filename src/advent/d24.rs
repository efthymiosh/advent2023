use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::multi::{many1, separated_list1};
use nom::sequence::{separated_pair, tuple};
use nom::IResult;

use nalgebra::{matrix, vector};

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
        Err(nom::Err::Error(nom::error::ParseError::from_error_kind(
            input,
            nom::error::ErrorKind::SeparatedList,
        )))
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
    let input: String = std::fs::read_to_string(path)?.trim().parse()?;
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
            let v = [
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

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    // Solution heavily based on https://gitlab.com/silmeth/advent-of-code-2023/-/blob/main/day-24/src/lib.rs
    // Let the rock stone be defined as pos + velocity: Rp and Rv
    // Since the rock crosses all hailstones then for any hailstone hs
    // and a set time t:
    // Rp_x + t * Rv_x = hsp_x + t * hsv_x
    // => t * (Rv_x - hsv_x) = hsp_x - Rp_x
    // => t = (Rp_x - hsp_x) / (hsv_x - Rv_x)
    // equally for Y and Z
    // => t = (Rp_y - hsp_y) / (hsv_y - Rv_y)
    // => t = (Rp_z - hsp_z) / (hsv_z - Rv_z)
    // Therefore, by eliminating t:
    // => (Rp_x - hsp_x) / (hsv_x - Rv_x) = (Rp_y - hsp_y) / (hsv_y - Rv_y)
    // => (Rp_x - hsp_x) * (hsv_y - Rv_y) = (Rp_y - hsp_y) * (hsv_x - Rv_x)
    // Rp_x * hsv_y - Rp_x * Rv_y - hsp_x * hsv_y + hsp_x * Rv_y =
    //      Rp_y * hsv_x - Rp_y * Rv_x - hsp_y * hsv_x + hsp_y * Rv_x
    // => Rp_y * Rv_x - Rp_x * Rv_y =
    //    - Rp_x * hsv_y + hsp_x * hsv_y - hsp_x * Rv_y + Rp_y * hsv_x - hsp_y * hsv_x + hsp_y * Rv_x
    // So for any two hailstones hs1 and hs2:
    // Rp_y * Rv_x - Rp_x * Rv_y =
    //    - Rp_x * hs1v_y + hs1p_x * hs1v_y - hs1p_x * Rv_y + Rp_y * hs1v_x - hs1p_y * hs1v_x + hs1p_y * Rv_x
    // Rp_y * Rv_x - Rp_x * Rv_y =
    //    - Rp_x * hs2v_y + hs2p_x * hs2v_y - hs2p_x * Rv_y + Rp_y * hs2v_x - hs2p_y * hs2v_x + hs2p_y * Rv_x
    // =>
    //    - Rp_x * hs1v_y + hs1p_x * hs1v_y - hs1p_x * Rv_y + Rp_y * hs1v_x - hs1p_y * hs1v_x + hs1p_y * Rv_x
    //    =
    //    - Rp_x * hs2v_y + hs2p_x * hs2v_y - hs2p_x * Rv_y + Rp_y * hs2v_x - hs2p_y * hs2v_x + hs2p_y * Rv_x
    // =>
    //   Rp_x * (hs2v_y - hs1v_y)
    // + Rp_y * (hs1v_x - hs2v_x)
    // + Rv_x * (hs1p_y - hs2p_y)
    // + Rv_y * (hs2p_x - hs1p_x)
    // =
    //   hs2p_x * hs2v_y
    // - hs1p_x * hs1v_y
    // + hs1p_y * hs1v_x
    // - hs2p_y * hs2v_x
    //
    // Where the above is a system with 4 unknowns. We can solve this using linear algebra with
    // Gaussian Elimination
    let input: String = std::fs::read_to_string(path)?.trim().parse()?;
    let (_, hs) = parse_input(&input).unwrap();

    // generate a matrix for a system with 4 equations to solve for the 4 unknowns
    // This requires 5 hailstones

    // coefficients from lines L130 to L133
    let mut lhs = matrix![
        hs[1].v.1 - hs[0].v.1,      hs[0].v.0 - hs[1].v.0,      hs[0].p.1 - hs[1].p.1,      hs[1].p.0 - hs[0].p.0;
        hs[2].v.1 - hs[1].v.1,      hs[1].v.0 - hs[2].v.0,      hs[1].p.1 - hs[2].p.1,      hs[2].p.0 - hs[1].p.0;
        hs[3].v.1 - hs[2].v.1,      hs[2].v.0 - hs[3].v.0,      hs[2].p.1 - hs[3].p.1,      hs[3].p.0 - hs[2].p.0;
        hs[4].v.1 - hs[3].v.1,      hs[3].v.0 - hs[4].v.0,      hs[3].p.1 - hs[4].p.1,      hs[4].p.0 - hs[3].p.0;
    ];

    // right-hand-side L135 to L138
    let rhs = vector![
        hs[1].p.0 * hs[1].v.1 - hs[0].p.0 * hs[0].v.1 + hs[0].p.1 * hs[0].v.0 - hs[1].p.1 * hs[1].v.0,
        hs[2].p.0 * hs[2].v.1 - hs[1].p.0 * hs[1].v.1 + hs[1].p.1 * hs[1].v.0 - hs[2].p.1 * hs[2].v.0,
        hs[3].p.0 * hs[3].v.1 - hs[2].p.0 * hs[2].v.1 + hs[2].p.1 * hs[2].v.0 - hs[3].p.1 * hs[3].v.0,
        hs[4].p.0 * hs[4].v.1 - hs[3].p.0 * hs[3].v.1 + hs[3].p.1 * hs[3].v.0 - hs[4].p.1 * hs[4].v.0,
    ];

    lhs.try_inverse_mut();

    // Solving these equations gives us (Rp_x, Rp_y, Rv_x, Rv_y):
    let res = lhs * rhs;
    let rp_x = res[(0,0)];
    let rp_y = res[(1,0)];
    let rv_x = res[(2,0)];

    // What’s left is calculating Rp_z. Let’s just compute t_0, t_1, and then use:
    //   Rp_z + Rv_z · t_n = z_n + v_zn · t_n.

    let t0 = (rp_x - hs[0].p.0) / (hs[0].v.0 - rv_x);
    let t1 = (rp_x - hs[1].p.0) / (hs[1].v.0 - rv_x);

    let mut z_eq_lh = matrix![1., t0; 1., t1];
    z_eq_lh.try_inverse_mut();
    let z_eq_rh = vector![
        hs[0].p.2 + hs[0].v.2 * t0,
        hs[1].p.2 + hs[1].v.2 * t1,
    ];

    let rv_z = (z_eq_lh * z_eq_rh)[(0,0)];

    println!("Sum of starting coordinates: {}", (rp_x + rp_y + rv_z) as u64);
    Ok(())
}
