# Counting Rooms: Benchmark Interpretation

Both versions solve the same problem — counting connected rooms in a grid — so the core result is identical. The difference is in how they process and organise the computation, which affects performance.

The **1st algorithm (flood fill)** uses a stack to go through each room and marks visited cells as it goes. Each cell is processed once, and no extra data structures are needed after the search finishes.

The **2nd algorithm (union-find)** first goes through the grid and connects neighboring floor cells, then scans the grid again to find each group's representative and stores it in a HashSet to count rooms. This means it does extra work after the merging step, including repeated find operations and hashing.

So the main difference is the extra work: 1st algorithm (flood fill) is faster since it does everything in a single pass using simple marking. The 2nd algorithm (union-find) requires another full scan of the whole grid and extra steps to count components. Because of this, union-find has more constant work in this benchmark, even though both have the same asymptotic complexity.
