use std::collections::HashSet;
// HashSet - a set built on hashing, it refuses to store the same value twice
use std::io::{self, Read};
// standard library input/output, Read lets us slurp the whole input in one go

// ----------------------------------------------------------------------
// Algorithm 1: Hashing (HashSet)
// ----------------------------------------------------------------------
fn count_distinct_hashset(values: &[i64]) -> usize {
    let mut seen = HashSet::new();
    // seen - empty hashset, this is where we throw every value
    for &v in values {
        seen.insert(v);
        // toss each value in, if it's already inside nothing changes
    }
    seen.len()
    // the size of the set IS the number of distinct values, done
}
// idea - hashing jumps each value straight to a bucket, no ordering kept
// cost - about O(n) on average, super fast but it does not sort anything

// ----------------------------------------------------------------------
// Algorithm 2: Balanced Binary Search Tree (AVL tree)
// ----------------------------------------------------------------------
struct Avl {
    key: Vec<i64>,
    // key - the number stored at each node
    left: Vec<i32>,
    // left - index of the left child, -1 means "no child"
    right: Vec<i32>,
    // right - index of the right child, -1 means "no child"
    height: Vec<i32>,
    // height - how tall the subtree under this node is, we use it to stay balanced
}
// we store the tree inside Vecs instead of pointers, way friendlier to rust
// a node is just a position (an index) shared across all four Vecs

impl Avl {
    fn new() -> Self {
        Avl { key: Vec::new(), left: Vec::new(), right: Vec::new(), height: Vec::new() }
        // start with no nodes at all, an empty tree
    }

    fn make_node(&mut self, k: i64) -> i32 {
        self.key.push(k);
        self.left.push(-1);
        self.right.push(-1);
        self.height.push(1);
        // a brand new node has no kids and height 1 (just itself)
        (self.key.len() - 1) as i32
        // its index is the last slot we just pushed into
    }

    fn h(&self, node: i32) -> i32 {
        if node < 0 { 0 } else { self.height[node as usize] }
        // empty spot counts as height 0, otherwise read the stored height
    }

    fn update(&mut self, node: i32) {
        let u = node as usize;
        let lh = self.h(self.left[u]);
        let rh = self.h(self.right[u]);
        self.height[u] = 1 + lh.max(rh);
        // my height = 1 + the taller of my two children
    }

    fn balance(&self, node: i32) -> i32 {
        let u = node as usize;
        self.h(self.left[u]) - self.h(self.right[u])
        // positive = left heavy, negative = right heavy, this tells us when to rotate
    }

    fn rotate_right(&mut self, y: i32) -> i32 {
        let yu = y as usize;
        let x = self.left[yu];
        let xu = x as usize;
        let t2 = self.right[xu];
        // x is y's left child, t2 is the subtree that has to switch parents
        self.right[xu] = y;
        self.left[yu] = t2;
        // swing x up to be the new top, y becomes x's right child
        self.update(y);
        self.update(x);
        // fix heights bottom up: the new child (y) first, then the new parent (x)
        x
        // x is the new root of this little piece
    }

    fn rotate_left(&mut self, x: i32) -> i32 {
        let xu = x as usize;
        let y = self.right[xu];
        let yu = y as usize;
        let t2 = self.left[yu];
        // mirror image of rotate_right, y is x's right child this time
        self.left[yu] = x;
        self.right[xu] = t2;
        // swing y up to be the new top, x becomes y's left child
        self.update(x);
        self.update(y);
        // again fix the lower node first, then the upper one
        y
        // y is the new root of this piece
    }

    fn insert(&mut self, node: i32, k: i64, inserted: &mut bool) -> i32 {
        if node < 0 {
            *inserted = true;
            return self.make_node(k);
        }
        // hit an empty spot? this value is new, plant a fresh node here
        let u = node as usize;
        if k < self.key[u] {
            let lc = self.left[u];
            let new_lc = self.insert(lc, k, inserted);
            self.left[u] = new_lc;
        // smaller than me -> go left and remember whatever comes back
        } else if k > self.key[u] {
            let rc = self.right[u];
            let new_rc = self.insert(rc, k, inserted);
            self.right[u] = new_rc;
        // bigger than me -> go right
        } else {
            return node;
        }
        // equal -> it's a duplicate, leave inserted false and bail out

        self.update(node);
        // the subtree changed, refresh this node's height
        let bf = self.balance(node);
        let lc = self.left[u];
        let rc = self.right[u];
        // bf tells us if we tipped over, lc/rc are the current children

        if bf > 1 && lc >= 0 && k < self.key[lc as usize] {
            return self.rotate_right(node);
        }
        // left-left case: too heavy on the left, one right rotation fixes it
        if bf < -1 && rc >= 0 && k > self.key[rc as usize] {
            return self.rotate_left(node);
        }
        // right-right case: too heavy on the right, one left rotation
        if bf > 1 && lc >= 0 && k > self.key[lc as usize] {
            let new_lc = self.rotate_left(lc);
            self.left[u] = new_lc;
            return self.rotate_right(node);
        }
        // left-right case: bend the left child first, then rotate right
        if bf < -1 && rc >= 0 && k < self.key[rc as usize] {
            let new_rc = self.rotate_right(rc);
            self.right[u] = new_rc;
            return self.rotate_left(node);
        }
        // right-left case: bend the right child first, then rotate left

        node
        // already balanced, hand back the same root
    }
}

fn count_distinct_avl(values: &[i64]) -> usize {
    let mut tree = Avl::new();
    let mut root: i32 = -1;
    // root - the whole tree, -1 because it starts empty
    let mut count: usize = 0;
    // count - how many genuinely new values we planted
    for &v in values {
        let mut inserted = false;
        // inserted - did this value actually create a node? assume no
        root = tree.insert(root, v, &mut inserted);
        if inserted {
            count += 1;
        }
        // only bump the count when the value was new, duplicates do nothing
    }
    count
    // every successful insert was one distinct value
}
// idea - the tree keeps everything sorted and self-balances after each insert
// cost - O(n log n), slower than hashing but the data stays ordered

// ----------------------------------------------------------------------
// fn main - the judge entry point, only the clean answer goes to stdout
// ----------------------------------------------------------------------
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    // read the entire input into one big string
    let mut data = input.split_whitespace();
    // chop it into separate tokens on any whitespace
    let n: usize = data.next().unwrap().parse().unwrap();
    // n - how many numbers follow, usize so it can't go negative
    let mut values: Vec<i64> = Vec::with_capacity(n);
    // values - where we collect the n numbers, i64 is roomy enough for 1e9
    for _ in 0..n {
        let x: i64 = data.next().unwrap().parse().unwrap();
        values.push(x);
        // read one number, stash it, repeat n times
    }

    let answer = count_distinct_hashset(&values);
    // use the fast hashing algorithm for the real answer
    println!("{}", answer);
    // print just the count, nothing else, exact-match judge is picky

    let run_bench = std::env::var("RUN_BENCHMARK").is_ok()
        || std::env::args().any(|a| a == "--bench");
    // benchmark only fires if you ask for it, env var OR a --bench flag
    if run_bench {
        benchmark();
    }
    // on the judge neither is set, so this never runs and never costs time
}

// ----------------------------------------------------------------------
// benchmark - times both algorithms on random data, prints to stderr
// ----------------------------------------------------------------------
fn benchmark() {
    use std::time::Instant;
    // Instant - a stopwatch from the standard library
    let n = 200_000;
    // n - the biggest list the problem allows, worst case for timing
    let mut state: u64 = 0x1234_5678_9abc_def0;
    // state - the seed for a tiny homemade random generator (no crates needed)
    let mut values: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        // xorshift - shuffles the bits around to fake randomness
        let v = (state % 1_000_000_000) as i64 + 1;
        values.push(v);
        // squeeze it into the 1..=1e9 range the problem uses
    }

    let t1 = Instant::now();
    let a = count_distinct_hashset(&values);
    let d1 = t1.elapsed();
    // start clock, run hashing, stop clock

    let t2 = Instant::now();
    let b = count_distinct_avl(&values);
    let d2 = t2.elapsed();
    // same for the balanced tree

    eprintln!("benchmark on n = {}", n);
    eprintln!("  hashset (hashing) : {} distinct in {:?}", a, d1);
    eprintln!("  avl (balanced bst): {} distinct in {:?}", b, d2);
    eprintln!("  match: {}", a == b);
    // eprintln goes to stderr so it never pollutes the judge's stdout answer
}
