use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    multi::{many1, separated_list1},
    IResult,
};

use super::util;

#[derive(Debug)]
struct Grid {
    map: Vec<Vec<char>>,
    columns: Vec<u64>,
    rows: Vec<u64>,
}

fn grid(input: &str) -> IResult<&str, Grid> {
    let (rem, map) = separated_list1(tag("\n"), many1(one_of("#.")))(input)?;
    let rows = map
        .iter()
        .map(|v| {
            v.iter()
                .enumerate()
                .map(|(idx, &c)| if c == '.' { 0 } else { 1 << idx })
                .sum()
        })
        .collect();
    let columns = map[0]
        .iter()
        .enumerate()
        .map(|(vidx, _)| {
            map.iter()
                .enumerate()
                .map(|(idx, v)| if v[vidx] == '.' { 0 } else { 1 << idx })
                .sum()
        })
        .collect();
    Ok((rem, Grid { map, columns, rows }))
}

fn parse_maps(input: &str) -> IResult<&str, Vec<Grid>> {
    separated_list1(tag("\n\n"), grid)(input)
}

fn find_horizon(vec: &Vec<u64>) -> usize {
    let mut horizon = 0;
    let mut griter = vec.iter().enumerate().peekable();
    while let Some((_, &num)) = griter.next() {
        if let Some((nidx, &next)) = griter.peek() {
            if num != next {
                continue;
            }
            let rev = vec.iter().take(*nidx).rev();
            let zip = vec.iter().skip(*nidx).zip(rev);
            if zip.fold(true, |s, (e1, e2)| if s { e1 == e2 } else { false }) {
                horizon = *nidx;
                break;
            }
        }
    }
    horizon
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(&path)?.trim().parse()?;

    let (rem, grids) = parse_maps(&input).unwrap();
    if !rem.is_empty() {
        panic!("Bad input, remainder: {}", rem);
    }
    let mut sum = 0;
    for grid in grids {
        util::print_gridvec(&grid.map, 3, '.');
        println!("columns {:?}\nrows {:?}", grid.columns, grid.rows);
        let horizon = find_horizon(&grid.columns);
        println!("Verti-rizon: {}", horizon);
        if horizon != 0 {
            sum += horizon;
            continue;
        }
        let horizon = find_horizon(&grid.rows);
        println!("Hori-rizon: {}", horizon);
        if horizon != 0 {
            sum += 100 * horizon;
            continue;
        }
        panic!("No horizon!");
    }
    println!("Sum {}", sum);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let _input: String = std::fs::read_to_string(&path)?.trim().parse()?;
    Ok(())
}