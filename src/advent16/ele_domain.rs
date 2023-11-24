use std::{
    collections::{BTreeSet, HashMap, HashSet},
    ops::Sub,
};

use itertools::Itertools;

use super::ele_algo::reduce_network;

type pressure = u32;
type weight = u32;

#[derive(Debug, PartialEq, Clone)]
pub struct Network<'a> {
    nodes: HashMap<&'a str, pressure>,
    edges: HashMap<&'a str, Vec<(&'a str, weight)>>,
}

impl<'a> Network<'a> {
    pub fn new(input: Vec<(&'a str, u32, Vec<&'a str>)>) -> Network {
        let mut network = Network {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        };
        for (name, flow, connections) in input.iter() {
            network.nodes.insert(name, *flow);
            network
                .edges
                .insert(name, connections.iter().map(|&c| (c, 1)).collect_vec());
        }
        let flow_nodes = network
            .nodes
            .iter()
            .filter(|n| *n.1 > 0 || n.0 == &"AA")
            .map(|node| *node.0)
            .collect_vec();
        let dist_matrix = reduce_network(&network, &flow_nodes);

        let mut new_nodes = HashMap::new();
        flow_nodes.iter().for_each(|f| {
            new_nodes.insert(*f, *network.nodes.get(f).unwrap());
        });

        let mut new_edges = HashMap::new();
        for i in 0..flow_nodes.len() {
            let node = flow_nodes[i];
            let mut edges = vec![];
            for j in 0..flow_nodes.len() {
                if i != j {
                    edges.push((flow_nodes[j], dist_matrix[i][j] as u32));
                }
            }
            new_edges.insert(node, edges);
        }

        Network {
            nodes: new_nodes,
            edges: new_edges,
        }
    }

    pub fn get_edges(&'a self, name: &'a str) -> &Vec<(&'a str, u32)> {
        let tmp = self.edges.get(name);
        if tmp.is_none() {
            panic!("fishy: {:?}, {}", self.edges, name);
        }
        tmp.unwrap()
    }

    pub fn get_pressure(&'a self, name: &'a str) -> &pressure {
        self.nodes.get(name).unwrap()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ERoute<'a> {
    pub current_valve: &'a str,
    pub total_pressure: u32,
    pub min_remaining: u32,
    pub route: BTreeSet<&'a str>,
    is_exhausted: bool,
}

impl<'a> ERoute<'a> {
    pub fn new(current_valve: &'a str, min_remaining: u32) -> ERoute<'a> {
        ERoute {
            current_valve,
            total_pressure: 0,
            min_remaining,
            route: BTreeSet::new(),
            is_exhausted: false,
        }
    }
    pub fn is_exhausted(&self) -> bool {
        self.is_exhausted
    }

    pub fn append_next<'b>(&'b mut self, next: &(&'a str, weight), pressure: pressure) {
        if self.min_remaining.checked_sub(next.1 + 1).is_some() && !self.route.contains(next.0) {
            self.current_valve = next.0;
            self.route.insert(next.0);
            self.min_remaining -= next.1 + 1;
            self.total_pressure += self.min_remaining as u32 * pressure;
        } else {
            self.is_exhausted = true;
        }
    }
}
