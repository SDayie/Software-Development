# Another Game: Benchmark Interpretation

Both versions scan through the heaps and check whether each one is odd or even, so the main work of reading and testing every value is the same.

**Algorithm 1 (parity scan)** uses a single boolean flag. It goes through the list and stops immediately when it finds the first odd heap, because that alone is enough to determine the answer. It uses constant memory and avoids any extra processing beyond the scan itself.

**Algorithm 2 (heap)** also checks every heap, but instead of just remembering "seen odd or not," it pushes every result into a `BinaryHeap`. That adds extra work because each insert causes the heap to reorder itself, and it also uses more memory. Unlike Algorithm 1, it cannot stop early and must process all values before it can return an answer.

So both do the same basic scan, but Algorithm 2 adds unnecessary heap overhead on top. With large inputs, that extra reordering and memory work becomes noticeable, even though the actual problem being solved is identical. The heap structure is powerful in general, but here it is overkill for a problem that only needs a single bit of information.
