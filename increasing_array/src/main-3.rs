use std::io;
// use standard input/output library, allows you to enter numbers in terminal
use std::time::Instant;
// Instant - lets us start and stop a stopwatch to time our code


// ALGORITHM 1 (in-place mutation) -----------------------------------------------------------------------------------------------
pub fn increasing_array_v1(numbers: &mut Vec<i64>) -> i64 {
    let mut moves: i64 = 0;
    // use let mut - because it can change and grow. 
    // moves - holds the values when when the numbers are decreasing
    let n = numbers.len();
    for i in 1..n {
        if numbers[i] < numbers[i - 1] {
            moves += numbers[i - 1] - numbers[i];
            numbers[i] = numbers[i - 1];
        }
    }
    // for i in 1..n     n holds the array [3 2 5 1 7]
    // for i(index), compare i = 2 to (i-1)=3
    // moves : 3-2 = 1; so store the difference (number 1);
    // if it's increasing it ignores it; if it's decreasing, subtract the values and add it to moves
    // index[1] and index[3]
    //[3, 2, 5, 1, 7]
    // [3, 3, 5, 5, 7].  numbers[i] = numbers [i - 1];
    // difference/ moves = 2 -> 3 and 1 -> 5. = 5
    moves
}
// ALGORITHM 2 (dynamic array) -----------------------------------------------------------------------------------------------
//dynamic array version, builds a brand new array instead of changing the old one
pub fn increasing_array_v2(numbers: &[i64]) -> i64 {
    let n = numbers.len();
    let mut adjusted: Vec<i64> = Vec::new();
    // Vec::new() - starts empty, grows bigger by itself as we add to it (a dynamic array)
    let mut moves: i64 = 0;
    // moves - same as algorithm 1, holds the total
    for i in 0..n {
        if i == 0 {
            adjusted.push(numbers[i]);
            // first number never needs lifting, nothing comes before it
        } else {
            let prev = *adjusted.last().unwrap();
            // .last() - grabs the last number we already added to adjusted
            if numbers[i] < prev {
                moves += prev - numbers[i];
                adjusted.push(prev);
                // push the lifted-up number instead of the original one
            } else {
                adjusted.push(numbers[i]);
                // already big enough, push it as is
            }
        }
    }
    // [3, 2, 5, 1, 7] stays untouched, adjusted grows into [3, 3, 5, 5, 7] on its own
    moves
}
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // let and mut because input can change and grow anytime. Whatever we
    //put in the output would be read as a string
    // io - standard library line. read_line - reads line from input, and there's no error.
    //& take a reference of the mutable input. take whatever you type, store it, if there's no error, continue
    let n: usize = input.trim().parse().unwrap();
    // let - because it can change. usize - unsigned integer datatype. creating a variable n that
    // is an unsigned integer. n holds [3 2 5 1 7]
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    //clear - ad=fter putting everything in the terminal, you can start afresh
    // io - standard library line, after every input because io reads what's in the input
    let numbers: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    // to create an array(input) of an unkwon size, we need vectors. i64 is big enough
    // split each number based on space (eg. 4 4 5)
    // map every element to a number. .parse = turns string to a number
    // . unwrap - no error
    // .collect - collect every elemnet into the array
    let mut numbers_for_v1 = numbers.clone();
    // .clone() - makes a copy, since algorithm 1 changes the array and we don't want that messing with algorithm 2
    let moves = increasing_array_v1(&mut numbers_for_v1);
    let moves_v2 = increasing_array_v2(&numbers);
    let _ = moves_v2;
    // moves_v2 should always match moves, both solve the same problem
    println!("{}", moves);
    //println!("{:?}", numbers_for_v1);
    // prints the moves as output
    //prints new array
    // judge only wants the number above, so the benchmark below only runs if we ask for it
    if std::env::var("RUN_BENCHMARK").is_err() {
        return;
    }
    
    //BENCHMARK -----------------------------------------------------------------------------------------------
    let bench_size = 200_000usize.max(n);
    // make a big array so the timer actually has something to measure
    let mut bench_data: Vec<i64> = Vec::with_capacity(bench_size);
    let mut seed: i64 = 12345;
    for _ in 0..bench_size {
        seed = (seed * 1103515245 + 12345) % 1_000_000_007;
        // simple formula that spits out a new fake-random number each time
        bench_data.push(seed % 1000);
    }
    let runs = 50;
    // repeat a bunch of times so the timing isn't just luck
    let start_v1 = Instant::now();
    for _ in 0..runs {
        let mut data_copy = bench_data.clone();
        // fresh copy every run since v1 changes the array
        increasing_array_v1(&mut data_copy);
    }
    let duration_v1 = start_v1.elapsed();
    let start_v2 = Instant::now();
    for _ in 0..runs {
        increasing_array_v2(&bench_data);
        // no copy needed, v2 never changes the original array
    }
    let duration_v2 = start_v2.elapsed();
    println!("--- Benchmark ({} runs on {} numbers) ---", runs, bench_size);
    println!("Algorithm 1 (in-place mutation): {:?}", duration_v1);
    println!("Algorithm 2 (dynamic array):     {:?}", duration_v2);
    


// BENCHMARK INTERPRETATION-----------------------------------------------------------------------------------------------------------------
//Both versions solve the problem the same way — one pass through the list with the same comparisons, so the core work is identical. 
//The speed difference is only about memory.

//The first algorithm (in-place mutation) changes the list directly, which is efficient, 
//but in the benchmark it must copy the whole list before each run. 
//That repeated copying takes a lot of time.

//The second algorithm (dynamic array) leaves the original alone and builds a new list instead. 
//It needs no copy, but the new list occasionally runs out of room and has to move everything into a bigger space, which costs a little time too.

//So each version has its own trade-off.— the first for copying, the second for growing. 
//Algorithm 1 includes a full copy of the list inside the timed section, so that copying time is counted as part of its speed. 
//Algorithm 2 doesn’t need to copy anything, so only its actual work is measured.
//Because of that, Algorithm 1 looks slower in the benchmark, but a lot of that time comes from copying the data, not from the algorithm itself. 
//If you remove the copying and compare only the real operations, Algorithm 1 is actually faster.



}
