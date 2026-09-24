//! <https://leetcode.com/problems/course-schedule/>
//! Medium - [depth-first-search, breadth-first-search, graph, topological-sort, directed-acyclic-graph]
//!
//! There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1.
//! You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.
//! For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
//! Return true if you can finish all courses. Otherwise, return false.
//!
//! Example 1:
//! Input: numCourses = 2, prerequisites = [[1,0]]
//! Output: true
//! Explanation: There are a total of 2 courses to take.
//! To take course 1 you should have finished course 0. So it is possible.
//! Example 2:
//! Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
//! Output: false
//! Explanation: There are a total of 2 courses to take.
//! To take course 1 you should have finished course 0, and to take course 0 you should also have finished course 1. So it is impossible.
//!
//! Constraints:
//! - 1 <= numCourses <= 2000
//! - 0 <= prerequisites.length <= 5000
//! - prerequisites[i].length == 2
//! - 0 <= ai, bi < numCourses
//! - All the pairs prerequisites[i] are unique.

use std::{collections::HashMap, fmt::Debug, hash::Hash};

#[derive(Clone)]
enum VisitState {
    Unvisited,
    Visiting,
    Visited,
}

type Graph<'a, T> = HashMap<&'a T, Vec<&'a T>>;
type Visited<'a, T> = HashMap<&'a T, VisitState>;

pub struct Solution {}

impl Solution {
    pub fn can_finish(_num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        Self::can_finish_usize(&prerequisites)
    }

    /// - Time: `O(E + V)`
    /// - Space: `O(V)`
    ///
    /// with `E`, `V` the number of edges and vertices, respectively.
    ///
    /// Consider a prerequisites adjacency list like:
    ///
    /// `[[1, 0], [1, 2], [2, 4], [2, 3], [3, 1], [4, 5]]`
    ///
    /// This builds the following graph:
    ///
    /// ```text
    /// ┌───────────┐
    /// v           |
    /// 3 ──> 1 ──> 2 ──> 4 ──> 5
    ///       |
    ///       └───> 0
    /// ```
    ///
    /// This graph contains a cycle and is therefore not a valid course schedule.    
    fn can_finish_usize(prerequisites: &[Vec<i32>]) -> bool {
        if prerequisites.is_empty() {
            return true;
        }

        let graph = Self::build_adjancency_list(prerequisites);
        let mut visited = Self::build_visited(&graph);

        !Self::dfs_cycle(&graph, &mut visited)
    }

    fn build_adjancency_list<T: Hash + Eq + Debug>(prerequisites: &'_ [Vec<T>]) -> Graph<'_, T> {
        let mut map = Graph::<T>::new();
        prerequisites.iter().for_each(|prereq| {
            map.entry(&prereq[0])
                .and_modify(|adjacency| adjacency.push(&prereq[1]))
                .or_insert(vec![&prereq[1]]);
        });
        map
    }

    fn build_visited<'a, T: Hash + Eq>(graph: &'a Graph<T>) -> Visited<'a, T> {
        let mut map = HashMap::new();
        graph.iter().for_each(|(&node, vertices)| {
            map.entry(node).or_insert(VisitState::Unvisited);
            vertices.iter().for_each(|vertex| {
                map.entry(vertex).or_insert(VisitState::Unvisited);
            });
        });
        map
    }

    fn dfs_cycle<'a, T: Hash + Eq>(graph: &'a Graph<'a, T>, visited: &mut Visited<'a, T>) -> bool {
        graph
            .keys()
            .any(|course| Self::dfs_helper(graph, course, visited))
    }

    fn dfs_helper<'a, T: Hash + Eq>(
        graph: &'a Graph<T>,
        edge: &'a T,
        visited: &mut Visited<'a, T>,
    ) -> bool {
        match visited[edge] {
            VisitState::Visited => return false,
            VisitState::Visiting => return true,
            VisitState::Unvisited => {}
        }

        visited.insert(edge, VisitState::Visiting);

        if let Some(prereqs) = graph.get(edge)
            && prereqs
                .iter()
                .any(|prereq| Self::dfs_helper(graph, *prereq, visited))
        {
            return true;
        }

        visited.insert(edge, VisitState::Visited);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn cycle() {
        assert!(!Solution::can_finish_usize(
            // ┌───────────┐
            // v           |
            // 3 ──> 1 ──> 2 ──> 4 ──> 5
            //       |
            //       └───> 0
            &[
                vec![1, 0],
                vec![1, 2],
                vec![2, 4],
                vec![2, 3],
                vec![3, 1],
                vec![4, 5]
            ]
        ));
    }

    #[test]
    fn no_cycle() {
        assert!(Solution::can_finish_usize(
            // ┌───────────┐
            // |           v
            // 3 ──> 1 ──> 2 ──> 4 ──> 5
            //       |
            //       └───> 0
            &[
                vec![1, 0],
                vec![1, 2],
                vec![2, 4],
                vec![3, 2],
                vec![3, 1],
                vec![4, 5]
            ]
        ));
    }

    #[test]
    fn short_no_cycle() {
        assert!(Solution::can_finish_usize(
            // 1 ──> 0
            &[vec![1, 0],]
        ));
    }

    #[test]
    fn short_cycle() {
        assert!(!Solution::can_finish_usize(
            // ┌─────┐
            // v     |
            // 1 ──> 0
            &[vec![1, 0], vec![0, 1]]
        ));
    }

    #[test]
    fn xxx() {
        assert!(Solution::can_finish_usize(
            // ┌───> 2 ────┐
            // |           v
            // 3 ──> 1 ──> 4
            &[vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]]
        ));
    }
}
