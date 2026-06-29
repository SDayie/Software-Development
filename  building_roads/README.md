# Building Roads: Benchmark Interpretation

Both versions solve the same problem: they find connected components ("islands") in a graph and then pick one representative city per island to chain them together. The output is identical, so any speed difference comes only from how each approach is implemented.

**Algorithm 1 (Union-Find / DSU)** works directly on the edge list. It keeps a `parent` array and a `size` array, merging groups as it reads each road. With path compression, future lookups become almost constant time. It never builds a full graph structure, so it stays lightweight and fast.

**Algorithm 2 (graph traversal with a heap)** first builds a full adjacency list, which already adds extra overhead. It then explores each island using a `BinaryHeap`, always picking the smallest next city. This heap adds `log n` cost to every push and pop, even though the ordering doesn't change the final result.

In short: both are correct and find the same islands, but DSU is faster because it avoids building an adjacency list and avoids heap operations. The difference is purely in efficiency, not correctness.
