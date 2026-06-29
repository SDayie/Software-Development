# Distinct Numbers: Benchmark Interpretation

Both versions solve the problem the same way — one pass through the list, putting each number into a structure that removes duplicates, then counting what remains. The difference is how they check "have I seen this before?"

## Algorithm 1 (HashSet)

Hashes each number and uses that value to jump to a bucket in a table. On average this is very fast because it does not depend on how many elements are already stored. There is some additional work in hashing and collision resolution, but it is still close to O(1) per insert. It does not keep any order, so values are stored in no sequence.

## Algorithm 2 (AVL tree)

Uses comparisons instead of hashing. Each number goes left or right depending on whether it is smaller or larger. The tree stays balanced using rotations, so its height stays about O(log n). This means the search path does not grow over time. It stays sorted because of the tree structure, but each insert does extra work to maintain balance.

## Result

In the benchmark, Algorithm 2 (AVL) is slower because of comparisons and rotations. Algorithm 1 (HashSet) is faster because it only needs uniqueness, not ordering. If we needed sorted values, the AVL tree would help — but here that extra work is unnecessary.
