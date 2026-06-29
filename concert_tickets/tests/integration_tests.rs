// integration tests. pulls the two algorithms out of the library crate and
// checks them on hand-picked cases plus a pile of random ones against a
// dead-simple brute force reference.

use concert_tickets::{solve_btree, solve_segment_tree};

// brute force - obviously correct, but slow (O(n*m)). only used to grade the fast ones.
// for each customer, scan every leftover ticket and take the most expensive one that fits.
fn brute(prices: &[i64], queries: &[i64]) -> Vec<i64> {
    let mut avail = prices.to_vec();
    let mut out = Vec::with_capacity(queries.len());
    for &t in queries {
        let mut best: Option<usize> = None;
        for i in 0..avail.len() {
            if avail[i] <= t && (best.is_none() || avail[i] > avail[best.unwrap()]) {
                best = Some(i);
            }
        }
        match best {
            Some(i) => {
                out.push(avail[i]);
                avail.swap_remove(i); // pull that ticket off the shelf.
            }
            None => out.push(-1),
        }
    }
    out
}

#[test]
fn example_from_statement() {
    let prices = [5, 3, 7, 8, 5];
    let queries = [4, 8, 3];
    let expected = vec![3, 8, -1];
    assert_eq!(solve_segment_tree(&prices, &queries), expected);
    assert_eq!(solve_btree(&prices, &queries), expected);
}

#[test]
fn single_ticket_taken() {
    let prices = [10];
    let queries = [10];
    assert_eq!(solve_segment_tree(&prices, &queries), vec![10]);
    assert_eq!(solve_btree(&prices, &queries), vec![10]);
}

#[test]
fn single_ticket_too_pricey() {
    let prices = [10];
    let queries = [9];
    assert_eq!(solve_segment_tree(&prices, &queries), vec![-1]);
    assert_eq!(solve_btree(&prices, &queries), vec![-1]);
}

#[test]
fn everyone_misses() {
    let prices = [100, 200, 300];
    let queries = [1, 1, 1];
    let expected = vec![-1, -1, -1];
    assert_eq!(solve_segment_tree(&prices, &queries), expected);
    assert_eq!(solve_btree(&prices, &queries), expected);
}

#[test]
fn duplicates_are_consumed_one_at_a_time() {
    // three identical tickets, three identical budgets -> all three get served, fourth misses.
    let prices = [5, 5, 5];
    let queries = [5, 5, 5, 5];
    let expected = vec![5, 5, 5, -1];
    assert_eq!(solve_segment_tree(&prices, &queries), expected);
    assert_eq!(solve_btree(&prices, &queries), expected);
}

#[test]
fn more_customers_than_tickets() {
    let prices = [2, 4];
    let queries = [10, 10, 10];
    let expected = vec![4, 2, -1];
    assert_eq!(solve_segment_tree(&prices, &queries), expected);
    assert_eq!(solve_btree(&prices, &queries), expected);
}

#[test]
fn picks_nearest_below_budget_not_the_cheapest() {
    // budget 6 should grab the 5, not the 2.
    let prices = [2, 5, 9];
    let queries = [6];
    assert_eq!(solve_segment_tree(&prices, &queries), vec![5]);
    assert_eq!(solve_btree(&prices, &queries), vec![5]);
}

#[test]
fn both_algorithms_agree_with_brute_force_random() {
    // tiny home-grown random generator (an LCG) so we need no external crate.
    let mut state: u64 = 0x1234_5678_9abc_def0;
    let mut next = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (state >> 33) as i64
    };

    for _ in 0..400 {
        let n = (next().unsigned_abs() % 40 + 1) as usize; // 1..=40 tickets.
        let m = (next().unsigned_abs() % 40 + 1) as usize; // 1..=40 customers.
        let prices: Vec<i64> = (0..n).map(|_| next().unsigned_abs() as i64 % 25 + 1).collect();
        let queries: Vec<i64> = (0..m).map(|_| next().unsigned_abs() as i64 % 25 + 1).collect();

        let reference = brute(&prices, &queries);
        assert_eq!(
            solve_segment_tree(&prices, &queries),
            reference,
            "segment tree disagreed on prices={:?} queries={:?}",
            prices, queries
        );
        assert_eq!(
            solve_btree(&prices, &queries),
            reference,
            "b-tree disagreed on prices={:?} queries={:?}",
            prices, queries
        );
    }
}
