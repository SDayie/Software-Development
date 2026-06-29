use std::collections::BinaryHeap;
// binaryheap - a priority pile. always hands you the "best" item first
use std::cmp::Reverse;
// reverse - flips a binaryheap from biggest-first into smallest-first

// ----------------------------------------------------------------------
// the problem: cities + roads. some cities can't reach each other.
// answer = number of separate "islands" of cities minus 1.
// connect one city from each island in a chain -> everything reachable.
// both algos below just find the islands (connected components) differently.
// ----------------------------------------------------------------------


// ======================================================================
// ALGORITHM 1 - union find (dsu)
// idea: glue cities into groups. each group keeps one "leader".
// count the leaders -> that's how many islands we have.
// ======================================================================

fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    // find - tells you the leader of x's group
    let mut root = x;
    // start at x and climb upward
    while parent[root] != root {
        // a city that points at itself is a leader. keep going till we hit one
        root = parent[root];
    }
    let mut cur = x;
    // path compression - second walk to point everyone straight at the leader
    while parent[cur] != root {
        // so next time the climb is instant
        let next = parent[cur];
        // remember where we were going
        parent[cur] = root;
        // re-point this city directly at the boss
        cur = next;
        // step to the one we remembered
    }
    root
    // hand back the leader
}

fn solve_dsu(n: usize, edges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut parent: Vec<usize> = (0..=n).collect();
    // parent[i] = i  -> every city starts as its own little group of one
    let mut size: Vec<usize> = vec![1; n + 1];
    // size[i] = how many cities sit in i's group. used to merge small into big

    for &(a, b) in edges {
        // walk every existing road
        let ra = find(&mut parent, a);
        // leader of a's group
        let rb = find(&mut parent, b);
        // leader of b's group
        if ra != rb {
            // already same group? do nothing. different? glue them
            if size[ra] < size[rb] {
                // hang the smaller group under the bigger one (keeps trees short)
                parent[ra] = rb;
                size[rb] += size[ra];
            } else {
                parent[rb] = ra;
                size[ra] += size[rb];
            }
        }
    }

    let mut reps: Vec<usize> = Vec::new();
    // reps - one city per island (we just grab the leaders)
    for city in 1..=n {
        // check every city
        if find(&mut parent, city) == city {
            // a city that is its own leader = the head of an island
            reps.push(city);
        }
    }

    let mut roads: Vec<(usize, usize)> = Vec::new();
    // the new roads we will build
    for w in reps.windows(2) {
        // windows(2) - slide over reps two at a time: (r0,r1),(r1,r2)...
        roads.push((w[0], w[1]));
        // chain the islands together one link at a time
    }
    roads
    // k = roads.len() = (islands - 1). exactly the minimum
}


// ======================================================================
// ALGORITHM 2 - heuristic search (greedy best-first flood fill)
// idea: spread out from a city to flood its whole island.
// the "heuristic": from everything we can reach next, always step into
// the smallest-numbered city first (a greedy best-first choice).
// different machinery than dsu -> a priority frontier instead of leaders.
// ======================================================================

fn solve_heuristic(n: usize, edges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    // adj - neighbour list. adj[c] = all cities that share a road with c
    for &(a, b) in edges {
        // roads go both ways
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut visited = vec![false; n + 1];
    // visited - cities we've already flooded
    let mut reps: Vec<usize> = Vec::new();
    // reps - one starting city per island

    for start in 1..=n {
        // try every city as a possible new island start
        if visited[start] {
            // seen it already? it's part of an island we did. skip
            continue;
        }
        reps.push(start);
        // brand new island -> this city represents it

        let mut frontier: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
        // frontier - the "to explore" pile, ordered smallest city first
        frontier.push(Reverse(start));
        // seed it with our starting city
        visited[start] = true;
        // mark seeded so we never re-add it

        while let Some(Reverse(city)) = frontier.pop() {
            // greedy step: pull out the smallest-numbered waiting city
            for &nb in &adj[city] {
                // look at its neighbours
                if !visited[nb] {
                    // new neighbour? it's in this same island
                    visited[nb] = true;
                    // claim it
                    frontier.push(Reverse(nb));
                    // and queue it to spread further
                }
            }
        }
        // frontier empty -> this whole island is flooded
    }

    let mut roads: Vec<(usize, usize)> = Vec::new();
    // same chaining as algo 1
    for w in reps.windows(2) {
        roads.push((w[0], w[1]));
    }
    roads
}


// ======================================================================
// fn main - the part the judge actually runs
// ======================================================================

fn main() {
    // bench mode first, before touching stdin, so the judge path stays clean
    if std::env::var("RUN_BENCHMARK").is_ok() || std::env::args().any(|a| a == "--bench") {
        // only fires when YOU ask (env var or --bench flag). never on the judge
        run_benchmark();
        return;
    }

    use std::io::{self, Read, Write};
    // io - reading stdin / writing stdout
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    // slurp the whole input in one go (fast)
    let mut it = input.split_whitespace();
    // chop it into separate numbers by spaces / newlines

    let n: usize = it.next().unwrap().parse().unwrap();
    // n - number of cities
    let m: usize = it.next().unwrap().parse().unwrap();
    // m - number of existing roads

    let mut edges: Vec<(usize, usize)> = Vec::with_capacity(m);
    // store the roads as pairs
    for _ in 0..m {
        // read m pairs
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        edges.push((a, b));
    }

    let roads = solve_dsu(n, &edges);
    // use algo 1 for the real answer (deterministic + fast). algo 2 is for the bench

    let mut out = String::new();
    // build the entire answer as one string, then print once -> way faster
    out.push_str(&roads.len().to_string());
    // first line = k, the number of new roads
    out.push('\n');
    for (a, b) in &roads {
        // then one line per new road
        out.push_str(&a.to_string());
        out.push(' ');
        out.push_str(&b.to_string());
        out.push('\n');
    }
    io::stdout().write_all(out.as_bytes()).unwrap();
    // only the clean answer hits stdout. nothing else
}


// ======================================================================
// benchmark - gated, prints to stderr, builds its own test graph
// (stderr so it can NEVER pollute the answer the judge reads)
// ======================================================================

fn run_benchmark() {
    use std::time::Instant;
    // instant - a stopwatch

    // make a chunky graph so the timings actually mean something:
    // 1000 little chains of 100 cities -> 1000 islands to find
    let big_n = 100_000;
    let chain = 100;
    let mut big_edges: Vec<(usize, usize)> = Vec::new();
    let mut c = 1;
    while c + chain - 1 <= big_n {
        // each block: c .. c+99 wired in a line
        for k in 0..chain - 1 {
            big_edges.push((c + k, c + k + 1));
        }
        c += chain;
    }

    let reps = 5;
    // run each a few times so a random hiccup doesn't fool us

    let t1 = Instant::now();
    let mut k1 = 0;
    for _ in 0..reps {
        k1 = solve_dsu(big_n, &big_edges).len();
    }
    let d1 = t1.elapsed();

    let t2 = Instant::now();
    let mut k2 = 0;
    for _ in 0..reps {
        k2 = solve_heuristic(big_n, &big_edges).len();
    }
    let d2 = t2.elapsed();

    eprintln!("graph: {} cities, {} roads, {} runs each", big_n, big_edges.len(), reps);
    eprintln!("union-find : k={} total {:?}  (avg {:?})", k1, d1, d1 / reps);
    eprintln!("heuristic  : k={} total {:?}  (avg {:?})", k2, d2, d2 / reps);
    eprintln!("same answer? {}", k1 == k2);
    // dsu is near-linear; heuristic carries an extra log from the priority pile.
    // expect dsu to win, but both spit out the same k
}
