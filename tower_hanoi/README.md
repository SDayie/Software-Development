<<<<<<< HEAD
# Tower of Hanoi: Benchmark Interpretation

Both versions solve the problem the same way—they generate the same (2^n - 1) moves and record each one into a list, so the core work is identical. The speed difference comes from how each version keeps track of the disks.

Algorithm 1 (recursive) lets the call stack handle the bookkeeping - so it mainly records each move without manually moving disks between stacks. Its benchmark measures the whole algorithm, but most of the bookkeeping is handled naturally by recursion, so it performs relatively little extra work (additional operations).

Algorithm 2 (iterative) manages three real stacks itself. Every step involves pushing and popping disks, and on half the steps it also checks the top disk on each stack, compares disk sizes, and creates a temporary list of the two possible (candidate) stacks before deciding the legal move. That extra work is included in the benchmark.

So each version has its own trade-off—Algorithm 1 (recursive) relies on recursion to keep the bookkeeping simple, while Algorithm 2 (iterative) manages everything explicitly and does more work per move. Both produce the same moves in the same number of steps, but for this implementation and benchmark size, Algorithm 1 (recursive) is faster because it performs less bookkeeping (work) per move.
=======
# Software-Development
Software Development exam portfolio — CSES problems solved in Rust
>>>>>>> d67aad7608b1fc9e39e2320e5c221bef7037a4a2
