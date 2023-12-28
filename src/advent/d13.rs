use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug)]
struct Grid {
    columns: Vec<u64>,
    rows: Vec<u64>,
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
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
    Ok((rem, Grid { columns, rows }))
}

fn parse_maps(input: &str) -> IResult<&str, Vec<Grid>> {
    separated_list1(tag("\n\n"), parse_grid)(input)
}

fn find_horizon(vec: &Vec<u64>, smudges: u32) -> Option<usize> {
    let mut horizon = None;
    let mut griter = vec.iter().enumerate().peekable();
    while let Some((_, &num)) = griter.next() {
        if let Some((nidx, &next)) = griter.peek() {
            if num != next && (num ^ next).count_ones() != 1 {
                continue;
            }
            let rev = vec.iter().take(*nidx).rev();
            let zip = vec.iter().skip(*nidx).zip(rev);
            let diff = zip.fold(0, |s, (e1, e2)| s + (e1 ^ *e2).count_ones());
            if diff == smudges {
                horizon = Some(*nidx);
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
        if let Some(horizon) = find_horizon(&grid.columns, 0) {
            sum += horizon;
        } else if let Some(horizon) = find_horizon(&grid.rows, 0) {
            sum += 100 * horizon;
        }
    }
    println!("Sum {}", sum);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(&path)?.trim().parse()?;

    let (rem, grids) = parse_maps(&input).unwrap();
    if !rem.is_empty() {
        panic!("Bad input, remainder: {}", rem);
    }
    let mut sum = 0;
    for grid in grids {
        if let Some(horizon) = find_horizon(&grid.columns, 1) {
            sum += horizon;
        } else if let Some(horizon) = find_horizon(&grid.rows, 1) {
            sum += 100 * horizon;
        }
    }
    println!("Sum {}", sum);
    Ok(())
}
