// lib.rs only exists so the integration tests can import the algorithms.
// main.rs stays self-contained for the judge, this is its test-facing twin.

use std::collections::HashSet;

// Algorithm 1: Hashing (HashSet)
pub fn count_distinct_hashset(values: &[i64]) -> usize {
    let mut seen = HashSet::new();
    // seen - drops duplicates automatically
    for &v in values {
        seen.insert(v);
    }
    seen.len()
    // size of the set = number of distinct values
}

// Algorithm 2: Balanced Binary Search Tree (AVL tree)
struct Avl {
    key: Vec<i64>,
    left: Vec<i32>,
    right: Vec<i32>,
    height: Vec<i32>,
}
// nodes live inside Vecs, a node is just an index, -1 means "no child"

impl Avl {
    fn new() -> Self {
        Avl { key: Vec::new(), left: Vec::new(), right: Vec::new(), height: Vec::new() }
    }

    fn make_node(&mut self, k: i64) -> i32 {
        self.key.push(k);
        self.left.push(-1);
        self.right.push(-1);
        self.height.push(1);
        // fresh node, no kids, height 1
        (self.key.len() - 1) as i32
    }

    fn h(&self, node: i32) -> i32 {
        if node < 0 { 0 } else { self.height[node as usize] }
        // empty spot is height 0
    }

    fn update(&mut self, node: i32) {
        let u = node as usize;
        let lh = self.h(self.left[u]);
        let rh = self.h(self.right[u]);
        self.height[u] = 1 + lh.max(rh);
        // height = 1 + taller child
    }

    fn balance(&self, node: i32) -> i32 {
        let u = node as usize;
        self.h(self.left[u]) - self.h(self.right[u])
        // + means left heavy, - means right heavy
    }

    fn rotate_right(&mut self, y: i32) -> i32 {
        let yu = y as usize;
        let x = self.left[yu];
        let xu = x as usize;
        let t2 = self.right[xu];
        self.right[xu] = y;
        self.left[yu] = t2;
        self.update(y);
        self.update(x);
        // lower node first, then the new parent
        x
    }

    fn rotate_left(&mut self, x: i32) -> i32 {
        let xu = x as usize;
        let y = self.right[xu];
        let yu = y as usize;
        let t2 = self.left[yu];
        self.left[yu] = x;
        self.right[xu] = t2;
        self.update(x);
        self.update(y);
        // mirror of rotate_right
        y
    }

    fn insert(&mut self, node: i32, k: i64, inserted: &mut bool) -> i32 {
        if node < 0 {
            *inserted = true;
            return self.make_node(k);
        }
        // empty spot -> brand new value
        let u = node as usize;
        if k < self.key[u] {
            let lc = self.left[u];
            let new_lc = self.insert(lc, k, inserted);
            self.left[u] = new_lc;
        } else if k > self.key[u] {
            let rc = self.right[u];
            let new_rc = self.insert(rc, k, inserted);
            self.right[u] = new_rc;
        } else {
            return node;
        }
        // equal -> duplicate, do nothing

        self.update(node);
        let bf = self.balance(node);
        let lc = self.left[u];
        let rc = self.right[u];

        if bf > 1 && lc >= 0 && k < self.key[lc as usize] {
            return self.rotate_right(node);
        }
        // left-left
        if bf < -1 && rc >= 0 && k > self.key[rc as usize] {
            return self.rotate_left(node);
        }
        // right-right
        if bf > 1 && lc >= 0 && k > self.key[lc as usize] {
            let new_lc = self.rotate_left(lc);
            self.left[u] = new_lc;
            return self.rotate_right(node);
        }
        // left-right
        if bf < -1 && rc >= 0 && k < self.key[rc as usize] {
            let new_rc = self.rotate_right(rc);
            self.right[u] = new_rc;
            return self.rotate_left(node);
        }
        // right-left

        node
    }
}

pub fn count_distinct_avl(values: &[i64]) -> usize {
    let mut tree = Avl::new();
    let mut root: i32 = -1;
    let mut count: usize = 0;
    for &v in values {
        let mut inserted = false;
        root = tree.insert(root, v, &mut inserted);
        if inserted {
            count += 1;
        }
        // count only the genuinely new values
    }
    count
}
