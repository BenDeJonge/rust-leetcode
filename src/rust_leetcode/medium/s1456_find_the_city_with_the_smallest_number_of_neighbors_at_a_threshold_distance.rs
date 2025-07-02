//! https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/
//! Medium - [dynamic-programming, graph, shortest-path]
//!
//! There are n cities numbered from `0` to `n-1`. Given:
//! - the array edges where `edges[i] = [fromi, toi, weighti]` represents a
//!   bidirectional and weighted edge between cities `fromi` and `toi`;
//! - the integer `distanceThreshold`.
//!
//! Return the city with the smallest number of cities that are reachable
//! through some path and whose distance is at most `distanceThreshold`.
//! If there are multiple such cities, return the city with the greatest number.
//! Notice that the distance of a path connecting cities `i` and `j` is equal to
//! the sum of the edges' weights along that path.
//!
//! Example 1:
//! Input:
//! - `n = 4`
//! - `edges = [[0,1,3],[1,2,1],[1,3,4],[2,3,1]]`
//! - `distanceThreshold = 4`
//!
//! Output: `3`
//! Explanation: The figure above describes the graph.
//! The neighboring cities at a `distanceThreshold = 4` for each city are:
//! City 0 -> [City 1, City 2]
//! City 1 -> [City 0, City 2, City 3]
//! City 2 -> [City 0, City 1, City 3]
//! City 3 -> [City 1, City 2]
//! Cities 0 and 3 have 2 neighboring cities at a `distanceThreshold = 4`,
//! but we have to return city 3 since it has the greatest number.
//!
//! Example 2:
//! Input:
//! - `n = 5`
//! - `edges = [[0,1,2],[0,4,8],[1,2,3],[1,4,2],[2,3,1],[3,4,1]]`
//! - `distanceThreshold = 2`
//!
//! Output: `0`
//! Explanation: The figure above describes the graph.
//! The neighboring cities at a distanceThreshold = 2 for each city are:
//! City 0 -> [City 1]
//! City 1 -> [City 0, City 4]
//! City 2 -> [City 3, City 4]
//! City 3 -> [City 2, City 4]
//! City 4 -> [City 1, City 2, City 3]
//! The city 0 has 1 neighboring city at a distanceThreshold = 2.
//!
//! Constraints:
//! - `2 <= n <= 100`
//! - `1 <= edges.length <= n * (n - 1) / 2`
//! - `edges[i].length == 3`
//! - `0 <= fromi < toi < n`
//! - `1 <= weighti,distanceThreshold <= 10^4`
//! - All pairs `(fromi, toi)` are distinct.

use std::collections::BinaryHeap;

#[derive(Clone)]
struct Edge<N, C> {
    node: N,
    cost: C,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct MinHeapState<N, C> {
    node: N,
    cost: C,
}

// Explicitly implement Ord to construct a min-heap instead of a max-heap.
// States are sorted by minimal total_distance. In case of a tie, the position
// is compared as well.
impl<N: Ord, C: Ord> Ord for MinHeapState<N, C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl<N: Ord, C: Ord> PartialOrd for MinHeapState<N, C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Solution {}

impl Solution {
    fn construct_adjacency_list(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<Edge<usize, usize>>> {
        let mut new_edges = vec![vec![]; n as usize];
        for edge in edges {
            if let [city, destination, distance] = edge[..] {
                let [ci, de, di] = [city as usize, destination as usize, distance as usize];
                new_edges[ci].push(Edge { node: de, cost: di });
                new_edges[de].push(Edge { node: ci, cost: di });
            } else {
                unimplemented!()
            }
        }
        new_edges
    }

    /// Compute the shortest path from `start` to all vertices in `graph` in
    /// `O(E log(V))` with:
    /// - `E` the number of edges in `graph`. Linear because we have to run on
    ///   each edge.
    /// - `V` the number of vertices in `graph`. Logarithmic because of the
    ///   insertion in the binary heap.
    fn dijkstra(start: usize, graph: &[Vec<Edge<usize, usize>>]) -> Vec<usize> {
        let mut costs = vec![usize::MAX; graph.len()];
        let mut min_heap = BinaryHeap::new();

        // Reaching the start is free.
        costs[start] = 0;
        min_heap.push(MinHeapState {
            node: start,
            cost: 0,
        });

        // Get the closest state.
        while let Some(MinHeapState { node, cost }) = min_heap.pop() {
            // There exists a shorter way to this node.
            if cost > costs[node] {
                continue;
            }
            // Compute the distance between the start and all neighbors of the
            // current note, assuming we go through the current node.
            for edge in &graph[node] {
                let next = MinHeapState {
                    node: edge.node,
                    cost: cost + edge.cost,
                };
                if next.cost < costs[next.node] {
                    min_heap.push(next);
                    costs[next.node] = next.cost;
                }
            }
        }
        costs
    }

    /// Find the city with the least neighbors within the threshold.
    /// If multiple cities have the same minimum number of neighbors,
    /// return the city with the highest index.
    ///
    /// Complexity:
    /// - time: `O(V E log(V))` because we have to run Dijkstra on each vertex.
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let graph = Self::construct_adjacency_list(n, edges);
        let max_dist = distance_threshold as usize;
        // Compute the costs for each starting node.
        (0..(n as usize))
            .map(|start| {
                Self::dijkstra(start, &graph)
                    .iter()
                    .filter(|dist| **dist <= max_dist)
                    .count()
            })
            .enumerate()
            // Min neighbors, max index.
            .min_by(|(i_a, n_a), (i_b, n_b)| n_a.cmp(n_b).then_with(|| i_b.cmp(i_a)))
            .unwrap()
            // (index, neighbors)
            .0 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1456() {
        assert_eq!(
            Solution::find_the_city(
                4,
                vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]],
                4
            ),
            3
        );
        assert_eq!(
            Solution::find_the_city(
                5,
                vec![
                    vec![0, 1, 2],
                    vec![0, 4, 8],
                    vec![1, 2, 3],
                    vec![1, 4, 2],
                    vec![2, 3, 1],
                    vec![3, 4, 1]
                ],
                2
            ),
            0
        );
    }
}
