// lib.rs - this file exists ONLY so the tests in tests/ can import the
// two algorithms. it's a mirror of the functions inside main.rs.
// the judge never sees this file; it runs main.rs standalone.

use std::collections::BinaryHeap;
// same priority pile as in main
use std::cmp::Reverse;
// flips it to smallest-first


// ---------- algorithm 1: union find (dsu) ----------

fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    // climb to x's group leader
    let mut root = x;
    while parent[root] != root {
        root = parent[root];
    }
    let mut cur = x;
    // path compression - flatten the path for next time
    while parent[cur] != root {
        let next = parent[cur];
        parent[cur] = root;
        cur = next;
    }
    root
}

pub fn solve_dsu(n: usize, edges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut parent: Vec<usize> = (0..=n).collect();
    // each city its own group at the start
    let mut size: Vec<usize> = vec![1; n + 1];
    // group sizes, for merging small into big

    for &(a, b) in edges {
        let ra = find(&mut parent, a);
        let rb = find(&mut parent, b);
        if ra != rb {
            if size[ra] < size[rb] {
                parent[ra] = rb;
                size[rb] += size[ra];
            } else {
                parent[rb] = ra;
                size[ra] += size[rb];
            }
        }
    }

    let mut reps: Vec<usize> = Vec::new();
    // grab one leader per island
    for city in 1..=n {
        if find(&mut parent, city) == city {
            reps.push(city);
        }
    }

    let mut roads: Vec<(usize, usize)> = Vec::new();
    // chain the leaders together
    for w in reps.windows(2) {
        roads.push((w[0], w[1]));
    }
    roads
}


// ---------- algorithm 2: heuristic search (greedy best-first) ----------

pub fn solve_heuristic(n: usize, edges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    // neighbour list
    for &(a, b) in edges {
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut visited = vec![false; n + 1];
    // flooded cities
    let mut reps: Vec<usize> = Vec::new();
    // one start city per island

    for start in 1..=n {
        if visited[start] {
            continue;
        }
        reps.push(start);
        // new island represented by start

        let mut frontier: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
        // greedy frontier - smallest city number first
        frontier.push(Reverse(start));
        visited[start] = true;

        while let Some(Reverse(city)) = frontier.pop() {
            // expand the smallest waiting city
            for &nb in &adj[city] {
                if !visited[nb] {
                    visited[nb] = true;
                    frontier.push(Reverse(nb));
                }
            }
        }
    }

    let mut roads: Vec<(usize, usize)> = Vec::new();
    for w in reps.windows(2) {
        roads.push((w[0], w[1]));
    }
    roads
}
