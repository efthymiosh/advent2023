use nom::bytes::complete::tag;
use nom::character::complete::none_of;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::IResult;

fn parse_input(input: &str) -> IResult<&str, Vec<String>> {
    let (rem, v) = separated_list1(tag(","), many1(none_of(",")))(input)?;
    Ok((rem, v.iter().map(|vc| vc.iter().collect()).collect()))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(path)?.trim().parse()?;

    let (_, sequences) = parse_input(&input).unwrap();

    let sum: u64 = sequences
        .iter()
        .map(|s| s.chars().fold(0, |s, c| ((s + c as u64) * 17) % 256))
        .sum();

    println!("Sum of hashes: {}", sum);
    Ok(())
}

struct Instr {
    label: String,
    op: char,
    target: u32,
}

fn parse_input_pt2(input: &str) -> IResult<&str, Vec<Instr>> {
    let (rem, v) =
        separated_list1(tag(","), pair(many1(none_of("=-")), many1(none_of(","))))(input)?;
    let v = v
        .iter()
        .map(|(l, o)| {
            let mut target = 0;
            if o[0] == '=' {
                target = o[1].to_digit(10).unwrap();
            }
            Instr {
                label: l.iter().collect::<String>(),
                op: o[0],
                target,
            }
        })
        .collect();
    Ok((rem, v))
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(path)?.trim().parse()?;

    let (_, sequences) = parse_input_pt2(&input).unwrap();

    let mut boxes: Vec<Vec<(String, u32)>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }
    sequences
        .iter()
        .for_each(|instr| {
            let pos = instr.label.chars().fold(0, |s, c| ((s + c as u64) * 17) % 256) as usize;
            let mut it = boxes[pos].iter().enumerate().skip_while(|(_, e)| e.0 != instr.label);
            if let Some((idx, _)) = it.next() {
                if instr.op == '-' {
                    boxes[pos].remove(idx);
                }
                if instr.op == '=' {
                    boxes[pos][idx] = (instr.label.clone(), instr.target);
                }
            } else if instr.op == '=' {
                boxes[pos].push((instr.label.clone(), instr.target));
            }
        });

    let sum: usize = boxes.iter().enumerate().map(|(i, v)| {
        v.iter().enumerate().map(|(vi, (_, d))| {
            (i + 1) * (vi + 1) * (*d as usize)
        }).sum::<usize>()
    }).sum();

    println!("Sum of hashes: {}", sum);
    Ok(())
}
