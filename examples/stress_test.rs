use std::time::Instant;

use scd_topology_checker::model::{DirectedGraph, GraphNode, NodeType};
use scd_topology_checker::{TarjanSCC, KosarajuSCC};

fn build_deep_chain_graph(size: usize) -> DirectedGraph<'static> {
    let mut graph = DirectedGraph::new();

    for i in 0..size {
        let name = format!("NODE-{:08}", i);
        let name_static: &'static str = Box::leak(name.into_boxed_str());
        let node = GraphNode {
            index: i,
            ied_name: name_static,
            ap_name: "AP1",
            node_type: NodeType::IED,
        };
        graph.add_node(node);
    }

    for i in 0..size - 1 {
        graph.add_edge(i, i + 1);
    }

    graph
}

fn build_single_large_cycle(size: usize) -> DirectedGraph<'static> {
    let mut graph = DirectedGraph::new();

    for i in 0..size {
        let name = format!("CYCLE-{:08}", i);
        let name_static: &'static str = Box::leak(name.into_boxed_str());
        let node = GraphNode {
            index: i,
            ied_name: name_static,
            ap_name: "AP1",
            node_type: NodeType::IED,
        };
        graph.add_node(node);
    }

    for i in 0..size - 1 {
        graph.add_edge(i, i + 1);
    }
    graph.add_edge(size - 1, 0);

    graph
}

fn build_many_small_cycles(cycle_count: usize, cycle_size: usize) -> DirectedGraph<'static> {
    let total = cycle_count * cycle_size;
    let mut graph = DirectedGraph::new();

    for i in 0..total {
        let name = format!("SC-{:06}", i);
        let name_static: &'static str = Box::leak(name.into_boxed_str());
        let node = GraphNode {
            index: i,
            ied_name: name_static,
            ap_name: "AP1",
            node_type: NodeType::IED,
        };
        graph.add_node(node);
    }

    for c in 0..cycle_count {
        let base = c * cycle_size;
        for i in 0..cycle_size - 1 {
            graph.add_edge(base + i, base + i + 1);
        }
        graph.add_edge(base + cycle_size - 1, base);
    }

    graph
}

fn build_dense_graph(size: usize, edges_per_node: usize) -> DirectedGraph<'static> {
    let mut graph = DirectedGraph::new();

    for i in 0..size {
        let name = format!("DENSE-{:06}", i);
        let name_static: &'static str = Box::leak(name.into_boxed_str());
        let node = GraphNode {
            index: i,
            ied_name: name_static,
            ap_name: "AP1",
            node_type: NodeType::IED,
        };
        graph.add_node(node);
    }

    use rand::Rng;
    let mut rng = rand::thread_rng();

    for i in 0..size {
        for _ in 0..edges_per_node {
            let j = rng.gen_range(0..size);
            if i != j {
                graph.add_edge(i, j);
            }
        }
    }

    graph
}

fn main() {
    println!("============================================================");
    println!("  大规模图算法压力测试 - 栈溢出验证");
    println!("============================================================");
    println!();

    println!("【测试 1】深度链式图 - 10,000 节点单链");
    let graph = build_deep_chain_graph(10_000);
    println!("  节点数: {}, 边数: {}", graph.node_count(), graph.edge_count());

    let start = Instant::now();
    let tarjan_sccs = TarjanSCC::compute(&graph);
    let tarjan_time = start.elapsed();
    println!("  Tarjan SCC: {} 个分量, 耗时 {:.2?}", tarjan_sccs.len(), tarjan_time);
    assert_eq!(tarjan_sccs.len(), 10_000, "链式图应有 10000 个 SCC");

    let start = Instant::now();
    let kosaraju_sccs = KosarajuSCC::compute(&graph);
    let kosaraju_time = start.elapsed();
    println!("  Kosaraju SCC: {} 个分量, 耗时 {:.2?}", kosaraju_sccs.len(), kosaraju_time);
    assert_eq!(kosaraju_sccs.len(), 10_000, "链式图应有 10000 个 SCC");

    println!("  ✓ 通过 - 无栈溢出");
    println!();

    println!("【测试 2】单一大环图 - 10,000 节点单环");
    let graph = build_single_large_cycle(10_000);
    println!("  节点数: {}, 边数: {}", graph.node_count(), graph.edge_count());

    let start = Instant::now();
    let tarjan_sccs = TarjanSCC::compute(&graph);
    let tarjan_time = start.elapsed();
    println!("  Tarjan SCC: {} 个分量, 最大规模: {}, 耗时 {:.2?}",
        tarjan_sccs.len(),
        tarjan_sccs.iter().map(|s| s.len()).max().unwrap(),
        tarjan_time
    );
    assert_eq!(tarjan_sccs.len(), 1, "单环图应有 1 个 SCC");
    assert_eq!(tarjan_sccs[0].len(), 10_000, "SCC 规模应为 10000");

    let start = Instant::now();
    let kosaraju_sccs = KosarajuSCC::compute(&graph);
    let kosaraju_time = start.elapsed();
    println!("  Kosaraju SCC: {} 个分量, 最大规模: {}, 耗时 {:.2?}",
        kosaraju_sccs.len(),
        kosaraju_sccs.iter().map(|s| s.len()).max().unwrap(),
        kosaraju_time
    );
    assert_eq!(kosaraju_sccs.len(), 1, "单环图应有 1 个 SCC");

    println!("  ✓ 通过 - 无栈溢出");
    println!();

    println!("【测试 3】大量小环图 - 1000 个环 × 每环 10 节点 = 10,000 节点");
    let graph = build_many_small_cycles(1000, 10);
    println!("  节点数: {}, 边数: {}", graph.node_count(), graph.edge_count());

    let start = Instant::now();
    let tarjan_sccs = TarjanSCC::compute(&graph);
    let tarjan_time = start.elapsed();
    println!("  Tarjan SCC: {} 个分量, 耗时 {:.2?}", tarjan_sccs.len(), tarjan_time);
    assert_eq!(tarjan_sccs.len(), 1000, "1000个环应有 1000 个 SCC");

    let start = Instant::now();
    let kosaraju_sccs = KosarajuSCC::compute(&graph);
    let kosaraju_time = start.elapsed();
    println!("  Kosaraju SCC: {} 个分量, 耗时 {:.2?}", kosaraju_sccs.len(), kosaraju_time);
    assert_eq!(kosaraju_sccs.len(), 1000, "1000个环应有 1000 个 SCC");

    println!("  ✓ 通过 - 无栈溢出");
    println!();

    println!("【测试 4】超大规模图 - 50,000 节点链式图（终极考验）");
    let graph = build_deep_chain_graph(50_000);
    println!("  节点数: {}, 边数: {}", graph.node_count(), graph.edge_count());

    let start = Instant::now();
    let tarjan_sccs = TarjanSCC::compute(&graph);
    let tarjan_time = start.elapsed();
    println!("  Tarjan SCC: {} 个分量, 耗时 {:.2?}", tarjan_sccs.len(), tarjan_time);
    assert_eq!(tarjan_sccs.len(), 50_000, "链式图应有 50000 个 SCC");

    let start = Instant::now();
    let kosaraju_sccs = KosarajuSCC::compute(&graph);
    let kosaraju_time = start.elapsed();
    println!("  Kosaraju SCC: {} 个分量, 耗时 {:.2?}", kosaraju_sccs.len(), kosaraju_time);
    assert_eq!(kosaraju_sccs.len(), 50_000, "链式图应有 50000 个 SCC");

    println!("  ✓ 通过 - 无栈溢出");
    println!();

    println!("============================================================");
    println!("  所有压力测试通过！迭代算法完全免疫栈溢出");
    println!("============================================================");
}
