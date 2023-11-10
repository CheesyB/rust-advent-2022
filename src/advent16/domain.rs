use std::{cell::RefCell, collections::HashMap, vec};

use itertools::Itertools;
use nom::error_node_position;

pub type JName = [char; 2];

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

    pub fn get_flow_map(&self) -> HashMap<JName, u32> {
        let mut tmp = HashMap::new();
        let junk = self.get_flow_junctions();
        for j in junk.iter() {
            tmp.insert(j.clone(), self.get_junction(j).flow);
        }
        tmp
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
pub struct ERoute {
    pub path: Vec<JName>,
    pub route: Vec<JName>,
}

impl ERoute {
    pub fn new(path: Vec<JName>, route: Vec<JName>) -> ERoute {
        ERoute {
            path: path,
            route: route,
        }
    }
    pub fn score(&self, network: &Network) -> u32 {
        let mut pressure: [u32; 26] = [0; 26];
        let flow_map = network.get_flow_map();
        for (i, r) in self.route.iter().enumerate() {
            if i > 0 && r == self.route.get(i - 1).unwrap() {
                let additional_rpm = flow_map.get(r).unwrap();
                for j in (i)..26 {
                    pressure[j] += additional_rpm
                }
            }
        }
        pressure.iter().fold(0, |acc, val| acc + val)
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
