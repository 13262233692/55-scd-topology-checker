use crate::model::*;

pub struct GraphBuilder;

impl GraphBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build_from_scd<'a>(model: &'a SCDModel<'a>) -> DirectedGraph<'a> {
        let mut graph = DirectedGraph::new();

        for ied in &model.ieds {
            for ap in &ied.access_points {
                let has_pubs = !ap.goose_pubs.is_empty() || !ap.sv_pubs.is_empty();
                let has_subs = !ap.goose_subs.is_empty() || !ap.sv_subs.is_empty();

                let node_type = if has_pubs && has_subs {
                    NodeType::IED
                } else if has_pubs {
                    NodeType::Publisher
                } else if has_subs {
                    NodeType::Subscriber
                } else {
                    NodeType::IED
                };

                let node = GraphNode {
                    index: 0,
                    ied_name: ied.name,
                    ap_name: ap.name,
                    node_type,
                };
                graph.add_node(node);
            }
        }

        for ied in &model.ieds {
            for ap in &ied.access_points {
                let sub_key = (ied.name, ap.name);
                if let Some(&sub_idx) = graph.node_map.get(&sub_key) {
                    for sub_vt in &ap.goose_subs {
                        let pub_key = (sub_vt.ied_name, sub_vt.ap_name);
                        if let Some(&pub_idx) = graph.node_map.get(&pub_key) {
                            graph.add_edge(pub_idx, sub_idx);
                        }
                    }
                    for sub_vt in &ap.sv_subs {
                        let pub_key = (sub_vt.ied_name, sub_vt.ap_name);
                        if let Some(&pub_idx) = graph.node_map.get(&pub_key) {
                            graph.add_edge(pub_idx, sub_idx);
                        }
                    }
                }
            }
        }

        for (i, node) in graph.nodes.iter_mut().enumerate() {
            node.index = i;
        }

        graph
    }

    pub fn build_multilayer_graph<'a>(model: &'a SCDModel<'a>) -> DirectedGraph<'a> {
        let mut graph = DirectedGraph::new();

        for sub_net in &model.sub_networks {
            let node = GraphNode {
                index: 0,
                ied_name: sub_net.name,
                ap_name: "SWITCH",
                node_type: NodeType::Switch,
            };
            graph.add_node(node);
        }

        for ied in &model.ieds {
            for ap in &ied.access_points {
                let has_pubs = !ap.goose_pubs.is_empty() || !ap.sv_pubs.is_empty();
                let has_subs = !ap.goose_subs.is_empty() || !ap.sv_subs.is_empty();

                let node_type = if has_pubs {
                    NodeType::Publisher
                } else if has_subs {
                    NodeType::Subscriber
                } else {
                    NodeType::IED
                };

                let node = GraphNode {
                    index: 0,
                    ied_name: ied.name,
                    ap_name: ap.name,
                    node_type,
                };
                let node_idx = graph.add_node(node);

                for sub_net in &model.sub_networks {
                    for &(ied_n, ap_n) in &sub_net.access_points {
                        if ied_n == ied.name && ap_n == ap.name {
                            let switch_key = (sub_net.name, "SWITCH");
                            if let Some(&switch_idx) = graph.node_map.get(&switch_key) {
                                graph.add_edge(node_idx, switch_idx);
                                graph.add_edge(switch_idx, node_idx);
                            }
                        }
                    }
                }
            }
        }

        for (i, node) in graph.nodes.iter_mut().enumerate() {
            node.index = i;
        }

        graph
    }
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}
