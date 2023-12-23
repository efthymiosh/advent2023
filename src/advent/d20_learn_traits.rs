use std::collections::HashMap;
use std::fmt::{Debug, Display};

use std::cell::RefCell;
use std::rc::Rc;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, none_of};
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;

type ModulePtr = Rc<RefCell<Box<dyn Module>>>;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
enum Signal {
    HIGH,
    LOW,
}

impl Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if *self == Signal::LOW {"low"} else {"high"})
    }
}

trait Module: Debug {
    fn process(&mut self, from: String, signal: Signal) -> (usize, usize);
    fn add_output(&mut self, output: ModulePtr);
    fn add_input(&mut self, input: ModulePtr);
    fn get_name(&self) -> &String;
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    on: bool,
    outputs: Vec<ModulePtr>,
}

impl Module for FlipFlop {
    fn process(&mut self, from: String, signal: Signal) -> (usize, usize) {
        println!("{} -{}> {}", from, signal, self.name);
        let (mut lowcount, mut highcount) = (0, 0);
        if signal == Signal::LOW {
            lowcount += 1;
            self.on = !self.on;
        } else {
            highcount += 1;
        }
        let send_signal = if self.on { Signal::HIGH } else { Signal::LOW };
        for m in &self.outputs {
            let (l, h) = m.borrow_mut().process(
                self.name.clone(),
                send_signal,
            );
            lowcount += l;
            highcount += h;
        }
        (lowcount, highcount)
    }

    fn add_output(&mut self, output: ModulePtr) {
        self.outputs.push(output);
    }

    fn add_input(&mut self, _input: ModulePtr) {
        return;
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug)]
struct Broadcaster {
    name: String,
    outputs: Vec<ModulePtr>,
}

impl Module for Broadcaster {
    fn process(&mut self, from: String, signal: Signal) -> (usize, usize) {
        println!("{} -{}> {}", from, signal, self.name);
        let (mut lowcount, mut highcount) = (0, 0);
        if signal == Signal::LOW {
            lowcount += 1;
        } else {
            highcount += 1;
        }
        for m in &self.outputs {
            let (l, h) = m.borrow_mut().process(self.name.clone(), signal);
            lowcount += l;
            highcount += h;
        }
        (lowcount, highcount)
    }

    fn add_output(&mut self, output: ModulePtr) {
        self.outputs.push(output);
    }

    fn add_input(&mut self, _input: ModulePtr) {
        return;
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    inputstate: HashMap<String, Signal>,
    outputs: Vec<ModulePtr>,
}

impl Module for Conjunction {
    fn process(&mut self, from: String, signal: Signal) -> (usize, usize) {
        println!("{} -{}> {}", from, signal, self.name);
        let (mut lowcount, mut highcount) = (0, 0);
        if signal == Signal::LOW {
            lowcount += 1;
        } else {
            highcount += 1;
        }
        self.inputstate.insert(from, signal);
        let send_signal = self.inputstate.values().fold(Signal::LOW, |s, sig| if *sig == Signal::LOW { Signal::HIGH } else {s});
        for m in &self.outputs {
            let (l, h) = m.borrow_mut().process(self.name.clone(), send_signal);
            lowcount += l;
            highcount += h;
        }
        (lowcount, highcount)
    }

    fn add_output(&mut self, output: ModulePtr) {
        self.outputs.push(output);
    }

    fn add_input(&mut self, input: ModulePtr) {
        self.inputstate
            .insert(input.borrow_mut().get_name().clone(), Signal::LOW);
        return;
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

fn parse_module<'a>(input: &'a str) -> IResult<&'a str, ((String, ModulePtr), Vec<String>)> {
    let (rem, (modchars, modoutputstr)) = separated_pair(
        many1(none_of(" ")),
        tag(" -> "),
        separated_list1(tag(", "), alpha1),
    )(input)?;
    let raw = modoutputstr.iter().map(|s| s.to_string()).collect();
    let (name, module): (String, ModulePtr) = match modchars[0] {
        '%' => (
            modchars[1..].iter().collect(),
            Rc::new(RefCell::new(Box::new(FlipFlop {
                name: modchars[1..].iter().collect(),
                on: false,
                outputs: Vec::new(),
            }))),
        ),
        '&' => (
            modchars[1..].iter().collect(),
            Rc::new(RefCell::new(Box::new(Conjunction {
                name: modchars[1..].iter().collect(),
                inputstate: HashMap::new(),
                outputs: Vec::new(),
            }))),
        ),
        _ => (
            modchars.iter().collect(),
            Rc::new(RefCell::new(Box::new(Broadcaster {
                name: modchars.iter().collect(),
                outputs: Vec::new(),
            }))),
        ),
    };
    Ok((rem, ((name, module), raw)))
}

fn parse_input<'a>(input: &'a str) -> IResult<&'a str, HashMap<String, ModulePtr>> {
    let (rem, modsvec) = separated_list1(tag("\n"), parse_module)(input)?;
    let ((keys, values), raw): ((Vec<String>, Vec<ModulePtr>), Vec<Vec<String>>) =
        modsvec.into_iter().unzip();
    let k = keys.clone();
    let mut mods: HashMap<&String, &ModulePtr> = keys.iter().zip(values.iter()).collect();
    for (key, outputs) in k.into_iter().zip(raw.into_iter()) {
        if let Some(module) = mods.get_mut(&key) {
            for outmodstr in outputs {
                let mut added = false;
                for outmodule in &values {
                    if outmodule.borrow_mut().get_name() != &outmodstr {
                        continue;
                    }
                    module.borrow_mut().add_output(outmodule.clone());
                    outmodule.borrow_mut().add_input(module.clone());
                    added = true;
                }
                // This is an output that is not referenced otherwise
                if !added {
                    module.borrow_mut().add_output(
                        Rc::new(RefCell::new(Box::new(Broadcaster {
                            name: outmodstr,
                            outputs: Vec::new(),
                        }))));
                }
            }
        }
    }
    let mods: HashMap<String, ModulePtr> = keys.into_iter().zip(values.into_iter()).collect();
    Ok((rem, mods))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(&path)?.trim().parse()?;
    let (rem, mut mods) = parse_input(&input).unwrap();
    if !rem.is_empty() {
        panic!("Remaining input");
    }
    let broadcaster = mods.get_mut("broadcaster").unwrap();
    let (mut lowcount, mut highcount) = (0, 0);
    for _ in 0..1000 {
        let (l, h) = broadcaster
            .borrow_mut()
            .process("button".to_owned(), Signal::LOW);
        lowcount += l;
        highcount += h;
    }
    println!("Lowcount {}\nHighcount {}\nTotal {}", lowcount, highcount, lowcount + highcount);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let _input: String = std::fs::read_to_string(&path)?.trim().parse()?;
    Ok(())
}
