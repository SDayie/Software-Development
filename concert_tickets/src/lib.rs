// lib.rs - a mirror of the two algorithms living in main.rs.
// the judge only ever runs main.rs (it's standalone). this copy exists purely
// so the integration tests in tests/ can import the functions and check them.
// keep these in sync with main.rs.

use std::collections::BTreeMap;
// b-tree map, used by algorithm 2.

// algorithm 1 - segment tree (min tree). sort tickets, then slide down the tree
// grabbing the biggest available price that still fits the budget.
pub fn solve_segment_tree(prices: &[i64], queries: &[i64]) -> Vec<i64> {
    let n = prices.len();
    // how many tickets.
    let mut sorted = prices.to_vec();
    // own copy so we don't wreck the caller's list.
    sorted.sort_unstable();
    // cheapest -> priciest.
    const INF: i64 = i64::MAX;
    // fake huge price for empty / sold slots.
    let mut size = 1usize;
    // leaf count, must be a power of two.
    while size < n.max(1) { size <<= 1; }
    // double until it covers everything.
    let mut tree = vec![INF; 2 * size];
    // tree in one array, root at slot 1, leaves in the back half.
    for i in 0..n { tree[size + i] = sorted[i]; }
    // real prices into leaves, leftovers stay INF.
    for i in (1..size).rev() { tree[i] = tree[2 * i].min(tree[2 * i + 1]); }
    // parents keep the smaller kid = min price in that chunk.
    let mut out = Vec::with_capacity(queries.len());
    // answers.
    for &t in queries {
        // t = budget.
        if tree[1] > t {
            // cheapest left is still too dear -> nobody can help.
            out.push(-1);
            continue;
        }
        let mut node = 1;
        // walk down from the root.
        while node < size {
            // until we hit a leaf.
            if tree[2 * node + 1] <= t {
                // pricier half still has something that fits -> best deal is there.
                node = 2 * node + 1;
            } else {
                // else look in the cheaper half.
                node = 2 * node;
            }
        }
        out.push(tree[node]);
        // leaf price = what they pay.
        tree[node] = INF;
        // sell it.
        node >>= 1;
        // climb up.
        while node >= 1 {
            // refresh the mins on the way to the root.
            tree[node] = tree[2 * node].min(tree[2 * node + 1]);
            node >>= 1;
        }
    }
    out
}

// algorithm 2 - b-tree multiset. price -> count map, ask for the biggest key <= budget.
pub fn solve_btree(prices: &[i64], queries: &[i64]) -> Vec<i64> {
    let mut shelf: BTreeMap<i64, u32> = BTreeMap::new();
    // price -> how many tickets at that price.
    for &p in prices {
        *shelf.entry(p).or_insert(0) += 1;
        // count each ticket.
    }
    let mut out = Vec::with_capacity(queries.len());
    for &t in queries {
        // t = budget.
        if let Some((&price, _)) = shelf.range(..=t).next_back() {
            // biggest price <= budget = best fit.
            out.push(price);
            let c = shelf.get_mut(&price).unwrap();
            *c -= 1;
            // take one off the shelf.
            if *c == 0 { shelf.remove(&price); }
            // drop the key if that was the last one.
        } else {
            out.push(-1);
            // nothing cheap enough.
        }
    }
    out
}
