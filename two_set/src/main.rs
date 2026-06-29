use std::io;
use std::time::Instant;
// Instant=a stopwatch, we use it later to time how fast each algorithm runs
// ============================================================
// ALGORITHM 1: GREEDY (largest number first)
// ============================================================
pub fn solve_greedy(n: usize) -> Option<(Vec<usize>, Vec<usize>)> {
let total_sum = n * (n + 1) / 2;
if total_sum % 2 != 0 {
return None;
    }
//%2!=0( not divisible by 2)
//%2==0(divible by 2)
let mut target = total_sum / 2;
let mut set1: Vec<usize> = Vec::new();
let mut used = vec![false; n + 1];
//vec an array that can change and grow
//set1=where you pick the biggest numbers from the univeral set
// used = vec![false; n + 1] = the numbers that have been already used, so we don't keep on taking the same number again( it acts like a checklist)
for number in (1..=n).rev() {
if number <= target {
set1.push(number);
used[number] = true;
target -= number;
        }
    }
// for number in (1..=n).rev() = it reverses the list, so that it includes n(7,6,5,4,3,2,1)
let mut set2: Vec<usize> = Vec::new();
for number in 1..=n {
if !used[number] {
set2.push(number);
        }
    }
// if a number is not used, push it to the new set, and the new set here is set 2
Some((set1, set2))
// len=lenght(how many elements are in the set):
//4
//1 2 4 7
//3
//3 5 6
// for index in set1/set2 print the individual value. number=index
}

// ============================================================
// ALGORITHM 2: TWO-POINTER PAIRING
// ============================================================
pub fn solve_pairing(n: usize) -> Option<(Vec<usize>, Vec<usize>)> {
let total_sum = n * (n + 1) / 2;
if total_sum % 2 != 0 {
return None;
    }
//%2!=0( not divisible by 2)
//%2==0(divible by 2)
let target = total_sum / 2;
// target=half the total, this time we don't shrink it, we just check it at the end
let mut set1: Vec<usize> = Vec::new();
let mut used = vec![false; n + 1];
//vec an array that can change and grow
// used = vec![false; n + 1] = the numbers that have been already used, same checklist as before
let m = n / 4;
// m=how many pairs we need. every pair of (small number, big number) adds up to the same thing so we just count pairs instead of picking one at a time
for number in 1..=m {
let high = n - number + 1;
// high=the partner for number, so number=1 pairs with high=n, number=2 pairs with high=n-1, and so on
set1.push(number);
set1.push(high);
used[number] = true;
used[high] = true;
    }
// for number in 1..=m = grabs m pairs (1,n) (2,n-1)... and pushes both halves into set1
if n % 4 == 3 {
let extra = n - m;
// sometimes the pairs aren't enough and we need exactly one more number to close the gap, extra is that number
set1.push(extra);
used[extra] = true;
    }
let mut set2: Vec<usize> = Vec::new();
for number in 1..=n {
if !used[number] {
set2.push(number);
        }
    }
// if a number is not used, push it to the new set, and the new set here is set 2
debug_assert_eq!(set1.iter().sum::<usize>(), target, "set1 didn't reach target");
// just a check, makes the program crash on purpose if the math was wrong somewhere
Some((set1, set2))
}
fn print_answer(result: Option<(Vec<usize>, Vec<usize>)>) {
// this is the ONLY thing allowed to reach stdout, since the judge expects exactly
// one YES/NO answer in this exact shape, nothing else mixed in
match result {
None => println!("NO"),
Some((set1, set2)) => {
println!("YES");
println!("{}", set1.len());
for number in &set1 {
print!("{} ", number);
            }
println!();
println!("{}", set2.len());
for number in &set2 {
print!("{} ", number);
            }
println!();
        }
    }
}
fn main() {
let mut input = String::new();
// input=an empty piece of text that we'll fill with whatever the user types
io::stdin().read_line(&mut input).unwrap();
// read_line waits for the user to type something and press enter, then stores it in input
let n: usize = input.trim().parse().unwrap();
// .trim() removes the newline from pressing enter, .parse() turns the text "7" into the number 7
// the judge only wants ONE answer on stdout, so we just print algorithm 1's result here
print_answer(solve_greedy(n));

debug_assert_eq!(
solve_greedy(n).is_some(),
solve_pairing(n).is_some(),
"the two algorithms disagree on whether a split exists"
    );

    if std::env::var("RUN_BENCHMARK").is_err() {
return;
    }
// ============================================================
// BENCHMARK
// ============================================================
eprintln!("\n=== Benchmark (n={n}, 1000 runs each) ===");
// eprintln prints to stderr instead of stdout, so the benchmark numbers don't mix in with the real YES/NO answer
let _ = solve_greedy(n);
// run it once first as a warmup so the first run doesn't mess up the timing
let start = Instant::now();
// start the stopwatch right now
for _ in 0..1000 {
// run it 1000 times in a row so tiny speed differences add up and are easier to see
let _ = solve_greedy(n);
// the underscore means we run it but don't care about the actual answer, just the time it takes
    }
let greedy_elapsed = start.elapsed();
// .elapsed()=stops the stopwatch, gives us the total time for all 1000 runs
let _ = solve_pairing(n);
// warmup run for algorithm 2 too, so it's a fair comparison
let start = Instant::now();
// restart the stopwatch for algorithm 2
for _ in 0..1000 {
let _ = solve_pairing(n);
    }
let pairing_elapsed = start.elapsed();
// stop the stopwatch, this is algorithm 2's total time
eprintln!(
"  [Greedy]  total={:?} | avg={:.2}µs",
greedy_elapsed,
greedy_elapsed.as_micros() as f64 / 1000.0
    );
// {:?} prints the raw time, as_micros() turns it into microseconds so we can divide by 1000 and get the average per run
eprintln!(
"  [Pairing] total={:?} | avg={:.2}µs",
pairing_elapsed,
pairing_elapsed.as_micros() as f64 / 1000.0
    );
eprintln!("========================================");
// a line to make the benchmark output easier to read


// BENCHMARK INTERPRETATION-----------------------------------------------------------------------------------------------------------------

//Both versions get to the same answer, 
//and both build their sets by pushing numbers into new lists, 
//so they both use a similar approach.

//The first one (greedy) looks at every number from n down to 1 and asks, "Does this fit?" before deciding. 
//That means it goes through all n numbers to build set1, 
//so it spends more time checking numbers before adding them.

//The second one (Two-pointer pairing) skips all that asking. 
//It already knows 1 goes with n, 2 goes with n−1, 
//so it just adds those pairs straight into set1. 
//It uses fewer loops to build the first set because it already knows which numbers belong together.

//Both algorithms still go through all the numbers to build set2, 
//so that part takes the same amount of work. 
//The greedy version is slower because it checks every number before deciding whether to use it, 
//while the pairing version is faster because it already knows which numbers to pick. 
//This doesn't mean the greedy method is worse —it just does more checking to reach the same result.


}
