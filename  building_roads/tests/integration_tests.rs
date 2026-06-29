// integration tests - run with `cargo test`
// we pull both algorithms out of the lib and check two things:
//   1. they build the right NUMBER of roads (islands - 1)
//   2. after building those roads, every city can reach every other

use building_roads::{solve_dsu, solve_heuristic};

// tiny dsu just for the tests - lets us count islands and check connectivity
// without trusting the code we're testing
fn root(parent: &mut Vec<usize>, mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]]; // squash a little while climbing
        x = parent[x];
    }
    x
}

fn unite(parent: &mut Vec<usize>, a: usize, b: usize) {
    let ra = root(parent, a);
    let rb = root(parent, b);
    if ra != rb {
        parent[ra] = rb;
    }
}

// how many separate islands does this graph have?
fn count_components(n: usize, edges: &[(usize, usize)]) -> usize {
    let mut parent: Vec<usize> = (0..=n).collect();
    for &(a, b) in edges {
        unite(&mut parent, a, b);
    }
    let mut seen = std::collections::HashSet::new();
    for c in 1..=n {
        let r = root(&mut parent, c);
        seen.insert(r);
    }
    seen.len()
}

// is the graph one single island after we add the new roads?
fn fully_connected(n: usize, edges: &[(usize, usize)], extra: &[(usize, usize)]) -> bool {
    let mut all = edges.to_vec();
    all.extend_from_slice(extra);
    count_components(n, &all) == 1 || n == 0
}

#[test]
fn example_from_statement() {
    // 4 cities, roads 1-2 and 3-4 -> two islands -> need exactly 1 road
    let n = 4;
    let edges = vec![(1, 2), (3, 4)];

    let r1 = solve_dsu(n, &edges);
    let r2 = solve_heuristic(n, &edges);

    assert_eq!(r1.len(), 1);
    assert_eq!(r2.len(), 1);
    assert!(fully_connected(n, &edges, &r1));
    assert!(fully_connected(n, &edges, &r2));
}

#[test]
fn already_connected_needs_nothing() {
    // a line 1-2-3 is already one island -> 0 new roads
    let n = 3;
    let edges = vec![(1, 2), (2, 3)];

    assert_eq!(solve_dsu(n, &edges).len(), 0);
    assert_eq!(solve_heuristic(n, &edges).len(), 0);
}

#[test]
fn all_isolated() {
    // 5 lonely cities, no roads -> 5 islands -> 4 new roads
    let n = 5;
    let edges: Vec<(usize, usize)> = vec![];

    let r1 = solve_dsu(n, &edges);
    let r2 = solve_heuristic(n, &edges);

    assert_eq!(r1.len(), 4);
    assert_eq!(r2.len(), 4);
    assert!(fully_connected(n, &edges, &r1));
    assert!(fully_connected(n, &edges, &r2));
}

#[test]
fn single_city() {
    // one city is already "all connected" -> 0 roads, no crash
    let n = 1;
    let edges: Vec<(usize, usize)> = vec![];

    assert_eq!(solve_dsu(n, &edges).len(), 0);
    assert_eq!(solve_heuristic(n, &edges).len(), 0);
}

#[test]
fn both_agree_on_count() {
    // a messier graph - both algorithms must need the same number of roads
    let n = 10;
    let edges = vec![(1, 2), (2, 3), (4, 5), (6, 7), (7, 8), (8, 6)];
    // islands: {1,2,3} {4,5} {6,7,8} {9} {10} = 5 islands -> 4 roads

    let r1 = solve_dsu(n, &edges);
    let r2 = solve_heuristic(n, &edges);

    let expected = count_components(n, &edges) - 1;
    assert_eq!(r1.len(), expected);
    assert_eq!(r2.len(), expected);
    assert_eq!(r1.len(), r2.len());
    assert!(fully_connected(n, &edges, &r1));
    assert!(fully_connected(n, &edges, &r2));
}

#[test]
fn bigger_deterministic_graph() {
    // build 200 chains of 50 cities -> 200 islands -> 199 roads
    let n = 10_000;
    let mut edges = Vec::new();
    let mut c = 1;
    while c + 49 <= n {
        for k in 0..49 {
            edges.push((c + k, c + k + 1));
        }
        c += 50;
    }

    let expected = count_components(n, &edges) - 1; // 199
    let r1 = solve_dsu(n, &edges);
    let r2 = solve_heuristic(n, &edges);

    assert_eq!(r1.len(), expected);
    assert_eq!(r2.len(), expected);
    assert!(fully_connected(n, &edges, &r1));
    assert!(fully_connected(n, &edges, &r2));
}
