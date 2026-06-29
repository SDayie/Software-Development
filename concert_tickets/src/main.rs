use std::io::{self, Read, Write};
// std library io. Read lets us slurp all the input at once, Write lets us dump output fast.
use std::collections::BTreeMap;
// BTreeMap = a balanced search tree (a b-tree). it keeps its keys sorted for us. used in algorithm 2.

// =====================================================================
// algorithm 1 - segment tree (a "min tree")
// plan: sort the tickets cheap -> pricey. build a tree where every chunk
// remembers the SMALLEST price still left in it. to serve a customer we
// slide down the tree always grabbing the biggest price that still fits.
// runs in about (n + m) * log(n) steps.
// =====================================================================
fn solve_segment_tree(prices: &[i64], queries: &[i64]) -> Vec<i64> {
    let n = prices.len();
    // n = how many tickets we have.
    let mut sorted = prices.to_vec();
    // make our own copy so we don't mess up the caller's list.
    sorted.sort_unstable();
    // line them up cheapest -> priciest. once sorted, "biggest that fits" is just the rightmost that fits.
    const INF: i64 = i64::MAX;
    // INF = a fake huge price. we park it on empty / sold slots so they never look buyable.
    let mut size = 1usize;
    // size = number of leaves in the tree. it has to be a power of two.
    while size < n.max(1) { size <<= 1; }
    // keep doubling size until it covers every ticket. <<= 1 just means "times two".
    let mut tree = vec![INF; 2 * size];
    // the whole tree lives in one array. slot 1 is the root, the leaves sit in the back half.
    for i in 0..n { tree[size + i] = sorted[i]; }
    // drop each real price into its own leaf. any leftover leaves stay INF (empty shelf slots).
    for i in (1..size).rev() { tree[i] = tree[2 * i].min(tree[2 * i + 1]); }
    // fill the parents from the bottom up. each parent keeps the smaller of its two kids = min price in that chunk.
    let mut out = Vec::with_capacity(queries.len());
    // out = the answer we hand back, one number per customer.
    for &t in queries {
        // t = this customer's budget (the most they'll pay).
        if tree[1] > t {
            // the root holds the cheapest ticket left. if even that is too pricey, we can't help them.
            out.push(-1);
            // -1 = they leave empty handed.
            continue;
            // move on to the next customer.
        }
        let mut node = 1;
        // start at the root and walk downward.
        while node < size {
            // keep walking until we land on a real leaf (a single ticket).
            if tree[2 * node + 1] <= t {
                // the right kid holds the pricier half. if its cheapest still fits, the best deal lives over there.
                node = 2 * node + 1;
                // go right (greedy: grab the most expensive ticket that still fits the budget).
            } else {
                // otherwise everything affordable is in the cheaper left half.
                node = 2 * node;
                // go left.
            }
        }
        out.push(tree[node]);
        // we reached a leaf - that price is what this customer pays.
        tree[node] = INF;
        // sell it: blank the slot so nobody can buy this same ticket again.
        node >>= 1;
        // hop up to the parent. >>= 1 just means "divide by two".
        while node >= 1 {
            // climb back to the root, fixing every min we just disturbed.
            tree[node] = tree[2 * node].min(tree[2 * node + 1]);
            // each parent re-checks its two kids and keeps the smaller.
            node >>= 1;
            // keep climbing.
        }
    }
    out
    // hand back all the answers in order.
}

// =====================================================================
// algorithm 2 - b-tree multiset (BTreeMap)
// plan: pour every ticket into a sorted map of price -> how many we have.
// to serve a customer, ask the map for the biggest key that is <= budget.
// also about (n + m) * log(n), but the work is hidden inside the b-tree.
// =====================================================================
fn solve_btree(prices: &[i64], queries: &[i64]) -> Vec<i64> {
    let mut shelf: BTreeMap<i64, u32> = BTreeMap::new();
    // shelf = our stock. key = a price, value = how many tickets sit at that price (handles duplicates).
    for &p in prices {
        // walk through every ticket once.
        *shelf.entry(p).or_insert(0) += 1;
        // bump the count for that price. or_insert(0) starts it at zero if this price is brand new.
    }
    let mut out = Vec::with_capacity(queries.len());
    // answers go here.
    for &t in queries {
        // t = this customer's budget.
        if let Some((&price, _)) = shelf.range(..=t).next_back() {
            // range(..=t) = every price up to and including t. next_back() grabs the biggest of those = best fit.
            out.push(price);
            // they pay that price.
            let c = shelf.get_mut(&price).unwrap();
            // grab the counter for that price so we can take one off the shelf.
            *c -= 1;
            // one fewer ticket at this price.
            if *c == 0 { shelf.remove(&price); }
            // if that was the last one at this price, drop the key completely.
        } else {
            // nothing left on the shelf is cheap enough.
            out.push(-1);
            // empty handed.
        }
    }
    out
    // done.
}

fn main() {
    let mut input = String::new();
    // a fresh string to hold everything the judge feeds in.
    io::stdin().read_to_string(&mut input).unwrap();
    // slurp the whole input in one shot (way faster than reading line by line).
    let mut data = input.split_whitespace();
    // chop the input into number-shaped pieces, ignoring spaces and newlines.
    let n: usize = data.next().unwrap().parse().unwrap();
    // n = number of tickets. usize = a count, can't go negative.
    let m: usize = data.next().unwrap().parse().unwrap();
    // m = number of customers.
    let prices: Vec<i64> = (0..n).map(|_| data.next().unwrap().parse().unwrap()).collect();
    // read the next n numbers = the ticket prices.
    let queries: Vec<i64> = (0..m).map(|_| data.next().unwrap().parse().unwrap()).collect();
    // read the next m numbers = each customer's budget, in arrival order.

    let answers = solve_segment_tree(&prices, &queries);
    // solve it with algorithm 1. (both algorithms give the same answers - this is the one we ship.)

    let mut output = String::with_capacity(answers.len() * 4);
    // build all the output in one big string so we print only once (printing per line is slow).
    for a in &answers {
        // go through the answers in order.
        output.push_str(&a.to_string());
        // turn the number into text and tack it on.
        output.push('\n');
        // newline after each answer.
    }
    let stdout = io::stdout();
    // grab the standard output.
    let mut w = io::BufWriter::new(stdout.lock());
    // wrap it in a buffer so the one big write is fast.
    w.write_all(output.as_bytes()).unwrap();
    // dump the whole answer string. ONLY this clean answer ever hits stdout.

    let want_bench = std::env::var("RUN_BENCHMARK").is_ok()
        || std::env::args().any(|a| a == "--bench");
    // benchmark switch: on if RUN_BENCHMARK is set OR you pass --bench. off by default so the judge never runs it.
    if want_bench {
        run_benchmark(&prices, &queries);
        // only fires when you ask for it.
    }
}

// short benchmark - times both algorithms and checks they agree.
// everything here goes to stderr (eprintln!), never stdout, so it can't spoil the judge's answer.
fn run_benchmark(prices: &[i64], queries: &[i64]) {
    use std::time::{Duration, Instant};
    // Instant = a stopwatch, Duration = a length of time.
    let rounds: u32 = 5;
    // run each algorithm a few times so one noisy run doesn't fool us.
    let mut seg_total = Duration::ZERO;
    // running total of time spent in algorithm 1.
    let mut bt_total = Duration::ZERO;
    // running total of time spent in algorithm 2.
    let mut last_seg = Vec::new();
    // remember algorithm 1's last result so we can compare.
    let mut last_bt = Vec::new();
    // remember algorithm 2's last result so we can compare.
    for _ in 0..rounds {
        // repeat the whole thing a handful of times.
        let start = Instant::now();
        // start the stopwatch.
        last_seg = solve_segment_tree(prices, queries);
        // time algorithm 1.
        seg_total += start.elapsed();
        // add how long that took.
        let start = Instant::now();
        // restart the stopwatch.
        last_bt = solve_btree(prices, queries);
        // time algorithm 2.
        bt_total += start.elapsed();
        // add how long that took.
    }
    eprintln!("[bench] tickets={} customers={} rounds={}", prices.len(), queries.len(), rounds);
    // basic info about this run.
    eprintln!("[bench] algo 1 segment tree : {:?} total, {:?} avg", seg_total, seg_total / rounds);
    // algorithm 1 timing.
    eprintln!("[bench] algo 2 b-tree       : {:?} total, {:?} avg", bt_total, bt_total / rounds);
    // algorithm 2 timing.
    eprintln!("[bench] both agree          : {}", last_seg == last_bt);
    // sanity check: the two algorithms should always produce the exact same answers.
}
