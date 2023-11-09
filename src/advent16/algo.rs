use super::domain::*;
use itertools::Itertools;

pub fn start_ele_explore(network: &Network) -> Vec<ERoute> {
    let junction = network.get_junction(&['A', 'A']);
    let start_point = ERoute::new(junction.name);
    ele_explore(network, start_point)
}

fn ele_explore(network: &Network, start: ERoute) -> Vec<ERoute> {
    let mut perm = vec![];
    let flows = network.get_flow_junctions();

    for i in 1..flows.len() {
        perm.extend(flows.iter().combinations(i).collect_vec());
    }
}

pub fn ele_bfs(network: &Network, start: Route) -> Vec<Route> {
    vec![Route::new(['A', 'A'])]
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

fn prune_ele(heuristic: Vec<ERoute>, min_passed: u32) -> Vec<ERoute> {
    let max_score = heuristic
        .iter()
        .map(|route| return route.score(min_passed) as u32)
        .max()
        .unwrap();
    let max_pressure = heuristic
        .iter()
        .map(|route| {
            //route.print_paths();
            return route.pressure(min_passed);
        })
        .max()
        .unwrap();
    let heuristic_len = heuristic.len();

    let pruned: Vec<_> = heuristic
        .into_iter()
        .filter(|route| route.score(min_passed) as f64 >= max_score as f64 * 0.8)
        .collect();
    println!(
        "max score {}, max pressure {} at min {} reduction {:.2}% form {} to {}",
        max_score,
        max_pressure,
        min_passed,
        1.0 - pruned.len() as f64 / heuristic_len as f64,
        heuristic_len,
        pruned.len()
    );
    pruned
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
