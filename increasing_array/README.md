# Increasing Array: Benchmark Interpretation

Both versions solve the problem the same way — one pass through the list with the same comparisons, so the core work is identical. The speed difference is only about memory.

The **first algorithm (in-place mutation)** changes the list directly, which is efficient, but in the benchmark it must copy the whole list before each run. That repeated copying takes a lot of time.

The **second algorithm (dynamic array)** leaves the original alone and builds a new list instead. It needs no copy, but the new list occasionally runs out of room and has to move everything into a bigger space, which costs a little time too.

So each version has its own trade-off — the first for copying, the second for growing. Algorithm 1 includes a full copy of the list inside the timed section, so that copying time is counted as part of its speed. Algorithm 2 doesn't need to copy anything, so only its actual work is measured. Because of that, Algorithm 1 looks slower in the benchmark, but a lot of that time comes from copying the data, not from the algorithm itself. If you remove the copying and compare only the real operations, Algorithm 1 is actually faster.
