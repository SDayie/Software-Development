# Finding Borders: Benchmark Interpretation

Both versions reach the same answer, and both make a single pass over the string — and both have the same theoretical time complexity (O(n)). But unlike cases where two methods do identical work and only differ in memory, they do a different amount of work per letter, so the gap in the benchmark is a real performance difference.

The first algorithm (prefix function) does very little at each step: compare two letters and write one number into a single small array it reads straight through. The array is read and written in order, so memory access stays fast (efficient) and predictable.

The second algorithm (hash + HashSet) does more for the same letter: two hash calculations, extra array work for prefix/power tables, and then many HashSet operations — each one involving hashing and jumping through a hash table in memory. That requires more work from the processor and slower memory access.

The benchmark itself is fair: neither algorithm copies the input first, so neither algorithm receives an unfair timing advantage or disadvantage. That means the prefix function isn't just looking faster — it's genuinely faster because it does less work and uses memory more efficiently. Both are O(n), but one performs much more work per character.
