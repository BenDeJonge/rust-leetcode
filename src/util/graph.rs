//     2 - 3
//   /     |
// 1       |
//   \     |
//     4 - 5
//
// An adjaceny list stores all edges of any vertex.
// This is convenient to iterate over the edges of any vertex.
//
// 1: [2, 4]
// 2: [1, 3, 4]
// 3: [2, 4, 5]
// 4: [1, 2, 3, 5]
// 5: [3, 4]
//
// An adjacency matrix stores all edges between all vertices pairs.
// This is convenient to identify edges between two vertices.
//
//      1   2   3   4   5
// 1  | 0 | 1 | 0 | 1 | 0 |
// 2  | 1 | 0 | 1 | 0 | 0 |
// 3 <  0 | 1 | 0 | 1 | 1  >
// 4  | 1 | 1 | 1 | 0 | 1 |
// 5  | 0 | 0 | 1 | 1 | 0 |

use std::{fmt::Display, ops::Range};

pub struct Vertex<'g, T> {
    pub value: &'g T,
    pub edges: &'g [usize],
}

#[derive(Debug, PartialEq)]
pub struct Graph<T> {
    vertices: Vec<T>,
    edges: Vec<Vec<usize>>,
    max_vertex: usize,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            vertices: vec![],
            edges: vec![],
            max_vertex: 0,
        }
    }

    pub fn add_vertex(&mut self, value: T) -> usize {
        self.vertices.push(value);
        self.edges.push(vec![]);
        self.vertices.len() - 1
    }

    pub fn add_vertices(&mut self, values: impl IntoIterator<Item = T>) -> Range<usize> {
        let start = self.vertices.len();
        self.vertices.extend(values);
        let stop = self.vertices.len();
        self.edges.extend(vec![vec![]; stop - start]);
        start..stop
    }

    pub fn add_edge(&mut self, origin: usize, destination: usize) {
        self.edges[origin].push(destination);
        self.max_vertex = self.max_vertex.max(destination);
    }

    pub fn add_edge_bidirectional(&mut self, origin: usize, destination: usize) {
        self.add_edge(origin, destination);
        self.add_edge(destination, origin);
    }

    pub fn is_valid(&self) -> bool {
        self.max_vertex < self.vertices.len()
    }

    pub fn get_vertex(&self, i: usize) -> Option<Vertex<'_, T>> {
        if i >= self.vertices.len() {
            None
        } else {
            Some(Vertex {
                value: &self.vertices[i],
                edges: &self.edges[i],
            })
        }
    }

    pub fn n_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn n_edges(&self) -> usize {
        // Edges are counted in both directions.
        self.edges.iter().map(|e| e.len()).sum::<usize>() / 2
    }

    pub fn iter(&self) -> GraphIterator<'_, T> {
        GraphIterator {
            graph: self,
            index: 0,
        }
    }
}

impl<T> Default for Graph<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GraphIterator<'g, T> {
    graph: &'g Graph<T>,
    index: usize,
}

impl<'g, T> Iterator for GraphIterator<'g, T> {
    type Item = Vertex<'g, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.graph.get_vertex(self.index);
        self.index += 1;
        result
    }
}

impl<T: Display> Display for Graph<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: Vec<String> = self
            .edges
            .iter()
            .enumerate()
            .map(|(i, e)| {
                format!(
                    "{}: {}",
                    self.get_vertex(i).unwrap().value,
                    e.iter()
                        .map(|int| self.get_vertex(*int).unwrap().value.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            })
            .collect();
        write!(f, "{}", v.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;
    use std::collections::VecDeque;

    /// ```text
    ///    1
    ///   / \
    ///  /   \
    /// 4 --- 2
    ///   \   |
    ///    \  |
    ///     \ |
    /// 5 --- 3
    /// ```
    fn get_graph() -> Graph<i32> {
        let mut graph = Graph::new();
        graph.add_vertices(vec![1, 2, 3, 4, 5]);
        graph.add_edge(0, 1);
        graph.add_edge(1, 0);

        graph.add_edge(0, 3);
        graph.add_edge(3, 0);

        graph.add_edge(1, 2);
        graph.add_edge(2, 1);

        graph.add_edge(1, 3);
        graph.add_edge(3, 1);

        graph.add_edge(2, 3);
        graph.add_edge(3, 2);

        graph.add_edge(2, 4);
        graph.add_edge(4, 2);
        graph
    }

    #[test]
    #[cfg_attr(not(feature = "util"), ignore)]
    fn test_is_valid() {
        let mut g = get_graph();
        assert!(g.is_valid());
        g.add_edge(0, g.n_vertices());
        assert!(!g.is_valid());
    }

    #[test]
    #[cfg_attr(not(feature = "util"), ignore)]
    fn looped_add_vertex_is_equivalent_to_add_vertices() {
        let mut g1 = Graph::new();
        for i in 1..6 {
            g1.add_vertex(i);
        }
        let mut g2 = Graph::new();
        g2.add_vertices(1..6);

        assert_eq!(g1, g2)
    }

    #[test]
    #[cfg_attr(not(feature = "util"), ignore)]
    fn depth_first_search() {
        fn dfs_longest_path_2_to_5(
            g: &Graph<i32>,
            i: usize,
            current: usize,
            longest: &mut usize,
            visited: &mut [bool],
        ) {
            if visited[i] {
                return;
            }
            visited[i] = true;
            let vertex = g.get_vertex(i).unwrap();
            if *vertex.value == 5i32 {
                if current > *longest {
                    *longest = current;
                }
                visited[i] = false;
                return;
            }
            for edge in vertex.edges {
                dfs_longest_path_2_to_5(g, *edge, current + 1, longest, visited);
            }
            visited[i] = false;
        }

        let g = get_graph();
        let mut visited = vec![false; g.n_vertices()];
        let mut longest = 0;
        dfs_longest_path_2_to_5(&get_graph(), 1, 0, &mut longest, &mut visited);

        // 2 -> 1 -> 3 -> 4 -> 5
        assert_eq!(longest, 4);
    }

    #[test]
    #[cfg_attr(not(feature = "util"), ignore)]
    fn breadth_first_search() {
        fn bfs_shortest_path_2_to_5(g: &Graph<i32>) -> Option<usize> {
            let mut queue = VecDeque::new();
            let mut visited = vec![false; g.n_vertices()];
            let mut distances = vec![None; g.n_vertices()];
            let i_start = 1usize;
            let i_stop = 4usize;
            distances[i_start] = Some(0);
            queue.push_back(i_start);
            visited[i_start] = true;

            while let Some(i) = queue.pop_front() {
                let vertex = g.get_vertex(i).unwrap();

                for &edge in vertex.edges {
                    if visited[edge] {
                        continue;
                    }
                    visited[edge] = true;

                    distances[edge] = Some(distances[i].unwrap() + 1);
                    if g.get_vertex(edge).unwrap().value == &5 {
                        return distances[i_stop];
                    }
                    queue.push_back(edge);
                }
            }
            distances[i_stop]
        }

        // 2 -> 3 -> 5
        assert_eq!(bfs_shortest_path_2_to_5(&get_graph()), Some(2));
    }

    #[test]
    #[cfg_attr(not(feature = "util"), ignore)]
    fn test_display() {
        let s1 = format!("{}", get_graph());
        let s2 = "1: 2, 4
2: 1, 3, 4
3: 2, 4, 5
4: 1, 2, 3
5: 3"
            .to_string();
        assert_eq!(s1, s2);
    }
}
