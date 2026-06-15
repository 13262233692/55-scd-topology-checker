use crate::model::{DirectedGraph, NodeIndex};

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

        for v in 0..n {
            if indices[v].is_none() {
                Self::strongconnect(
                    graph,
                    v,
                    &mut index,
                    &mut indices,
                    &mut lowlink,
                    &mut on_stack,
                    &mut stack,
                    &mut sccs,
                );
            }
        }

        sccs
    }

    fn strongconnect(
        graph: &DirectedGraph<'_>,
        v: NodeIndex,
        index: &mut usize,
        indices: &mut [Option<usize>],
        lowlink: &mut [usize],
        on_stack: &mut [bool],
        stack: &mut Vec<NodeIndex>,
        sccs: &mut Vec<Vec<NodeIndex>>,
    ) {
        indices[v] = Some(*index);
        lowlink[v] = *index;
        *index += 1;
        stack.push(v);
        on_stack[v] = true;

        for &w in &graph.adjacency[v] {
            if indices[w].is_none() {
                Self::strongconnect(graph, w, index, indices, lowlink, on_stack, stack, sccs);
                lowlink[v] = lowlink[v].min(lowlink[w]);
            } else if on_stack[w] {
                lowlink[v] = lowlink[v].min(indices[w].unwrap());
            }
        }

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

        for v in 0..n {
            if !visited[v] {
                Self::dfs1(graph, v, &mut visited, &mut order);
            }
        }

        let mut visited = vec![false; n];
        let mut sccs = Vec::new();

        for &v in order.iter().rev() {
            if !visited[v] {
                let mut scc = Vec::new();
                Self::dfs2(graph, v, &mut visited, &mut scc);
                sccs.push(scc);
            }
        }

        sccs
    }

    fn dfs1(graph: &DirectedGraph<'_>, v: NodeIndex, visited: &mut [bool], order: &mut Vec<NodeIndex>) {
        visited[v] = true;
        for &u in &graph.adjacency[v] {
            if !visited[u] {
                Self::dfs1(graph, u, visited, order);
            }
        }
        order.push(v);
    }

    fn dfs2(graph: &DirectedGraph<'_>, v: NodeIndex, visited: &mut [bool], scc: &mut Vec<NodeIndex>) {
        visited[v] = true;
        scc.push(v);
        for &u in &graph.reverse_adjacency[v] {
            if !visited[u] {
                Self::dfs2(graph, u, visited, scc);
            }
        }
    }
}

impl Default for KosarajuSCC {
    fn default() -> Self {
        Self::new()
    }
}
