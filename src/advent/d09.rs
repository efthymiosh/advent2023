use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::multi::separated_list1;
use nom::IResult;

fn parse_sequence(input: &str) -> IResult<&str, Vec<i64>> {
    let (rem, ret) = separated_list1(tag(" "), i64)(input)?;
    let (rem, _) = tag("\n")(rem)?;
    Ok((rem, ret.to_owned()))
}

trait YieldPrediction {
    fn yield_prediction(self: &mut Self);
    fn yield_inverse(self: &mut Self);
}

impl YieldPrediction for Vec<i64> {
    fn yield_prediction(self: &mut Vec<i64>) {
        println!("Calculating  {:?}", self);
        if self.iter().filter(|&x| *x != 0).collect::<Vec<&i64>>().len() == 0 {
            self.push(0);
            return;
        } else {
            let mut diffvec: Vec<i64> = self
                .iter()
                .skip(1)
                .scan(self[0], |s, e| {
                    let ret = Some(*e - *s);
                    *s = *e;
                    ret
                })
                .collect();
            diffvec.yield_prediction();
            self.push(self[self.len() - 1] + diffvec[diffvec.len() - 1]);
            println!("Yielded      {:?}", self);
        }
    }
    fn yield_inverse(self: &mut Vec<i64>) {
        println!("Calculating  {:?}", self);
        if self.iter().filter(|&x| *x != 0).collect::<Vec<&i64>>().len() == 0 {
            self.push(0);
            return;
        } else {
            let mut diffvec: Vec<i64> = self
                .iter()
                .skip(1)
                .scan(self[0], |s, e| {
                    let ret = Some(*s - *e);
                    *s = *e;
                    ret
                })
                .collect();
            diffvec.yield_inverse();
            self.push(self[self.len() - 1] - diffvec[diffvec.len() - 1]);
            println!("Yielded      {:?}", self);
        }
    }
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(&path)?.parse()?;

    let mut v: Vec<Vec<i64>> = Vec::new();

    let mut rem = input;
    loop {
        let (newrem, seq) = parse_sequence(&rem).unwrap();
        rem = newrem.to_owned();
        v.push(seq);
        if rem.is_empty() {
            break;
        }
    }

    for seq in &mut v {
        seq.yield_prediction();
    }

    let sum: i64 = v.iter().map(|ve| ve.last().unwrap()).sum();
    println!("Sum of last elements: {}", sum);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(&path)?.parse()?;

    let mut v: Vec<Vec<i64>> = Vec::new();
    let mut rem = input;
    while let Ok((newrem, seq)) = parse_sequence(&rem) {
        rem = newrem.to_owned();
        v.push(seq.into_iter().rev().collect());
    }
    for seq in &mut v {
        seq.yield_inverse();
    }

    let sum: i64 = v.iter().map(|ve| ve.last().unwrap()).sum();
    println!("Sum of last elements: {}", sum);
    Ok(())
}
