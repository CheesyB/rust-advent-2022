use std::{cell::RefCell, collections::HashMap, vec};

use nom::error_node_position;

type JName = [char; 2];

#[derive(Debug, PartialEq, Clone)]
pub struct Junction {
    pub name: JName,
    pub flow: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Network {
    nodes: HashMap<JName, Junction>,
    connection: HashMap<JName, Vec<JName>>,
}

fn str2array(input: &str) -> [char; 2] {
    let mut tmp = input.chars();
    [tmp.next().unwrap(), tmp.next().unwrap()]
}
impl<'a> Network {
    pub fn new(input: Vec<(&'a str, u32, Vec<&'a str>)>) -> Network {
        let mut network = Network {
            nodes: HashMap::new(),
            connection: HashMap::new(),
        };
        for (name, flow, connections) in input.iter() {
            network.nodes.insert(
                str2array(name),
                Junction {
                    name: str2array(name),
                    flow: *flow,
                },
            );
            network.connection.insert(
                str2array(name),
                connections.iter().map(|s| str2array(s)).collect(),
            );
        }
        network
    }

    pub fn get_junction(&self, name: &JName) -> &Junction {
        self.nodes.get(name).unwrap()
    }
    pub fn get_connection(&self, name: &JName) -> &Vec<JName> {
        self.connection.get(name).unwrap()
    }
    pub fn get_flow_junctions(&self) -> Vec<JName> {
        self.nodes
            .iter()
            .filter(|n| self.get_junction(n.0).flow > 0)
            .map(|n| n.1.name)
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Route {
    pub path: Vec<JName>,
    pub pressure: [u32; 30],
    pub opened_valves: Vec<JName>,
}

impl Route {
    pub fn new(name: JName) -> Route {
        Route {
            path: vec![name],
            pressure: [0; 30],
            opened_valves: vec![],
        }
    }
    pub fn is_still_closed(&self, name: &JName) -> bool {
        self.opened_valves.iter().find(|n| *n == name).is_none()
    }
    pub fn open_valve(&mut self, min_passed: usize, additional_rpm: u32) {
        for i in min_passed..30 {
            self.pressure[i] += additional_rpm
        }
        self.opened_valves.push(self.path.last().unwrap().clone());
    }

    pub fn score(&self, min_passed: u32) -> u32 {
        self.pressure
            .iter()
            .take(min_passed as usize)
            .fold(0, |acc, val| acc + val)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ERoute {
    pub my_path: Vec<JName>,
    pub ele_path: Vec<JName>,
    pub pressure: [u32; 26],
    pub debug: Vec<(usize, u32, JName)>,
    pub opened_valves: Vec<JName>,
}

impl ERoute {
    pub fn new(name: JName) -> ERoute {
        ERoute {
            my_path: vec![name],
            ele_path: vec![name],
            pressure: [0; 26],
            debug: vec![],
            opened_valves: vec![],
        }
    }
    pub fn is_still_closed(&self, name: &JName) -> bool {
        self.opened_valves.iter().find(|n| *n == name).is_none()
    }
    pub fn open_valve(&mut self, min_passed: usize, additional_ppm: u32, jname: &JName) {
        for i in min_passed..26 {
            self.pressure[i] += additional_ppm
        }
        self.opened_valves.push(jname.clone());
        self.debug.push((min_passed, additional_ppm, jname.clone()));
    }
    pub fn pressure(&self, min_passed: u32) -> u32 {
        self.pressure
            .iter()
            .take(min_passed as usize)
            .fold(0, |acc, val| acc + val)
    }

    pub fn score(&self, min_passed: u32) -> f32 {
        self.pressure
            .iter()
            .take(min_passed as usize)
            .enumerate()
            .fold(0.0, |acc, val| {
                acc as f32 + (*val.1 as f32 * (min_passed as f32 - val.0 as f32 * 0.2))
            })
    }
    pub fn print_paths(&self) {
        println!("my: {:?}", self.my_path);
        println!("ele: {:?}", self.ele_path);
        println!();
    }

    pub fn are_all_valves_open(&self) -> bool {
        self.opened_valves.len() == 6
    }
}
