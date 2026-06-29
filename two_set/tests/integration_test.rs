// tests/integration_test.rs
//
// Tests for both Two Sets algorithms (greedy + two-pointer pairing).
// Each test runs both solve_greedy and solve_pairing and checks:
//   1. The feasibility answer (Some vs None) is correct.
//   2. When a solution exists, verify() confirms it is valid.

use two_sets::{solve_greedy, solve_pairing, verify};

// ----------------------------------------------------------------
// Helper: run both algorithms and assert both agree + are correct
// ----------------------------------------------------------------
fn check_both(n: usize, expect_solution: bool) {
    let greedy = solve_greedy(n);
    let pairing = solve_pairing(n);

    assert_eq!(
        greedy.is_some(), expect_solution,
        "Algorithm 1 (greedy) gave wrong feasibility for n={n}"
    );
    assert_eq!(
        pairing.is_some(), expect_solution,
        "Algorithm 2 (pairing) gave wrong feasibility for n={n}"
    );

    if expect_solution {
        let (s1, s2) = greedy.unwrap();
        assert!(
            verify(n, &s1, &s2),
            "Algorithm 1 (greedy) produced invalid sets for n={n}"
        );

        let (s1, s2) = pairing.unwrap();
        assert!(
            verify(n, &s1, &s2),
            "Algorithm 2 (pairing) produced invalid sets for n={n}"
        );
    }
}

// ----------------------------------------------------------------
// Boundary / small cases
// ----------------------------------------------------------------

#[test]
fn test_n1_no_solution() {
    // total_sum = 1, odd -> NO
    check_both(1, false);
}

#[test]
fn test_n2_no_solution() {
    // total_sum = 3, odd -> NO
    check_both(2, false);
}

#[test]
fn test_n3_yes_solution() {
    check_both(3, true);
}

#[test]
fn test_n4_yes_solution() {
    check_both(4, true);
}

#[test]
fn test_n5_no_solution() {
    check_both(5, false);
}

#[test]
fn test_n6_no_solution() {
    // total_sum = 21, odd -> NO (matches example 2 from the problem statement)
    check_both(6, false);
}

#[test]
fn test_n7_yes_solution() {
    // total_sum = 28, half = 14 -> YES (matches example 1 from the problem statement)
    check_both(7, true);
}

#[test]
fn test_n8_yes_solution() {
    check_both(8, true);
}

// ----------------------------------------------------------------
// Pattern check: n%4==0 or n%4==3 -> YES, else NO
// ----------------------------------------------------------------

#[test]
fn test_feasibility_pattern_up_to_50() {
    for n in 1..=50usize {
        let total_sum = n * (n + 1) / 2;
        let expect = total_sum % 2 == 0;
        check_both(n, expect);
    }
}

// ----------------------------------------------------------------
// Explicit sum verification for the example inputs
// ----------------------------------------------------------------

#[test]
fn test_example1_n7_correct_sums() {
    let n = 7usize;
    let half = 14usize;

    let (s1, s2) = solve_greedy(n).expect("n=7 should have a solution");
    assert_eq!(s1.iter().sum::<usize>(), half, "greedy set1 sum wrong for n=7");
    assert_eq!(s2.iter().sum::<usize>(), half, "greedy set2 sum wrong for n=7");

    let (s1, s2) = solve_pairing(n).expect("n=7 should have a solution");
    assert_eq!(s1.iter().sum::<usize>(), half, "pairing set1 sum wrong for n=7");
    assert_eq!(s2.iter().sum::<usize>(), half, "pairing set2 sum wrong for n=7");
}

#[test]
fn test_example2_n6_no_solution() {
    assert!(solve_greedy(6).is_none(), "greedy: n=6 should return None");
    assert!(solve_pairing(6).is_none(), "pairing: n=6 should return None");
}

// ----------------------------------------------------------------
// All numbers 1..=n appear exactly once in the union
// ----------------------------------------------------------------

#[test]
fn test_no_duplicates_no_missing_n12() {
    let n = 12usize;
    let (s1, s2) = solve_greedy(n).unwrap();
    assert!(verify(n, &s1, &s2));

    let (s1, s2) = solve_pairing(n).unwrap();
    assert!(verify(n, &s1, &s2));
}

#[test]
fn test_no_duplicates_no_missing_n100() {
    let n = 100usize;
    let (s1, s2) = solve_greedy(n).unwrap();
    assert!(verify(n, &s1, &s2));

    let (s1, s2) = solve_pairing(n).unwrap();
    assert!(verify(n, &s1, &s2));
}

// ----------------------------------------------------------------
// Large inputs (stress / performance sanity)
// ----------------------------------------------------------------

#[test]
fn test_large_n_1000() {
    let n = 1000usize; // 1000 % 4 == 0 -> YES
    check_both(n, true);
}

#[test]
fn test_large_n_999() {
    let n = 999usize; // 999 % 4 == 3 -> YES
    check_both(n, true);
}

#[test]
fn test_large_n_998() {
    let n = 998usize; // 998 % 4 == 2 -> NO
    check_both(n, false);
}

#[test]
fn test_large_n_997() {
    let n = 997usize; // 997 % 4 == 1 -> NO
    check_both(n, false);
}

#[test]
fn test_upper_bound_n_1_000_000() {
    let n = 1_000_000usize;
    let (s1, s2) = solve_greedy(n).expect("n=1_000_000 should be solvable");
    assert!(verify(n, &s1, &s2), "greedy invalid for n=1_000_000");

    let (s1, s2) = solve_pairing(n).expect("n=1_000_000 should be solvable");
    assert!(verify(n, &s1, &s2), "pairing invalid for n=1_000_000");
}

// ----------------------------------------------------------------
// Edge: n = 0 (below constraint, but shouldn't panic)
// ----------------------------------------------------------------

#[test]
fn test_n0_edge() {
    let _ = solve_greedy(0);
    let _ = solve_pairing(0);
}
