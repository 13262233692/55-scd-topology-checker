use crate::model::*;
use crate::graph::scc::TarjanSCC;

pub struct TopologyAnalyzer;

impl TopologyAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(scd_model: &SCDModel<'_>, graph: &DirectedGraph<'_>) -> TopologyStats {
        let total_ieds = scd_model.ieds.len();
        let total_access_points: usize = scd_model.ieds.iter().map(|ied| ied.access_points.len()).sum();

        let mut total_goose_pubs = 0;
        let mut total_goose_subs = 0;
        let mut total_sv_pubs = 0;
        let mut total_sv_subs = 0;

        for ied in &scd_model.ieds {
            for ap in &ied.access_points {
                total_goose_pubs += ap.goose_pubs.len();
                total_goose_subs += ap.goose_subs.len();
                total_sv_pubs += ap.sv_pubs.len();
                total_sv_subs += ap.sv_subs.len();
            }
        }

        let total_goose_connections = scd_model.goose_connections.len();
        let total_sv_connections = scd_model.sv_connections.len();

        let graph_nodes = graph.node_count();
        let graph_edges = graph.edge_count();

        let sccs = TarjanSCC::compute(graph);
        let scc_count = sccs.len();
        let largest_scc_size = sccs.iter().map(|s| s.len()).max().unwrap_or(0);

        let mut isolated_nodes = 0;
        let mut total_out = 0;
        let mut max_out = 0;
        let mut total_in = 0;
        let mut max_in = 0;

        for i in 0..graph_nodes {
            let out_deg = graph.out_degree(i);
            let in_deg = graph.in_degree(i);

            if out_deg == 0 && in_deg == 0 {
                isolated_nodes += 1;
            }

            total_out += out_deg;
            max_out = max_out.max(out_deg);
            total_in += in_deg;
            max_in = max_in.max(in_deg);
        }

        let avg_fan_out = if graph_nodes > 0 {
            total_out as f64 / graph_nodes as f64
        } else {
            0.0
        };

        let avg_fan_in = if graph_nodes > 0 {
            total_in as f64 / graph_nodes as f64
        } else {
            0.0
        };

        TopologyStats {
            total_ieds,
            total_access_points,
            total_goose_pubs,
            total_goose_subs,
            total_sv_pubs,
            total_sv_subs,
            total_goose_connections,
            total_sv_connections,
            graph_nodes,
            graph_edges,
            scc_count,
            largest_scc_size,
            isolated_nodes,
            avg_fan_out,
            max_fan_out: max_out,
            avg_fan_in,
            max_fan_in: max_in,
        }
    }

    pub fn print_stats(stats: &TopologyStats) {
        println!("{}", "=".repeat(60));
        println!("      变电站配置拓扑特征统计报告");
        println!("{}", "=".repeat(60));
        println!();
        println!("【 设备规模统计 】");
        println!("  智能电子设备(IED)总数:   {:>8}", stats.total_ieds);
        println!("  接入点(AccessPoint)总数:  {:>8}", stats.total_access_points);
        println!();
        println!("【 虚端子规模统计 】");
        println!("  GOOSE 发布端总数:         {:>8}", stats.total_goose_pubs);
        println!("  GOOSE 订阅端总数:         {:>8}", stats.total_goose_subs);
        println!("  SV 发布端总数:            {:>8}", stats.total_sv_pubs);
        println!("  SV 订阅端总数:            {:>8}", stats.total_sv_subs);
        println!("  GOOSE 连接数:             {:>8}", stats.total_goose_connections);
        println!("  SV 连接数:                {:>8}", stats.total_sv_connections);
        println!();
        println!("【 图模型特征 】");
        println!("  图节点总数:               {:>8}", stats.graph_nodes);
        println!("  图边总数:                 {:>8}", stats.graph_edges);
        println!("  强连通分量(SCC)数:        {:>8}", stats.scc_count);
        println!("  最大SCC规模:              {:>8}", stats.largest_scc_size);
        println!("  孤立节点数:               {:>8}", stats.isolated_nodes);
        println!();
        println!("【 出入度分析 】");
        println!("  平均出度(扇出):           {:>12.2}", stats.avg_fan_out);
        println!("  最大出度(扇出):           {:>8}", stats.max_fan_out);
        println!("  平均入度(扇入):           {:>12.2}", stats.avg_fan_in);
        println!("  最大入度(扇入):           {:>8}", stats.max_fan_in);
        println!();
        println!("{}", "=".repeat(60));
    }

    pub fn print_graphviz(graph: &DirectedGraph<'_>) {
        println!("digraph SCDTopology {{");
        println!("  rankdir=LR;");
        println!("  node [shape=box, style=filled, color=lightblue];");
        println!();

        for node in &graph.nodes {
            let label = format!("{}\\n({})", node.ied_name, node.ap_name);
            let color = match node.node_type {
                NodeType::Publisher => "fillcolor=lightgreen",
                NodeType::Subscriber => "fillcolor=lightcoral",
                NodeType::Switch => "fillcolor=gold",
                NodeType::IED => "fillcolor=lightblue",
            };
            println!("  \"{}_{}\" [label=\"{}\", {}];", node.ied_name, node.ap_name, label, color);
        }

        for (from, neighbors) in graph.adjacency.iter().enumerate() {
            let from_node = &graph.nodes[from];
            for &to in neighbors {
                let to_node = &graph.nodes[to];
                println!("  \"{}_{}\" -> \"{}_{}\";", from_node.ied_name, from_node.ap_name, to_node.ied_name, to_node.ap_name);
            }
        }

        println!("}}");
    }
}

impl Default for TopologyAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
