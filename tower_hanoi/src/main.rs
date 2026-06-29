use std::io::{self, BufRead};
use std::time::Instant;
// use standard input/output library, allows you to enter numbers in terminal

// ============================================================
// ALGORITHM 1: RECURSIVE
// ============================================================

fn tower_of_hanoi(
    n: u8,
    source_pole: u8,
    auxiliary_pole: u8,
    destination_pole: u8,
    moves: &mut Vec<(u8, u8)>,
) {
// tower_of_hanoi is the function name. n = number of disks.
// source_pole = where disks start, auxiliary_pole = helper stack, destination_pole = where disks finally need to go
// moves - stores every move we make as a pair of numbers (from, to)
// &mut - we pass moves by reference so the same list gets updated every time the function calls itself

    if n == 1 {
        moves.push((source_pole, destination_pole));
        return;
    }
// if there's only 1 disk left, just move it straight from source to destination, no thinking needed
// .push - adds that move to our list. return - we're done with this branch, go back up

    tower_of_hanoi(n - 1, source_pole, destination_pole, auxiliary_pole, moves);
// before we can move the biggest disk, we move everything above it out of the way
// we flip destination and auxiliary here because the helper stack becomes the target temporarily

    moves.push((source_pole, destination_pole));
// now the bottom disk is free, move it directly to where it needs to go

    tower_of_hanoi(n - 1, auxiliary_pole, source_pole, destination_pole, moves);
// now grab all the disks we parked on the helper stack and move them on top of the big disk
// source and auxiliary flip again because now the helper stack is where the disks are coming from
}

pub fn solve_recursive(n: u8) -> Vec<(u8, u8)> {
    let mut moves = Vec::new();
// let mut - because moves grows every time we add to it. Vec - a list that can grow, we don't know the size upfront

    if n > 0 {
        tower_of_hanoi(n, 1, 2, 3, &mut moves);
    }
// stack 1 = left (start), stack 2 = middle (helper), stack 3 = right (destination)
// &mut moves - pass the list in so the function can keep adding to it

    moves
// return the finished list of moves
}

// ============================================================
// ALGORITHM 2: ITERATIVE
// ============================================================

pub fn solve_iterative(n: u8) -> Vec<(u8, u8)> {
    if n == 0 {
        return Vec::new();
    }
// if there are no disks, return an empty list, nothing to do

    let mut stacks: [Vec<u8>; 3] = [vec![], vec![], vec![]];
// create 3 actual stacks as vectors. index 0 = stack 1 (left), index 1 = middle, index 2 = right
// [Vec<u8>; 3] - an array of exactly 3 vectors, each holding u8 numbers (disk sizes)

    for disk in (1..=n).rev() {
        stacks[0].push(disk);
    }
// load all disks onto the left stack biggest first (bottom to top)
// .rev() - reverses the order so disk n goes in first (bottom), disk 1 goes in last (top)

    let mut moves: Vec<(u8, u8)> = Vec::new();
// moves holds every (from, to) pair. let mut because it grows with every step

    let total_moves = (1u64 << n) - 1;
// total moves is always 2^n - 1. 
// 1u64 << n - shifts the number 1 left by n bits, same as 2^n. subtract 1 to get the exact count

    let mut smallest_disk_pos: usize = 0;
// track which stack the smallest disk is sitting on. starts at 0 (left stack)
// key - the smallest disk always rotates in one direction

    for step in 0..total_moves {
        if step % 2 == 0 {
// even steps - always move the smallest disk. it has a fixed rotation pattern it follows every time

            let next_pos = if n % 2 == 1 {
                (smallest_disk_pos + 2) % 3
            } else {
                (smallest_disk_pos + 1) % 3
            };
// if n is odd, smallest disk rotates left: 0 -> 2 -> 1 -> 0
// if n is even, smallest disk rotates right: 0 -> 1 -> 2 -> 0
// % 3 wraps it back around so it never goes out of bounds

            let disk = stacks[smallest_disk_pos].pop().unwrap();
            stacks[next_pos].push(disk);
            moves.push((smallest_disk_pos as u8 + 1, next_pos as u8 + 1));
            smallest_disk_pos = next_pos;
// remove the smallest disk off its current stack, move it onto the next one
// +1 because our stacks are 0-indexed internally but the output uses 1, 2, 3
// update smallest_disk_pos so we know where it is next time

        } else {
// odd steps - make the only other legal move. there's always exactly one valid move that doesn't involve the smallest disk

            let candidates: Vec<usize> = (0..3).filter(|&s| s != smallest_disk_pos).collect();
            let s0 = candidates[0];
            let s1 = candidates[1];
// find the two stacks that don't have the smallest disk
// .filter - keeps only stacks that aren't where the smallest disk is sitting
// .collect - gathers them into a list

            let (from, to) = match (stacks[s0].last(), stacks[s1].last()) {
                (None, _) => (s1, s0),
                (_, None) => (s0, s1),
                (Some(&a), Some(&b)) if a < b => (s0, s1),
                _ => (s1, s0),
            };
// .last() - peek at the top disk without removing it
// if one stack is empty, move from the other one. no choice there
// if both have disks, move from whichever has the smaller top disk
// you can never put a bigger disk on a smaller one, so this is always the only legal move

            let disk = stacks[from].pop().unwrap();
            stacks[to].push(disk);
            moves.push((from as u8 + 1, to as u8 + 1));
// remove from the chosen stack, move to the target, record the move
        }
    }

    moves
// return the complete list of moves
}



fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).expect("Failed to read input");
// io::stdin - reads from the terminal. read_line - waits for you to type a number and press enter
// &mut line - pass the string by reference so read_line can write into it

    let n: u8 = line.trim().parse().expect("Please enter a number between 1 and 16");
// .trim() removes extra spaces and the Enter key character so the input can be converted into a number
// .parse() - converts the string "5" into the actual number 5
// u8 - unsigned 8-bit integer, big enough for 1 to 16

    let moves = solve_recursive(n);
// solve using algorithm 1 and store all the moves

    println!("{}", moves.len());
    for &(a, b) in &moves {
        println!("{} {}", a, b);
    }
// first print the total number of moves, then print each move on its own line
// &(a, b) - destructure each pair so we can print a and b separately

    let moves_iter = solve_iterative(n);
    assert_eq!(moves.len(), moves_iter.len(), "BUG: algorithms disagree on move count for n={n}!");
// run algorithm 2 and make sure it gets the same number of moves as algorithm 1
// assert_eq - crashes with a message if the two values don't match. good safety check

    // ============================================================
    // BENCHMARK
    // ============================================================

    eprintln!("\n=== Benchmark (n=16, 300 runs each) ===");
// eprintln - prints to stderr (rough work / side notes) 
// stdout (standard output) — this is the "main" output. When you do println!, it goes here.
//stderr (standard error) — this is a separate space for extra info like benchmarks. When you do eprintln!, it goes here.

    let _ = solve_recursive(16);
    let start = Instant::now();
    for _ in 0..300 { let _ = solve_recursive(16); }
    let rec_elapsed = start.elapsed();
    eprintln!(
        "  [Recursive] total={:?} | avg={:.2}µs",
        rec_elapsed,
        rec_elapsed.as_micros() as f64 / 300.0
    );
// run recursive 300 times and measure total time. divide by 300 to get the average per run
// Instant::now() - starts the stopwatch. .elapsed() - stops it and gives us the duration

    let _ = solve_iterative(16);
    let start = Instant::now();
    for _ in 0..300 { let _ = solve_iterative(16); }
    let itr_elapsed = start.elapsed();
    eprintln!(
        "  [Iterative] total={:?} | avg={:.2}µs",
        itr_elapsed,
        itr_elapsed.as_micros() as f64 / 300.0
    );
// same thing for iterative. the first call before the loop is a warmup so the results are fair

    eprintln!("========================================");
    eprintln!("(Recursive wins here! Iterative is slower because it pushes/pops real Vec stacks on every step.)");
}

// BENCHMARK INTERPRETATION
//The benchmark shows that the recursive algorithm is faster than the iterative algorithm for 16 disks. 
//The recursive version mainly follows a mathematical pattern and records moves, 
//while the iterative version must constantly manage stacks and perform extra checks. 
//Although both produce the same number of moves, 
//the iterative algorithm does more work each step, making it slower.
