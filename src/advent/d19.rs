use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i64, one_of};
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Debug)]
enum Op {
    GT,
    LT,
}

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
enum Var {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Rule<'a> {
    condition: Option<(Var, Op, i64)>,
    target: &'a str,
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    let (rem, varc) = one_of("<>")(input)?;
    match varc {
        '>' => Ok((rem, Op::GT)),
        '<' => Ok((rem, Op::LT)),
        _ => panic!("Bad input"),
    }
}

fn parse_var(input: &str) -> IResult<&str, Var> {
    let (rem, varc) = one_of("xmas")(input)?;
    match varc {
        'x' => Ok((rem, Var::X)),
        'm' => Ok((rem, Var::M)),
        'a' => Ok((rem, Var::A)),
        's' => Ok((rem, Var::S)),
        _ => panic!("Bad input"),
    }
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let mut i = input;
    let mut condition = None;
    if let Ok((rem, (var, op, target, _))) = tuple((parse_var, parse_op, i64, tag(":")))(i) {
        i = rem;
        condition = Some((var, op, target));
    }
    let (rem, target) = alpha1(i)?;
    Ok((rem, Rule { condition, target }))
}

fn parse_workflow(input: &str) -> IResult<&str, (&str, Vec<Rule>)> {
    let (remainder, (name, v)) = tuple((
        alpha1,
        delimited(tag("{"), separated_list1(tag(","), parse_rule), tag("}")),
    ))(input)?;
    Ok((remainder, (name, v)))
}

fn parse_workflows(input: &str) -> IResult<&str, HashMap<&str, Vec<Rule>>> {
    let (rem, v) = separated_list1(tag("\n"), parse_workflow)(input)?;
    let (rem, _) = tag("\n\n")(rem)?;
    Ok((rem, v.into_iter().collect()))
}

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    let (rem, v) = separated_list1(
        tag("\n"),
        tuple((
            tag("{x="),
            i64,
            tag(",m="),
            i64,
            tag(",a="),
            i64,
            tag(",s="),
            i64,
            tag("}"),
        )),
    )(input)?;
    Ok((
        rem,
        v.into_iter()
            .map(|(_, x, _, m, _, a, _, s, _)| Part { x, m, a, s })
            .collect(),
    ))
}

fn apply_workflow(workflows: &HashMap<&str, Vec<Rule>>, part: &Part, wf: &[Rule<'_>]) -> i64 {
    for rule in wf {
        if let Some((var, op, check)) = &rule.condition {
            let val = match var {
                Var::X => part.x,
                Var::M => part.m,
                Var::A => part.a,
                Var::S => part.s,
            };
            let pass = match op {
                Op::GT => val > *check,
                Op::LT => val < *check,
            };
            if !pass {
                continue;
            }
        }
        return match rule.target {
            "A" => part.x + part.m + part.a + part.s,
            "R" => 0,
            x => apply_workflow(workflows, part, workflows.get(x).unwrap()),
        };
    }
    return 0;
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(path)?.trim().parse()?;
    let (rem, workflows) = parse_workflows(&input).unwrap();
    let (_rem, parts) = parse_parts(rem).unwrap();

    let mut sum = 0;
    for part in parts {
        let wf = workflows.get("in").unwrap();
        sum += apply_workflow(&workflows, &part, wf);
    }
    println!("Sum of xmas of accepted parts: {}", sum);
    Ok(())
}

#[derive(Debug)]
struct PartRange {
    hm: HashMap<Var, (i64, i64)>,
}

fn apply_workflow_ranges(workflows: &HashMap<&str, Vec<Rule>>, pr: PartRange, wf: &Vec<Rule>, wfstart: usize) -> i64 {
    for (idx, rule) in wf.iter().enumerate().skip(wfstart) {
        if let Some((var, op, check)) = &rule.condition {
            let r = pr.hm.get(var).unwrap();
            // [1, 2, 3, 4] - >3 -> [1, 2, 3] , [4]
            // [1, 2, 3, 4] - <3 -> [1, 2] , [3, 4]
            // [1, 2, 3, 4] - <1 -> offrange et vice versa
            // [1, 2, 3, 4] - >1 -> inrange et vice versa
            match op {
                Op::GT => {
                    if *check >= r.0 && *check < r.1 {
                        let mut hm_high = pr.hm.clone();
                        let mut hm_low = pr.hm.clone();
                        hm_low.insert(*var, (r.0, *check));
                        hm_high.insert(*var, (*check + 1, r.1));
                        return apply_workflow_ranges(workflows, PartRange{ hm: hm_high }, wf, idx)
                        + apply_workflow_ranges(workflows, PartRange{ hm: hm_low }, wf, idx);
                    } else if r.0 < *check {
                        continue;
                    }
                },
                Op::LT => {
                    if *check > r.0 && *check <= r.1 {
                        let mut hm_high = pr.hm.clone();
                        let mut hm_low = pr.hm.clone();
                        hm_low.insert(*var, (r.0, *check - 1));
                        hm_high.insert(*var, (*check, r.1));
                        return apply_workflow_ranges(workflows, PartRange{ hm: hm_high }, wf, idx)
                        + apply_workflow_ranges(workflows, PartRange{ hm: hm_low }, wf, idx);
                    } else if r.1 > *check {
                        continue;
                    }
                },
            };
        }
        return match rule.target {
            "A" => pr.hm.into_values().fold(1, |acu, (s,e)| (e - s + 1) * acu),
            "R" => 0,
            x => apply_workflow_ranges(workflows, pr, workflows.get(x).unwrap(), 0),
        };
    }
    return 0;
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(path)?.trim().parse()?;
    let (_rem, workflows) = parse_workflows(&input).unwrap();

    let pr = PartRange {
        hm: HashMap::from([
            (Var::X, (1, 4000)),
            (Var::M, (1, 4000)),
            (Var::A, (1, 4000)),
            (Var::S, (1, 4000)),
        ]),
    };
    let sum = apply_workflow_ranges(&workflows, pr, workflows.get("in").unwrap(), 0);
    println!("Accepted Combinations of parts: {}", sum);
    Ok(())
}
