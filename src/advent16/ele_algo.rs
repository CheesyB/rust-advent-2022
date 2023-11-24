use std::collections::{BTreeSet, HashMap, VecDeque};

use itertools::Itertools;

use super::ele_domain::*;

pub fn reduce_network<'a>(network: &Network, reduced_nodes: &Vec<&'a str>) -> Vec<Vec<usize>> {
    let mut dist_matrix = vec![vec![0; reduced_nodes.len()]; reduced_nodes.len()];
    for i in 0..reduced_nodes.len() {
        for j in 0..i {
            if i == j {
                dist_matrix[i][j] = 0;
            }
            let distance = network_bfs(network, reduced_nodes[i], reduced_nodes[j]).len() - 1; // dont count "AA"
            println!(
                "dist i:{} j:{}: {}",
                reduced_nodes[i], reduced_nodes[j], distance
            );
            dist_matrix[i][j] = distance;
            dist_matrix[j][i] = distance;
        }
    }
    dist_matrix
}

pub fn network_bfs<'a>(network: &'a Network, start: &'a str, end: &'a str) -> Vec<&'a str> {
    let mut queue = VecDeque::new();
    let start_node = vec![start];
    queue.push_back(start_node);
    while let Some(path) = queue.pop_back() {
        // move each edge
        let last_stop = path.last().unwrap();
        for next_stop in network
            .get_edges(&last_stop)
            .iter()
            .filter(|ns| ns.0 != *last_stop)
        {
            let mut new_path = path.clone();
            new_path.push(next_stop.0.clone());

            if next_stop.0 == end {
                return new_path;
            }
            queue.push_front(new_path);
        }
    }
    panic!("need to find a path");
}

pub fn agent_bfs<'a>(
    network: &'a Network,
    start: &'a str,
    total_min: u32,
) -> HashMap<BTreeSet<&'a str>, u32> {
    let mut path_to_pressure = HashMap::new();
    let mut queue = VecDeque::new();
    let start_route = ERoute::new(start, total_min);
    queue.push_back(start_route);
    while let Some(route) = queue.pop_back() {
        // move each edge
        for next_stop in network
            .get_edges(route.current_valve)
            .iter()
            .filter(|ns| ns.0 != route.current_valve && ns.0 != "AA")
        {
            let mut new_route = route.clone();
            new_route.append_next(next_stop, network.get_pressure(next_stop.0).clone());
            if !new_route.is_exhausted() {
                if path_to_pressure
                    .entry(new_route.route.clone())
                    .or_insert(new_route.total_pressure.clone())
                    < &mut new_route.total_pressure
                {
                    path_to_pressure
                        .insert(new_route.route.clone(), new_route.total_pressure.clone());
                }
                queue.push_front(new_route);
            }
        }
    }
    path_to_pressure
}

pub fn two_agent_bfs<'a>(network: &'a Network, start: &'a str, total_min: u32) -> u32 {
    let pressure_map = agent_bfs(network, start, total_min);
    let mut max_pressure = 0;

    for my_path in pressure_map.iter() {
        for ele_path in pressure_map.iter() {
            if my_path.0.iter().all(|valve| !ele_path.0.contains(valve)) {
                if max_pressure < my_path.1 + ele_path.1 {
                    max_pressure = my_path.1 + ele_path.1;
                }
            }
        }
    }
    max_pressure
}
