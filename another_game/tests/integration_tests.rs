// integration tests - pull the two functions in from the library twin
use another_game::{solve_parity, solve_heap};

#[test]
fn example_cases() {
    // the three cases straight from the problem statement
    assert_eq!(solve_parity(&[1, 2, 3]), "first");
    assert_eq!(solve_parity(&[2, 2]), "second");
    assert_eq!(solve_parity(&[5, 5, 4, 5]), "first");
    // and the heap version must agree
    assert_eq!(solve_heap(&[1, 2, 3]), "first");
    assert_eq!(solve_heap(&[2, 2]), "second");
    assert_eq!(solve_heap(&[5, 5, 4, 5]), "first");
}

#[test]
fn single_heap() {
    // one odd heap -> first, one even heap -> second
    assert_eq!(solve_parity(&[1]), "first");
    assert_eq!(solve_parity(&[2]), "second");
    assert_eq!(solve_heap(&[1]), "first");
    assert_eq!(solve_heap(&[2]), "second");
}

#[test]
fn all_even_is_second() {
    // every heap even -> second wins, no matter how many
    let v = [2u64, 4, 6, 8, 100, 1000];
    assert_eq!(solve_parity(&v), "second");
    assert_eq!(solve_heap(&v), "second");
}

#[test]
fn one_odd_among_evens_is_first() {
    // a single odd hiding in a sea of evens still flips it to first
    let v = [2u64, 4, 6, 7, 8, 10];
    assert_eq!(solve_parity(&v), "first");
    assert_eq!(solve_heap(&v), "first");
}

#[test]
fn big_values() {
    // largest allowed value: 1e9 is even -> second
    assert_eq!(solve_parity(&[1_000_000_000]), "second");
    assert_eq!(solve_heap(&[1_000_000_000]), "second");
    // 999_999_999 is odd -> first
    assert_eq!(solve_parity(&[999_999_999]), "first");
    assert_eq!(solve_heap(&[999_999_999]), "first");
}

#[test]
fn algorithms_always_agree() {
    // hammer both algos on a thousand random inputs - they must never disagree
    let mut seed: u64 = 12345;
    for _ in 0..1000 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let n = (seed % 50) as usize + 1; // 1..=50 heaps
        let mut v: Vec<u64> = Vec::with_capacity(n);
        for _ in 0..n {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            v.push(seed % 1_000_000_000 + 1); // 1..=1e9
        }
        assert_eq!(solve_parity(&v), solve_heap(&v));
    }
}
