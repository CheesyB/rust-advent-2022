use std::collections::HashMap;

use super::domain::*;
use itertools::Itertools;

pub fn start_ele_explore(network: &Network) -> u32 {
    let junction = network.get_junction(&['A', 'A']);
    let start_point = Route::new(junction.name);
    ele_explore(network, start_point)
}

fn ele_explore(network: &Network, start: Route) -> u32 {
    let mut perm = HashMap::new();
    let flows = network.get_flow_junctions();
    let flow_len = flows.len();

    for i in 2..=flows.len() {
        let e_routes: Vec<ERoute> = flows
            .iter()
            .permutations(i)
            .map(|p| {
                ERoute::new(
                    p.iter().map(|&pp| pp.clone()).collect_vec(),
                    ele_bfs(network, &p),
                )
            })
            .collect();
        perm.insert(i, e_routes);
    }
    let solution = perm
        .iter()
        .flat_map(|(my_len, my_routes)| {
            my_routes.iter().flat_map(|my_route| {
                perm.get(&(flow_len - my_len.clone())).map(|ele_routes| {
                    println!("here");
                    ele_routes
                        .iter()
                        .filter(|ele_route| {
                            ele_route
                                .path
                                .iter()
                                .all(|ele_junk| !my_route.path.contains(ele_junk))
                        })
                        .map(|ele_route| my_route.score(network) + ele_route.score(network))
                        .collect::<Vec<_>>()
                })
            })
        })
        .flatten()
        .collect::<Vec<_>>();
    *solution.iter().max().unwrap()
}

pub fn ele_bfs(network: &Network, path: &Vec<&[char; 2]>) -> Vec<JName> {
    let mut final_route: Vec<JName> = vec![['A', 'A']];
    for destination in path {
        final_route = bfs_iteration(network, final_route, destination)
    }
    final_route
}

fn bfs_iteration(network: &Network, route: Vec<JName>, destination: &JName) -> Vec<JName> {
    let mut heuristic: Vec<Vec<JName>> = vec![route];
    loop {
        let mut new_heuristic = vec![];
        for route in heuristic.iter() {
            // get last junction of route
            let current_junction = route.last().unwrap().clone();
            // move each edge
            for next_stop in network.get_connection(&current_junction) {
                let mut new_route = route.clone();
                new_route.push(next_stop.clone());
                if next_stop == destination {
                    new_route.push(next_stop.clone());
                    return new_route;
                }
                new_heuristic.push(new_route)
            }
        }
        heuristic = new_heuristic;
    }
}

pub fn start_explore(network: &Network) -> Vec<Route> {
    let junction = network.get_junction(&['A', 'A']);
    let start_point = Route::new(junction.name);
    explore(network, start_point)
}

fn explore(network: &Network, start: Route) -> Vec<Route> {
    let mut heuristic: Vec<Route> = vec![];
    heuristic.push(start);
    for min in 1..=30 {
        let mut new_heuristic = vec![];
        for route in heuristic.iter() {
            // get last junction of route
            let current_junction = route.path.last().unwrap().clone();

            // move each edge
            for next_stop in network.get_connection(&current_junction) {
                let mut new_route = route.clone();
                new_route.path.push(next_stop.clone());
                new_heuristic.push(new_route)
            }
            //open valve
            let flow = network.get_junction(&current_junction).flow;
            if flow > 0 && route.is_still_closed(&current_junction) {
                let mut new_route = route.clone();
                new_route.open_valve(min, flow);
                new_heuristic.push(new_route);
            }
        }
        heuristic = new_heuristic;
        if min % 5 == 0 {
            heuristic = prune(heuristic, min as u32)
        }
    }
    heuristic
}

fn prune(heuristic: Vec<Route>, min_passed: u32) -> Vec<Route> {
    let max_score = heuristic
        .iter()
        .map(|route| route.score(min_passed))
        .max()
        .unwrap();
    let heuristic_len = heuristic.len();
    let pruned: Vec<_> = heuristic
        .into_iter()
        .filter(|route| route.score(min_passed) as f64 >= max_score as f64 * 0.95)
        .collect();
    println!(
        "max score {} at min {} reduction {:.2}% form {} to {}",
        max_score,
        min_passed,
        1.0 - pruned.len() as f64 / heuristic_len as f64,
        heuristic_len,
        pruned.len()
    );
    pruned
}
