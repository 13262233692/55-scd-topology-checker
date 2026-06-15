use crate::model::{DirectedGraph, NodeIndex};

struct TarjanFrame {
    node: NodeIndex,
    neighbor_idx: usize,
    visited: bool,
}

pub struct TarjanSCC;

impl TarjanSCC {
    pub fn new() -> Self {
        Self
    }

    pub fn compute(graph: &DirectedGraph<'_>) -> Vec<Vec<NodeIndex>> {
        let n = graph.node_count();
        if n == 0 {
            return Vec::new();
        }

        let mut index = 0;
        let mut indices = vec![None; n];
        let mut lowlink = vec![0; n];
        let mut on_stack = vec![false; n];
        let mut stack = Vec::with_capacity(n);
        let mut sccs = Vec::new();

        let mut call_stack: Vec<TarjanFrame> = Vec::with_capacity(n);

        for start_v in 0..n {
            if indices[start_v].is_some() {
                continue;
            }

            call_stack.push(TarjanFrame {
                node: start_v,
                neighbor_idx: 0,
                visited: false,
            });

            while let Some(frame) = call_stack.last_mut() {
                let v = frame.node;

                if !frame.visited {
                    frame.visited = true;
                    indices[v] = Some(index);
                    lowlink[v] = index;
                    index += 1;
                    stack.push(v);
                    on_stack[v] = true;
                }

                let neighbors = &graph.adjacency[v];

                if frame.neighbor_idx < neighbors.len() {
                    let w = neighbors[frame.neighbor_idx];
                    frame.neighbor_idx += 1;

                    if indices[w].is_none() {
                        call_stack.push(TarjanFrame {
                            node: w,
                            neighbor_idx: 0,
                            visited: false,
                        });
                    } else if on_stack[w] {
                        lowlink[v] = lowlink[v].min(indices[w].unwrap());
                    }
                } else {
                    if lowlink[v] == indices[v].unwrap() {
                        let mut scc = Vec::new();
                        loop {
                            let w = stack.pop().unwrap();
                            on_stack[w] = false;
                            scc.push(w);
                            if w == v {
                                break;
                            }
                        }
                        sccs.push(scc);
                    }

                    call_stack.pop();

                    if let Some(parent_frame) = call_stack.last_mut() {
                        let parent_v = parent_frame.node;
                        lowlink[parent_v] = lowlink[parent_v].min(lowlink[v]);
                    }
                }
            }
        }

        sccs
    }

    pub fn has_cycle(graph: &DirectedGraph<'_>) -> bool {
        let sccs = Self::compute(graph);
        sccs.iter().any(|scc| scc.len() > 1)
    }

    pub fn largest_scc_size(graph: &DirectedGraph<'_>) -> usize {
        let sccs = Self::compute(graph);
        sccs.iter().map(|scc| scc.len()).max().unwrap_or(0)
    }

    pub fn scc_count(graph: &DirectedGraph<'_>) -> usize {
        Self::compute(graph).len()
    }
}

impl Default for TarjanSCC {
    fn default() -> Self {
        Self::new()
    }
}

pub struct KosarajuSCC;

impl KosarajuSCC {
    pub fn new() -> Self {
        Self
    }

    pub fn compute(graph: &DirectedGraph<'_>) -> Vec<Vec<NodeIndex>> {
        let n = graph.node_count();
        if n == 0 {
            return Vec::new();
        }

        let mut visited = vec![false; n];
        let mut order = Vec::with_capacity(n);

        for start_v in 0..n {
            if visited[start_v] {
                continue;
            }

            let mut stack = vec![(start_v, false)];

            while let Some((v, processed)) = stack.pop() {
                if processed {
                    order.push(v);
                    continue;
                }

                if visited[v] {
                    continue;
                }
                visited[v] = true;

                stack.push((v, true));

                for &u in graph.adjacency[v].iter().rev() {
                    if !visited[u] {
                        stack.push((u, false));
                    }
                }
            }
        }

        let mut visited = vec![false; n];
        let mut sccs = Vec::new();

        for &start_v in order.iter().rev() {
            if visited[start_v] {
                continue;
            }

            let mut scc = Vec::new();
            let mut stack = vec![start_v];
            visited[start_v] = true;

            while let Some(v) = stack.pop() {
                scc.push(v);

                for &u in &graph.reverse_adjacency[v] {
                    if !visited[u] {
                        visited[u] = true;
                        stack.push(u);
                    }
                }
            }

            sccs.push(scc);
        }

        sccs
    }
}

impl Default for KosarajuSCC {
    fn default() -> Self {
        Self::new()
    }
}
