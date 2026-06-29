use std::collections::BinaryHeap;
// BinaryHeap - the built in max-heap, biggest value on top

// this file is just a twin of main.rs's two functions, made public so tests/ can reach them.
// the judge never uses this - it only ever runs main.rs.

// ---------- algorithm 1: parity scan ----------
pub fn solve_parity(heaps: &[u64]) -> &'static str {
// first player wins if ANY heap is odd, else second
    let mut any_odd = false;
// flag, flips true when we hit an odd heap
    for &h in heaps {
// look at every heap
        if h % 2 == 1 {
// odd?
            any_odd = true;
// yep, mark it
            break;
// stop early, we have our answer
        }
    }
    if any_odd { "first" } else { "second" }
// odd somewhere -> first. all even -> second
}

// ---------- algorithm 2: heap ----------
pub fn solve_heap(heaps: &[u64]) -> &'static str {
// same answer via a max-heap of parities
    let mut parities: BinaryHeap<u64> = BinaryHeap::new();
// heap of 0s and 1s
    for &h in heaps {
// each heap
        parities.push(h % 2);
// push its parity, heap sorts so any 1 floats up
    }
    match parities.peek() {
// top item = biggest parity
        Some(&1) => "first",
// a 1 was up there -> odd heap existed -> first
        _ => "second",
// only 0s (or empty) -> all even -> second
    }
}
