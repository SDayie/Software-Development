
use finding_borders::{borders_hash_membership, borders_prefix_function};

// the problem's own example: borders of "abcababcab" are lengths 2 and 5
#[test]
fn example_from_statement() {
    let s = b"abcababcab";
    assert_eq!(borders_prefix_function(s), vec![2, 5]);
    assert_eq!(borders_hash_membership(s), vec![2, 5]);
}

// all same letter: every proper prefix is also a suffix -> 1, 2, ..., n-1
#[test]
fn all_same_letters() {
    let s = b"aaaaa";
    assert_eq!(borders_prefix_function(s), vec![1, 2, 3, 4]);
    assert_eq!(borders_hash_membership(s), vec![1, 2, 3, 4]);
}

// no border at all
#[test]
fn no_borders() {
    let s = b"abcde";
    assert_eq!(borders_prefix_function(s), Vec::<usize>::new());
    assert_eq!(borders_hash_membership(s), Vec::<usize>::new());
}

// single character: a border can't be the whole string, so none
#[test]
fn single_char() {
    let s = b"a";
    assert_eq!(borders_prefix_function(s), Vec::<usize>::new());
    assert_eq!(borders_hash_membership(s), Vec::<usize>::new());
}

// small handmade case: borders of "aabaa" are lengths 1 and 2
#[test]
fn aabaa_case() {
    let s = b"aabaa";
    assert_eq!(borders_prefix_function(s), vec![1, 2]);
    assert_eq!(borders_hash_membership(s), vec![1, 2]);
}

// brute-force reference: a length L is a border iff prefix L == suffix L
fn brute(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut res = Vec::new();
    for len in 1..n {
        if s[..len] == s[n - len..] {
            res.push(len);
        }
    }
    res
}


#[test]
fn fuzz_against_brute_force() {
    let mut state: u64 = 0x1234_5678_9abc_def0;
    let mut next = || {
        // xorshift - cheap and good enough for test randomness
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };

    for _ in 0..400 {
        let len = (next() % 40) as usize + 1; // strings of length 1..=40
        let alphabet = (next() % 3) as u8 + 2; // 2..=4 distinct letters (more collisions = harder)
        let s: Vec<u8> = (0..len)
            .map(|_| b'a' + (next() % alphabet as u64) as u8)
            .collect();

        let expected = brute(&s);
        assert_eq!(borders_prefix_function(&s), expected, "prefix fn failed on {:?}", s);
        assert_eq!(borders_hash_membership(&s), expected, "hash fn failed on {:?}", s);
    }
}
