use std::io::{self, Read};
// std library, input/output stuff. read - lets us slurp the whole input in one go
use std::collections::BinaryHeap;
// BinaryHeap - rust's built in max-heap (priority queue). the biggest value always sits on top

// ---------- algorithm 1: parity scan ----------
fn solve_parity(heaps: &[u64]) -> &'static str {
    let mut any_odd = false;
// any_odd - a flag. starts false, flips true the second we spot one odd heap
    for &h in heaps {
// walk through every heap, one at a time
        if h % 2 == 1 {
// h % 2 - the leftover after dividing by 2. if it's 1 the heap is odd
            any_odd = true;
// found an odd one, raise the flag
            break;
// no point checking the rest, bail out early
        }
    }
    if any_odd { "first" } else { "second" }
// odd heap exists -> first wins. all even -> second wins
}

// ---------- algorithm 2: heap ----------
fn solve_heap(heaps: &[u64]) -> &'static str {
    let mut parities: BinaryHeap<u64> = BinaryHeap::new();
// parities - a max-heap holding 0s and 1s. 0 = that heap was even, 1 = that heap was odd
    for &h in heaps {
// go through each heap
        parities.push(h % 2);
// push its parity in. heap re-sorts itself so the largest drifts to the top
    }
    match parities.peek() {
// peek - look at the top item without taking it out. top = the biggest parity seen
        Some(&1) => "first",
// top is 1 -> at least one odd heap -> first wins
        _ => "second",
// top is 0 (or heap empty) -> everything was even -> second wins
    }
}

fn main() {
    let mut input = String::new();
// input - an empty string we'll dump the whole stdin into
    io::stdin().read_to_string(&mut input).unwrap();
// read everything from stdin straight into input
    let mut data = input.split_whitespace();
// split it on spaces/newlines so each number is its own little chunk
    let t: usize = data.next().unwrap().parse().unwrap();
// t - how many test cases. usize because a count is never negative
    let mut out = String::new();
// out - one big string we build up, then print once at the end (way faster than many prints)
    for _ in 0..t {
// loop once per test case
        let n: usize = data.next().unwrap().parse().unwrap();
// n - number of heaps in this test
        let mut heaps: Vec<u64> = Vec::with_capacity(n);
// heaps - a vector to hold this test's coin counts. pre-sized to n so it won't keep resizing
        for _ in 0..n {
// read n numbers
            let x: u64 = data.next().unwrap().parse().unwrap();
// x - coins in one heap (can be up to a billion, u64 is comfy)
            heaps.push(x);
// stash it
        }
        out.push_str(solve_parity(&heaps));
// run algo 1 (the fast one) and append "first"/"second" to our output
        out.push('\n');
// newline after each answer
    }
    print!("{}", out);
// print the whole thing once. ONLY the clean answers hit stdout, nothing else

    if std::env::var("RUN_BENCHMARK").is_ok() || std::env::args().any(|a| a == "--bench") {
        run_benchmark();
    }
}

// ---------- benchmark (never runs on the judge) ----------
fn run_benchmark() {
    use std::time::Instant;
// Instant - a stopwatch. read the clock before and after to measure how long something took

    let n = 2_000_000;
// n - size of our fake test. 2 million heaps, big enough to actually feel the gap
    let mut heaps: Vec<u64> = Vec::with_capacity(n);
// room for n numbers up front
    let mut seed: u64 = 88172645463325252;
// seed - the starting number for a tiny homemade random generator
    for _ in 0..n {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
// xorshift - 3 fast bit-twists that scramble seed into the next "random" number
        heaps.push((seed % 1_000_000_000) * 2 + 2);
// force it even (times 2) so neither algo gets to quit early - a fair fight where both scan everything
    }

    let rounds = 20;
// run each algo 20 times so the timing is steady, not just luck

    let t1 = Instant::now();
// start the stopwatch for algo 1
    let mut sink1 = "";
    for _ in 0..rounds {
        sink1 = solve_parity(&heaps);
// keep the result in sink1 so the compiler can't be clever and skip the work
    }
    let d1 = t1.elapsed();
// d1 - total time algo 1 took

    let t2 = Instant::now();
// start the stopwatch for algo 2
    let mut sink2 = "";
    for _ in 0..rounds {
        sink2 = solve_heap(&heaps);
    }
    let d2 = t2.elapsed();
// d2 - total time algo 2 took

    eprintln!("benchmark: {} heaps, {} rounds each (all-even input, no early exit)", n, rounds);
    eprintln!("algo1 parity scan : {:?}  -> {}", d1, sink1);
    eprintln!("algo2 heap        : {:?}  -> {}", d2, sink2);
// eprintln - prints to stderr, NOT stdout, so the judge's answer stays clean
// both should say the same thing; the heap one is slower since it allocates + sorts as it goes
}
