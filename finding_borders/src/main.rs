use std::io::{self, Read, Write};
// std library io: Read lets us slurp input, Write lets us print fast
use std::collections::HashSet;
// HashSet - a "bag" that answers "is this thing in here?" really fast


// =====================================================================
// ALGORITHM 1 : prefix function (KMP failure links)
// =====================================================================

fn borders_prefix_function(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    // n - how many letters are in the string
    if n == 0 {
        return Vec::new();
    }
    // empty string has no borders, leave early
    let mut pi = vec![0usize; n];
    // pi - longest border length for each prefix, starts all zeros
    for i in 1..n {
        // start at 1, a single letter on its own can't have a border
        let mut j = pi[i - 1];
        // j - try to reuse the border we had one step back
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1];
        }
        // letters don't match? fall back to the next shorter border and retry
        if s[i] == s[j] {
            j += 1;
        }
        // letters match? the border grows by one
        pi[i] = j;
        // remember the best border length ending at this spot
    }
    let mut res = Vec::new();
    // res - where we collect the answer
    let mut k = pi[n - 1];
    // k - the longest border of the full string sits at the last spot
    while k > 0 {
        res.push(k);
        // write down this border length
        k = pi[k - 1];
        // jump to the next shorter one (a border of the border)
    }
    res.reverse();
    // we collected biggest-first, flip it so it's smallest-first
    res
}


// =====================================================================
// ALGORITHM 2 : rolling hash + HashSet  (Membership Structures)
// =====================================================================

fn borders_hash_membership(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    // n - length of the string
    if n == 0 {
        return Vec::new();
    }
    // nothing to do for an empty string

    const B1: u64 = 131;
    const M1: u64 = 1_000_000_007;
    const B2: u64 = 137;
    const M2: u64 = 998_244_353;
    // B - the base we multiply by, M - a big prime we wrap around.
    // we use TWO of them so two different chunks basically never collide by luck

    let mut pref1 = vec![0u64; n + 1];
    let mut pref2 = vec![0u64; n + 1];
    // pref - running hash of the string built up from the left, one per hash
    let mut pow1 = vec![1u64; n + 1];
    let mut pow2 = vec![1u64; n + 1];
    // pow - powers of the base, needed to chop a substring out later

    for i in 0..n {
        pref1[i + 1] = (pref1[i] * B1 + s[i] as u64) % M1;
        pref2[i + 1] = (pref2[i] * B2 + s[i] as u64) % M2;
        pow1[i + 1] = (pow1[i] * B1) % M1;
        pow2[i + 1] = (pow2[i] * B2) % M2;
    }
    // one sweep builds every prefix hash and every power we'll need

    // hash of the slice s[l..r], pulled out of the prefix hashes
    let sub1 = |l: usize, r: usize| -> u64 {
        (pref1[r] + M1 - (pref1[l] * pow1[r - l]) % M1) % M1
    };
    let sub2 = |l: usize, r: usize| -> u64 {
        (pref2[r] + M2 - (pref2[l] * pow2[r - l]) % M2) % M2
    };
    // sub - subtract off the left part so we're left with just the chunk we want

    let mut prefixes: HashSet<(usize, u64, u64)> = HashSet::with_capacity(n);
    // prefixes - the membership bag, keyed by (length, hash1, hash2)
    for len in 1..n {
        prefixes.insert((len, sub1(0, len), sub2(0, len)));
    }
    // drop every proper prefix (length 1 up to n-1) into the bag

    let mut res = Vec::new();
    // res - the border lengths we discover
    for len in 1..n {
        let key = (len, sub1(n - len, n), sub2(n - len, n));
        // key - describe the suffix of this length the exact same way as a prefix
        if prefixes.contains(&key) {
            res.push(len);
        }
        // bag already holds a same-length prefix that matches? it's a border
    }
    res
    // already increasing since len marches 1, 2, 3, ...
}


// =====================================================================
// fn main : reads input, prints ONE clean answer for the judge
// =====================================================================

fn main() {
    let mut input = String::new();
    // input - empty string to dump everything we read into
    io::stdin().read_to_string(&mut input).unwrap();
    // slurp the whole input in one go
    let s = input.trim().as_bytes();
    // trim the trailing newline/spaces, then view it as raw bytes (the letters)

    let borders = borders_prefix_function(s);
    // use the fast prefix-function version for the real answer

    let out: Vec<String> = borders.iter().map(|x| x.to_string()).collect();
    // turn each number into text so we can print them on one line
    let stdout = io::stdout();
    let mut w = io::BufWriter::new(stdout.lock());
    // BufWriter - batches the output so huge prints stay fast
    writeln!(w, "{}", out.join(" ")).unwrap();
    // print all border lengths separated by spaces, then a newline

    // benchmark is GATED: only runs if we ask for it, never on the judge
    if std::env::var("RUN_BENCHMARK").is_ok() || std::env::args().any(|a| a == "--bench") {
        run_benchmark(s);
    }
    // needs the RUN_BENCHMARK env var OR the --bench flag, otherwise skipped
}


// =====================================================================
// benchmark : times both algorithms, prints to stderr only
// =====================================================================

fn run_benchmark(s: &[u8]) {
    use std::time::Instant;
    // Instant - a little stopwatch

    let reps: u32 = 5;
    // reps - run each a few times so the timing is less noisy

    let t1 = Instant::now();
    let mut r1 = Vec::new();
    for _ in 0..reps {
        r1 = borders_prefix_function(s);
    }
    let d1 = t1.elapsed();
    // time the prefix-function version

    let t2 = Instant::now();
    let mut r2 = Vec::new();
    for _ in 0..reps {
        r2 = borders_hash_membership(s);
    }
    let d2 = t2.elapsed();
    // time the hash + HashSet version

    eprintln!("n = {}", s.len());
    eprintln!("prefix function : {:?} per run -> {} borders", d1 / reps, r1.len());
    eprintln!("hash membership : {:?} per run -> {} borders", d2 / reps, r2.len());
    eprintln!("both agree?      : {}", r1 == r2);
    // EVERYTHING here goes to stderr (eprintln), so the judge's stdout stays clean
}


