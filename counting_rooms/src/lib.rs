// this is the library half of the project - same two algorithms as main.rs,
// just exposed as a proper crate so tests/ can import them normally instead
// of hacking main.rs in with #[path]

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
    // (a 1000x1000 map is up to a million squares, a recursive call that deep could crash)

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

// ALGORITHM 2: union-find (DSU) - a different way to do the same job
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
