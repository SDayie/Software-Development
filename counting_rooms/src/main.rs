use std::io::{self, Read};
// std library, input/output stuff; self lets us write io::stdin(), Read lets us call read_to_string
use std::time::Instant;
// Instant - basically a stopwatch, used later for the benchmark
use std::collections::HashSet;
// HashSet - a list that drops duplicates automatically

// ALGORITHM 1: flood fill, using a stack instead of recursion
// spreads out from a floor square like paint, everywhere it reaches counts as one room
// once it stops spreading we move to the next unvisited floor square, which starts the next room
pub fn count_rooms_flood_fill(grid: &[Vec<u8>], n: usize, m: usize) -> usize {
    let mut visited = vec![vec![false; m]; n];
    // visited - tracks which squares we already counted, starts all false
    let mut room_count = 0;
    let mut stack: Vec<(usize, usize)> = Vec::new();
    // stack - our to-do list of squares to check, a Vec instead of recursion so we don't overflow on huge maps

    for row in 0..n {
        for col in 0..m {
            if grid[row][col] == b'.' && !visited[row][col] {
                // unvisited floor square = brand new room
                room_count += 1;
                visited[row][col] = true;
                stack.push((row, col));

                while let Some((r, c)) = stack.pop() {
                    // pop the last square added, check its 4 neighbours

                    if r > 0 && grid[r - 1][c] == b'.' && !visited[r - 1][c] {
                        visited[r - 1][c] = true;
                        stack.push((r - 1, c));
                    }
                    if r + 1 < n && grid[r + 1][c] == b'.' && !visited[r + 1][c] {
                        visited[r + 1][c] = true;
                        stack.push((r + 1, c));
                    }
                    if c > 0 && grid[r][c - 1] == b'.' && !visited[r][c - 1] {
                        visited[r][c - 1] = true;
                        stack.push((r, c - 1));
                    }
                    if c + 1 < m && grid[r][c + 1] == b'.' && !visited[r][c + 1] {
                        visited[r][c + 1] = true;
                        stack.push((r, c + 1));
                    }
                    // up, down, left, right - same check 4 times, order doesn't matter
                    // any one we find unvisited and floor gets marked and added to the stack
                }
                // stack empties once the whole room is marked, then we move on
            }
        }
    }

    room_count
}

// ALGORITHM 2: union-find (DSU) 
// every floor square starts in its own team, touching floor squares get merged, teams left = rooms
// no spreading involved here, just merging pairs as we scan through the grid

struct UnionFind {
    parent: Vec<usize>,
    // parent - who's the boss of each square's team, parent[i] == i means i is the boss
    size: Vec<usize>,
    // size - how many squares are on the team led by i, used when deciding which team merges into which
}

impl UnionFind {
    fn new(count: usize) -> Self {
        UnionFind {
            parent: (0..count).collect(),
            // everyone starts as their own boss
            size: vec![1; count],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        // loop instead of recursion, same reason as the stack above - avoids overflow on big grids
        // (a chain of bosses pointing to bosses could get long before any compression happens)
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        // root = the real boss, found by climbing up the chain

        let mut current = x;
        while self.parent[current] != root {
            let next = self.parent[current];
            self.parent[current] = root;
            current = next;
        }
        // path compression - point everyone we just climbed past straight at root, makes the next find() faster
        // so the chain gets shorter every time someone calls find() on it

        root
    }

    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a == root_b {
            return;
        }
        // already same team, nothing to do

        if self.size[root_a] < self.size[root_b] {
            self.parent[root_a] = root_b;
            self.size[root_b] += self.size[root_a];
        } else {
            self.parent[root_b] = root_a;
            self.size[root_a] += self.size[root_b];
        }
        // union by size - smaller team joins the bigger one, keeps chains short so find() stays quick
    }
}

pub fn count_rooms_union_find(grid: &[Vec<u8>], n: usize, m: usize) -> usize {
    let mut uf = UnionFind::new(n * m);
    // one team per square on the map, walls get a team too, just never merged

    let index = |r: usize, c: usize| -> usize { r * m + c };
    // turns row,col into one flat number, since UnionFind uses a flat Vec not a grid

    for r in 0..n {
        for c in 0..m {
            if grid[r][c] == b'.' {
                if c + 1 < m && grid[r][c + 1] == b'.' {
                    uf.union(index(r, c), index(r, c + 1));
                }
                if r + 1 < n && grid[r + 1][c] == b'.' {
                    uf.union(index(r, c), index(r + 1, c));
                }
                // only check right and down - left and up already got checked from the other square's turn
                // so every touching pair of floor squares still gets merged exactly once
            }
        }
    }

    let mut bosses_seen: HashSet<usize> = HashSet::new();
    for r in 0..n {
        for c in 0..m {
            if grid[r][c] == b'.' {
                let boss = uf.find(index(r, c));
                bosses_seen.insert(boss);
            }
        }
    }
    // bosses_seen - a set, so the same team only ever gets counted once

    bosses_seen.len()
}

// MAIN: reads the map, solves it for real, prints the judge's answer
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    // read everything from stdin in one go, faster than line by line on a big map

    let mut tokens = input.split_whitespace();
    // splits on spaces/newlines - works since each map row has no spaces inside it

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let m: usize = tokens.next().unwrap().parse().unwrap();
    // n = rows, m = columns

    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(n);
    for _ in 0..n {
        let row = tokens.next().unwrap();
        grid.push(row.as_bytes().to_vec());
        // store as raw bytes, cheaper to compare than chars
    }

    let room_count = count_rooms_flood_fill(&grid, n, m);
    println!("{}", room_count);
    // this is the only line the judge actually checks

    run_benchmark();
    // extra - times both algorithms, doesn't affect the real answer above
}

// BENCHMARK: build a big test map and time both algorithms on it
fn run_benchmark() {
    // everything here prints to stderr, so it never touches the judge's expected stdout
    let n = 1000;
    let m = 1000;
    // biggest size the problem allows, this is where speed differences actually show up

    let big_grid = generate_maze_grid(n, m);
    // a maze grid full of tiny rooms, a rough test case for both algorithms
    // lots of small rooms means lots of separate union-find teams to track, and lots of stack pushes for flood fill

    let start_flood = Instant::now();
    let result_flood = count_rooms_flood_fill(&big_grid, n, m);
    let time_flood = start_flood.elapsed();

    let start_uf = Instant::now();
    let result_uf = count_rooms_union_find(&big_grid, n, m);
    let time_uf = start_uf.elapsed();

    eprintln!("--- Benchmark on a {}x{} maze-like grid ---", n, m);
    eprintln!(
        "Flood Fill (stack-based DFS): {} rooms found in {:?}",
        result_flood, time_flood
    );
    eprintln!(
        "Union-Find (DSU):             {} rooms found in {:?}",
        result_uf, time_uf
    );

    assert_eq!(
        result_flood, result_uf,
        "the two algorithms disagree -- something is broken!"
    );
    // sanity check - two correct algorithms must always agree
}

fn generate_maze_grid(n: usize, m: usize) -> Vec<Vec<u8>> {
    let mut grid = vec![vec![b'.'; m]; n];
    for r in 0..n {
        for c in 0..m {
            if r % 2 == 1 && c % 2 == 1 {
                grid[r][c] = b'#';
            }
        }
    }
    // walls on every odd row+col, chops the floor into lots of small separate rooms
    grid
}

// BENCHMARK INTERPRETATION-----------------------------------------------------------------------------------------------------------------

// Both versions solve the same problem — counting connected rooms in a grid — so the core result is identical. 
// //The difference is in how they process and organise the computation, which affects performance.

// The 1st algorithm (flood fill) uses a stack to go through each room and marks visited cells as it goes. 
// Each cell is processed once, and no extra data structures are needed after the search finishes.

// The 2nd algorithm (union-find ) first goes through the grid and connects neighboring floor cells,
// then scans the grid again to find each group’s representative and stores it in a HashSet to count rooms. 
// This means it does extra work after the merging step, including repeated find operations and hashing.

// So the main difference is the extra work: 1st algorithm (flood fill) is faster 
// since it does everything in a single pass using simple marking.
//  The 2nd algorithm (union-find ) requires another full scan of the whole grid and extra steps to count components. 
// Because of this, union-find has more constant work in this benchmark, even though both have the same asymptotic complexity. 
