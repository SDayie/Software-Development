use std::io::{self, Read};
use std::time::Instant;
// use standard input/output, and Instant for timing

// ============================================================
// ALGORITHM 1: QUEUE
// ============================================================
// builds the order vector by looping over it while it's still growing

pub fn solve_queue(n: usize, input_numbers: &[usize]) -> Vec<usize> {
// everything below this line is my original code, untouched

let mut children: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
// children - a vector that holds the subordinates under each employee, it's a vector of vectors(the family tree)
for employee in 2..=n {
let boss: usize = input_numbers[employee - 2];
children[boss].push(employee);
    }
// for employee in 2..=n - we start here from value 2, beacuse the 1st value is always the general boss
// push the subordinates we find into the children value
let mut order: Vec<usize> = Vec::new();
order.push(1);
//creates an empty vector called order, that can change
// start with 1-the boss
let mut index = 0;
// points the position in the order vector
while index < order.len() {
let current_employee = order[index];
// the while loop, loops over the order variable to check if we still have employees left
for &child in &children[current_employee] {
order.push(child);
        }
//go through all direct workers under the current employee
index += 1;
    }
// index += 1 : it tells the computer to move to the next employee    
let mut subordinates: Vec<usize> = vec![0; n + 1];
// it creates a vector to store subordinates for each and every employee
for &employee in order.iter().rev() {
for &child in &children[employee] {
subordinates[employee] += 1 + subordinates[child];
        }
    }
// for &employee in order.iter().rev() = reverse the list of employees(from eg 1-6 to 6-1) to count the children under each employee
//subordinates[employee] += 1 + subordinates[child] = it starts at zero, and if it finds a subordinate it increases by 1, until all subordinates are counted for the current employee
subordinates
// return the finished vector instead of printing it, so main can call this twice for the benchmark
}

// ============================================================
// ALGORITHM 2: TREE (recursive)
// ============================================================
// instead of a vector we loop over, this one calls itself and uses rust's own call stack

fn count_subordinates_recursive(
    employee: usize,
    children: &Vec<Vec<usize>>,
    subordinates: &mut Vec<usize>,
) -> usize {
// employee - who we're counting for. children - the tree, read only. subordinates - the answer vector we fill in

    let mut total = 0;
    for &child in &children[employee] {
        total += 1 + count_subordinates_recursive(child, children, subordinates);
    }
    // for every child, add 1 for the child plus everyone under them - found by calling this function again

    subordinates[employee] = total;
    // save the total once we know it

    total
    // hand it back up to whoever called us
}

pub fn solve_tree(n: usize, input_numbers: &[usize]) -> Vec<usize> {
// same job as solve_queue, but recursive instead

let mut children: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
// same family tree setup as algorithm 1
for employee in 2..=n {
let boss: usize = input_numbers[employee - 2];
children[boss].push(employee);
    }
// employee 1 is always the boss so we skip it and start from 2

    let mut subordinates: Vec<usize> = vec![0; n + 1];

    if n >= 1 {
        count_subordinates_recursive(1, &children, &mut subordinates);
    }
    // start the recursion at employee 1, it fills in everyone else on the way back up

    subordinates
}

fn main() {
    // runs everything on a thread with a bigger stack, since algorithm 2 recurses
    // deep on a chain-shaped input and the default 8MB stack might not be enough
    let builder = std::thread::Builder::new().stack_size(64 * 1024 * 1024);
    let handle = builder.spawn(run).unwrap();
    handle.join().unwrap();
}

fn run() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    // read everything typed in into one string

    let mut data = input.split_whitespace();
    // separate inputs into different values with white space

    let n: usize = data.next().unwrap().parse().unwrap();
    // usize means n can't be negative, only positive values

    let input_numbers: Vec<usize> = data.map(|x| x.parse().unwrap()).collect();
    // grab the rest of the numbers (the boss list) all at once

    let subordinates = solve_queue(n, &input_numbers);
    // run algorithm 1 and store the answer

    let result: Vec<String> = (1..=n)
        .map(|employee| subordinates[employee].to_string())
        .collect();
    println!("{}", result.join(" "));
    // prints the number of subordinates for each employee

    // the benchmark below only runs if you pass --bench on the command line
    // (e.g. "cargo run --release -- --bench"). on a judge, nothing extra ever
    // gets typed after the program name, so this whole block is skipped and
    // the program just prints the answer above and exits right away
    let args: Vec<String> = std::env::args().collect();
    if !args.iter().any(|a| a == "--bench") {
        return;
    }

    let subordinates_tree = solve_tree(n, &input_numbers);
    // run algorithm 2 on the same input

    assert_eq!(
        subordinates, subordinates_tree,
        "BUG: the two algorithms don't agree, something's wrong!"
    );
    // safety check, both algorithms should always agree

    // ============================================================
    // BENCHMARK
    // ============================================================

    const RUNS: u32 = 200;
    let bench_n: usize = 200_000;
    let bench_bosses: Vec<usize> = (1..bench_n).collect();
    // build a worst-case chain: 1 -> 2 -> 3 -> ... -> n

    eprintln!("\n=== Benchmark (n={bench_n}, {RUNS} runs each) ===");
    // eprintln prints to stderr, side notes that don't mess with the real answer on stdout

    let _ = solve_queue(bench_n, &bench_bosses);
    // warmup run, thrown away

    let start = Instant::now();
    for _ in 0..RUNS {
        let _ = solve_queue(bench_n, &bench_bosses);
    }
    let queue_elapsed = start.elapsed();
    eprintln!(
        "  [Queue] total={:?} | avg={:.2}µs",
        queue_elapsed,
        queue_elapsed.as_micros() as f64 / RUNS as f64
    );

    let _ = solve_tree(bench_n, &bench_bosses);

    let start = Instant::now();
    for _ in 0..RUNS {
        let _ = solve_tree(bench_n, &bench_bosses);
    }
    let tree_elapsed = start.elapsed();
    eprintln!(
        "  [Tree ] total={:?} | avg={:.2}µs",
        tree_elapsed,
        tree_elapsed.as_micros() as f64 / RUNS as f64
    );

    eprintln!("========================================");

    
// BENCHMARK INTERPRETATION-----------------------------------------------------------------------------------------------------------------

//Both versions produce the same result, 
//and both walk through every employee once to count how many people work under them, 
//so the basic work is the same. 

//1st algorithm (queue) builds an extra order list as it goes. 
//Every employee is added to that list once, and when the list gets full, 
//Rust sometimes has to make a bigger one and move the old values into a bigger list. 
//This doesn’t happen often, but it adds extra time.

//2nd algorithm (tree) doesn’t build that list. 
//Instead, it uses recursion, meaning the function calls itself for each employee. 
//In the benchmark, the input is a long chain (1 → 2 → 3 → ... → n), 
//so the program has to keep about 200,000 function calls open at once before it can finish. 
//That takes extra time because the computer has to remember all those unfinished calls.

//Both algorithms still visit every employee once and 
//calculate the subordinate counts in the same way, so the main work is identical. 
//The difference is that the 1st algorithm (queue) spends a little time resizing its list, 
//while the 2nd algorithm (tree) spends time handling many nested function calls. 
//Both do the same counting—they just manage the work in different ways, 
//but the queue version is faster because it uses simple looping instead of many nested function calls.


}
