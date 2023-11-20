use std::collections::{HashMap, VecDeque};
use std::thread;

use super::domain::*;
use itertools::Itertools;

pub fn start_ele_explore(network: &Network) -> u32 {
    let junction = network.get_junction(&['A', 'A']);
    let start_point = Route::new(junction.name);
    ele_explore(network, start_point)
}

fn ele_explore(network: &Network, start: Route) -> u32 {
    let mut perm = HashMap::new();
    let mut tmp = HashMap::new();
    let flows = network.get_flow_junctions();
    let flow_len = flows.len();

    for i in 2..=flows.len() {
        let e_routes = flows.iter().permutations(i).collect_vec();
        tmp.insert(i, e_routes);
    }

    for (i, perm_len) in tmp {
        let mut handles = vec![];
        let thong: Vec<Vec<_>> = perm_len.chunks(5).map(|x| x.to_vec()).collect();
        for chunk in thong {
            let tmp_network = network.clone();
            let tmp_chunk = chunk.clone();
            handles.push(thread::spawn(move || {
                println!("Thread started");
                tmp_chunk
                    .iter()
                    .map(|p| p.into_iter().map(|&c| c.clone()).collect_vec())
                    .map(|p| {
                        ERoute::new(
                            p.iter().map(|&pp| pp.clone()).collect_vec(),
                            ele_bfs(&tmp_network, &p),
                        )
                    })
                    .collect_vec()
            }));
        }
        let result = handles
            .into_iter()
            .map(|th| th.join().unwrap())
            .flatten()
            .collect_vec();

        println!("iteration {}", i);
        perm.insert(i, result.clone());
    }
    let solution = perm
        .iter()
        .flat_map(|(my_len, my_routes)| {
            my_routes.iter().flat_map(|my_route| {
                perm.get(&(flow_len - my_len.clone())).map(|ele_routes| {
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

pub fn ele_bfs(network: &Network, path: &Vec<[char; 2]>) -> Vec<JName> {
    let mut final_route: Vec<JName> = vec![['A', 'A']];
    for destination in path {
        final_route.extend(bfs_iteration(
            network,
            final_route.last().unwrap().clone(),
            destination,
        ));
    }
    final_route.remove(0);
    final_route.push(final_route.last().unwrap().clone());
    final_route
}

fn bfs_iteration(network: &Network, start: JName, destination: &JName) -> Vec<JName> {
    let mut heuristic: VecDeque<Vec<JName>> = VecDeque::new();
    heuristic.push_back(vec![start]);
    while let Some(route) = heuristic.pop_front() {
        // get last junction of route
        let current_junction = route.last().unwrap();
        // move each edge
        let next_stops = network.get_connection(current_junction).clone();
        for next_stop in next_stops.iter() {
            let mut new_route = route.clone();
            if !new_route.contains(next_stop) {
                new_route.push(next_stop.clone());
                if next_stop == destination {
                    return new_route;
                }
                heuristic.push_front(new_route);
            }
        }
        heuristic.retain(|x| x.len() < 20)
    }
    panic!("somethings off here ");
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
