use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Display};

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, none_of};
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;

use super::util;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
enum Signal {
    HIGH,
    LOW,
}

impl Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if *self == Signal::LOW { "low" } else { "high" })
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]
struct Module {
    kind: &'static str,
    name: String,
    on: bool,
    outputs: Vec<String>,
    inputstate: HashMap<String, Signal>,
}

impl Module {
    fn process(&mut self, from: &String, signal: Signal) -> Vec<(&String, Signal)> {
        match self.kind {
            "flipflop" => {
                if signal == Signal::LOW {
                    self.on = !self.on;
                    let send_signal = if self.on { Signal::HIGH } else { Signal::LOW };
                    self.outputs.iter().map(|e| (e, send_signal)).collect()
                } else {
                    Vec::new()
                }
            }
            "broadcaster" => self.outputs.iter().map(|e| (e, signal)).collect(),
            "conjunction" => {
                self.inputstate.insert(from.to_string(), signal);
                let send_signal = self.inputstate.values().fold(Signal::LOW, |s, sig| {
                    if *sig == Signal::LOW {
                        Signal::HIGH
                    } else {
                        s
                    }
                });
                self.outputs.iter().map(|e| (e, send_signal)).collect()
            }
            _ => unreachable!(),
        }
    }
}

fn parse_module<'a>(input: &'a str) -> IResult<&'a str, Module> {
    let (rem, (modchars, modoutputstr)) = separated_pair(
        many1(none_of(" ")),
        tag(" -> "),
        separated_list1(tag(", "), alpha1),
    )(input)?;
    let outputs = modoutputstr.iter().map(|s| s.to_string()).collect();
    let module: Module = match modchars[0] {
        '%' => Module {
            kind: "flipflop",
            name: modchars[1..].iter().collect(),
            on: false,
            inputstate: HashMap::new(),
            outputs,
        },
        '&' => Module {
            kind: "conjunction",
            on: false,
            name: modchars[1..].iter().collect(),
            inputstate: HashMap::new(),
            outputs,
        },
        _ => Module {
            kind: "broadcaster",
            on: false,
            name: modchars.iter().collect(),
            inputstate: HashMap::new(),
            outputs,
        },
    };
    Ok((rem, module))
}

fn parse_input<'a>(input: &'a str) -> IResult<&'a str, HashMap<String, Module>> {
    let (rem, mut modules) = separated_list1(tag("\n"), parse_module)(input)?;
    let mut mods: HashMap<String, Module> = modules
        .iter()
        .map(|m| (m.name.clone(), m.clone()))
        .collect();
    let mut cjs: HashMap<String, &mut Module> = modules
        .iter_mut()
        .filter_map(|m| {
            if m.kind == "conjunction" {
                Some((m.name.clone(), m))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    // initialize conjunctions
    for m in mods.values_mut() {
        for out in &m.outputs {
            if let Some(cj) = cjs.get_mut(out) {
                cj.inputstate.insert(m.name.clone(), Signal::LOW);
            }
        }
    }
    for (k, v) in cjs.into_iter() {
        mods.insert(k, v.clone());
    }

    Ok((rem, mods))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(&path)?.trim().parse()?;
    let (rem, mut mods) = parse_input(&input).unwrap();
    if !rem.is_empty() {
        panic!("Remaining input");
    }
    let (mut lowcount, mut highcount) = (0, 0);
    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back((
            String::from("button"),
            String::from("broadcaster"),
            Signal::LOW,
        ));
        while let Some((from, to, signal)) = queue.pop_front() {
            println!("{} -{}> {}", from, signal, to);
            match signal {
                Signal::LOW => lowcount += 1,
                Signal::HIGH => highcount += 1,
            };
            if let Some(to_mod) = mods.get_mut(&to) {
                for (next_to, next_signal) in to_mod.process(&from, signal) {
                    queue.push_back((to.clone(), next_to.clone(), next_signal));
                }
            }
        }
    }
    println!(
        "Lowcount {}\nHighcount {}\nResult {}",
        lowcount,
        highcount,
        lowcount * highcount
    );
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(&path)?.trim().parse()?;
    let (rem, mut mods) = parse_input(&input).unwrap();
    if !rem.is_empty() {
        panic!("Remaining input");
    }
    //based on input 4 independent pules arrive on vr, which is the input of rx
    let vr = mods.get("vr").unwrap();
    let mut cycle_map: HashMap<String, u64> = vr.inputstate.iter().map(|(e, _)| (e.clone(), 0)).collect();
    'outer: for i in 1.. {
        let mut queue = VecDeque::new();
        queue.push_back((
            String::from("button"),
            String::from("broadcaster"),
            Signal::LOW,
        ));
        while let Some((from, to, signal)) = queue.pop_front() {
            if let Some(mo) = cycle_map.get_mut(&from) {
                if signal == Signal::HIGH {
                    *mo = i;
                }
                if cycle_map.values().fold(true, |s, e| if s { *e != 0 } else {s}) {
                    break 'outer;
                }
            }
            if let Some(to_mod) = mods.get_mut(&to) {
                for (next_to, next_signal) in to_mod.process(&from, signal) {
                    queue.push_back((to.clone(), next_to.clone(), next_signal));
                }
            }
        }
    }
    println!("{:?}", cycle_map);
    let lcm = cycle_map.values().fold(
        *(cycle_map.values().next().unwrap()) as i64, |s, &x| util::math::lcm(s, x as i64));
    println!("Iterations to rx module: {:?}", lcm);
    Ok(())
}
