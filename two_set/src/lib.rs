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
    // used = vec![false; n + 1] = the numbers that have been already used, so we don't keep on taking the same number again, like a checklist
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
    // just a sanity check, makes the program crash on purpose if the math was wrong somewhere
    Some((set1, set2))
}

// ============================================================
pub fn verify(n: usize, set1: &[usize], set2: &[usize]) -> bool {
    let total_sum = n * (n + 1) / 2;
    if total_sum % 2 != 0 {
        return false;
    }
    let half = total_sum / 2;

    let sum1: usize = set1.iter().sum();
    let sum2: usize = set2.iter().sum();
    if sum1 != half || sum2 != half {
        return false;
    }

    let mut seen = vec![false; n + 1];
    for &x in set1.iter().chain(set2.iter()) {
        if x < 1 || x > n || seen[x] {
            return false;
        }
        seen[x] = true;
    }
    seen[1..=n].iter().all(|&b| b)
}
