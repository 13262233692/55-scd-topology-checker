use ahash::{AHashMap, AHashSet};

use crate::model::*;
use crate::graph::scc::TarjanSCC;

pub struct IsolationValidator {
    violations: Vec<IsolationViolation>,
}

impl IsolationValidator {
    pub fn new() -> Self {
        Self {
            violations: Vec::new(),
        }
    }

    pub fn validate_all(&mut self, scd_model: &SCDModel<'_>, graph: &DirectedGraph<'_>) -> &[IsolationViolation] {
        self.check_for_loops(graph);
        self.check_cross_zone_connections(scd_model, graph);
        self.check_vlan_consistency(scd_model);
        self.check_unauthorized_subscriptions(scd_model);
        self.check_redundant_paths(graph);
        self.check_protection_isolation(scd_model, graph);

        &self.violations
    }

    fn check_for_loops(&mut self, graph: &DirectedGraph<'_>) {
        let sccs = TarjanSCC::compute(graph);

        for scc in &sccs {
            if scc.len() > 1 {
                let involved_nodes: Vec<String> = scc
                    .iter()
                    .map(|&idx| {
                        let node = &graph.nodes[idx];
                        format!("{}/{}", node.ied_name, node.ap_name)
                    })
                    .collect();

                self.violations.push(IsolationViolation {
                    description: format!(
                        "检测到信号环路，包含 {} 个节点，违反继电保护单向传输原则",
                        scc.len()
                    ),
                    severity: ViolationSeverity::Critical,
                    involved_nodes,
                    violation_type: ViolationType::LoopDetected,
                });
            }
        }
    }

    fn check_cross_zone_connections(&mut self, scd_model: &SCDModel<'_>, graph: &DirectedGraph<'_>) {
        let mut ap_zones: AHashMap<(&str, &str), &str> = AHashMap::new();

        for sub_net in &scd_model.sub_networks {
            let zone_name = sub_net.name;
            for &(ied_name, ap_name) in &sub_net.access_points {
                ap_zones.insert((ied_name, ap_name), zone_name);
            }
        }

        if ap_zones.is_empty() {
            return;
        }

        for (from_idx, neighbors) in graph.adjacency.iter().enumerate() {
            let from_node = &graph.nodes[from_idx];
            let from_key = (from_node.ied_name, from_node.ap_name);
            let from_zone = ap_zones.get(&from_key);

            for &to_idx in neighbors {
                let to_node = &graph.nodes[to_idx];
                let to_key = (to_node.ied_name, to_node.ap_name);
                let to_zone = ap_zones.get(&to_key);

                if let (Some(fz), Some(tz)) = (from_zone, to_zone) {
                    if fz != tz {
                        self.violations.push(IsolationViolation {
                            description: format!(
                                "跨安全分区连接: {}/{} -> {}/{} ({} -> {})",
                                from_node.ied_name, from_node.ap_name,
                                to_node.ied_name, to_node.ap_name,
                                fz, tz
                            ),
                            severity: ViolationSeverity::High,
                            involved_nodes: vec![
                                format!("{}/{}", from_node.ied_name, from_node.ap_name),
                                format!("{}/{}", to_node.ied_name, to_node.ap_name),
                            ],
                            violation_type: ViolationType::CrossZoneConnection,
                        });
                    }
                }
            }
        }
    }

    fn check_vlan_consistency(&mut self, scd_model: &SCDModel<'_>) {
        use ahash::AHashMap;
        let mut vlan_map: AHashMap<(&str, &str, &str), Vec<&str>> = AHashMap::new();

        for ied in &scd_model.ieds {
            for ap in &ied.access_points {
                for vt in &ap.goose_pubs {
                    if let Some(vlan) = vt.vlan_id {
                        let key = (vt.ied_name, vt.ap_name, vt.cb_name);
                        vlan_map.entry(key).or_default().push(vlan);
                    }
                }
                for vt in &ap.sv_pubs {
                    if let Some(vlan) = vt.vlan_id {
                        let key = (vt.ied_name, vt.ap_name, vt.cb_name);
                        vlan_map.entry(key).or_default().push(vlan);
                    }
                }
            }
        }

        for ied in &scd_model.ieds {
            for ap in &ied.access_points {
                for vt in &ap.goose_subs {
                    let key = (vt.ied_name, vt.ap_name, vt.cb_name);
                    if let Some(pub_vlans) = vlan_map.get(&key) {
                        if !pub_vlans.is_empty() {
                            if let Some(sub_vlan) = vt.vlan_id {
                                if !pub_vlans.contains(&sub_vlan) {
                                    self.violations.push(IsolationViolation {
                                        description: format!(
                                            "GOOSE 订阅 VLAN 不匹配: {}/{}.{} 订阅 VLAN={}, 发布 VLAN={}",
                                            ied.name, ap.name, vt.cb_name,
                                            sub_vlan, pub_vlans[0]
                                        ),
                                        severity: ViolationSeverity::Medium,
                                        involved_nodes: vec![
                                            format!("{}/{}", vt.ied_name, vt.ap_name),
                                            format!("{}/{}", ied.name, ap.name),
                                        ],
                                        violation_type: ViolationType::VlanMismatch,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn check_unauthorized_subscriptions(&mut self, scd_model: &SCDModel<'_>) {
        use ahash::AHashSet;
        let mut pub_set: AHashSet<(&str, &str, &str)> = AHashSet::new();

        for ied in &scd_model.ieds {
            for ap in &ied.access_points {
                for vt in &ap.goose_pubs {
                    pub_set.insert((vt.ied_name, vt.ap_name, vt.cb_name));
                }
                for vt in &ap.sv_pubs {
                    pub_set.insert((vt.ied_name, vt.ap_name, vt.cb_name));
                }
            }
        }

        for ied in &scd_model.ieds {
            for ap in &ied.access_points {
                for vt in &ap.goose_subs {
                    let key = (vt.ied_name, vt.ap_name, vt.cb_name);
                    if !pub_set.contains(&key) && !vt.ied_name.is_empty() && !vt.cb_name.is_empty() {
                        self.violations.push(IsolationViolation {
                            description: format!(
                                "未授权 GOOSE 订阅: {}/{} 订阅不存在的发布 {}/{}.{}",
                                ied.name, ap.name, vt.ied_name, vt.ap_name, vt.cb_name
                            ),
                            severity: ViolationSeverity::High,
                            involved_nodes: vec![
                                format!("{}/{}", ied.name, ap.name),
                            ],
                            violation_type: ViolationType::UnauthorizedSubscription,
                        });
                    }
                }

                for vt in &ap.sv_subs {
                    let key = (vt.ied_name, vt.ap_name, vt.cb_name);
                    if !pub_set.contains(&key) && !vt.ied_name.is_empty() && !vt.cb_name.is_empty() {
                        self.violations.push(IsolationViolation {
                            description: format!(
                                "未授权 SV 订阅: {}/{} 订阅不存在的发布 {}/{}.{}",
                                ied.name, ap.name, vt.ied_name, vt.ap_name, vt.cb_name
                            ),
                            severity: ViolationSeverity::High,
                            involved_nodes: vec![
                                format!("{}/{}", ied.name, ap.name),
                            ],
                            violation_type: ViolationType::UnauthorizedSubscription,
                        });
                    }
                }
            }
        }
    }

    fn check_redundant_paths(&mut self, graph: &DirectedGraph<'_>) {
        for (from_idx, neighbors) in graph.adjacency.iter().enumerate() {
            let mut unique_targets = AHashSet::new();
            let mut duplicates = Vec::new();

            for &to_idx in neighbors {
                if !unique_targets.insert(to_idx) {
                    duplicates.push(to_idx);
                }
            }

            if !duplicates.is_empty() {
                let from_node = &graph.nodes[from_idx];
                let involved: Vec<String> = std::iter::once(format!("{}/{}", from_node.ied_name, from_node.ap_name))
                    .chain(duplicates.iter().map(|&idx| {
                        let node = &graph.nodes[idx];
                        format!("{}/{}", node.ied_name, node.ap_name)
                    }))
                    .collect();

                self.violations.push(IsolationViolation {
                    description: format!(
                        "检测到冗余路径: {}/{} 存在 {} 条重复连接",
                        from_node.ied_name, from_node.ap_name, duplicates.len()
                    ),
                    severity: ViolationSeverity::Low,
                    involved_nodes: involved,
                    violation_type: ViolationType::RedundantPath,
                });
            }
        }
    }

    fn check_protection_isolation(&mut self, _scd_model: &SCDModel<'_>, graph: &DirectedGraph<'_>) {
        let protection_keywords = ["PROT", "PROTECTION", "保护", "继电保护", "线路保护", "主变保护"];
        let process_keywords = ["PROCESS", "过程层", "SV", "SMV"];

        let mut protection_nodes: Vec<usize> = Vec::new();
        let mut process_nodes: Vec<usize> = Vec::new();

        for (idx, node) in graph.nodes.iter().enumerate() {
            let name_upper = node.ied_name.to_uppercase();
            if protection_keywords.iter().any(|kw| name_upper.contains(kw)) {
                protection_nodes.push(idx);
            }
            if process_keywords.iter().any(|kw| name_upper.contains(kw)) {
                process_nodes.push(idx);
            }
        }

        for &prot_idx in &protection_nodes {
            let reachable = Self::bfs_reachable(graph, prot_idx);
            for &proc_idx in &process_nodes {
                if reachable.contains(&proc_idx) {
                    let prot_node = &graph.nodes[prot_idx];
                    let proc_node = &graph.nodes[proc_idx];
                    self.violations.push(IsolationViolation {
                        description: format!(
                            "保护装置直连过程层设备: {}/{} -> {}/{} 可能存在安全风险",
                            prot_node.ied_name, prot_node.ap_name,
                            proc_node.ied_name, proc_node.ap_name
                        ),
                        severity: ViolationSeverity::Medium,
                        involved_nodes: vec![
                            format!("{}/{}", prot_node.ied_name, prot_node.ap_name),
                            format!("{}/{}", proc_node.ied_name, proc_node.ap_name),
                        ],
                        violation_type: ViolationType::CrossZoneConnection,
                    });
                }
            }
        }
    }

    fn bfs_reachable(graph: &DirectedGraph<'_>, start: usize) -> AHashSet<usize> {
        let mut visited = AHashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start);
        visited.insert(start);

        while let Some(node) = queue.pop_front() {
            for &neighbor in &graph.adjacency[node] {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        visited
    }

    pub fn print_report(&self) {
        println!();
        println!("{}", "=".repeat(60));
        println!("      特高压继电保护安全隔离规范校验报告");
        println!("{}", "=".repeat(60));
        println!();

        if self.violations.is_empty() {
            println!("  ✅ 校验通过: 未发现安全隔离违规");
            println!();
            return;
        }

        let critical_count = self.violations.iter().filter(|v| matches!(v.severity, ViolationSeverity::Critical)).count();
        let high_count = self.violations.iter().filter(|v| matches!(v.severity, ViolationSeverity::High)).count();
        let medium_count = self.violations.iter().filter(|v| matches!(v.severity, ViolationSeverity::Medium)).count();
        let low_count = self.violations.iter().filter(|v| matches!(v.severity, ViolationSeverity::Low)).count();

        println!("【 违规统计 】");
        println!("  严重(Critical):  {:>4} 项", critical_count);
        println!("  高危(High):      {:>4} 项", high_count);
        println!("  中危(Medium):    {:>4} 项", medium_count);
        println!("  低危(Low):       {:>4} 项", low_count);
        println!("  合计:            {:>4} 项", self.violations.len());
        println!();

        println!("【 详细违规列表 】");
        println!();

        for (i, violation) in self.violations.iter().enumerate() {
            let severity_str = match violation.severity {
                ViolationSeverity::Critical => "🔴 严重",
                ViolationSeverity::High => "🟠 高危",
                ViolationSeverity::Medium => "🟡 中危",
                ViolationSeverity::Low => "🟢 低危",
            };

            let type_str = match violation.violation_type {
                ViolationType::CrossZoneConnection => "跨区连接",
                ViolationType::LoopDetected => "环路检测",
                ViolationType::UnauthorizedSubscription => "未授权订阅",
                ViolationType::RedundantPath => "冗余路径",
                ViolationType::VlanMismatch => "VLAN 不匹配",
            };

            println!("  [{:02}] {} [{}]", i + 1, severity_str, type_str);
            println!("       {}", violation.description);
            if !violation.involved_nodes.is_empty() {
                println!("       涉及节点: {}", violation.involved_nodes.join(", "));
            }
            println!();
        }

        println!("{}", "=".repeat(60));
    }

    pub fn violations(&self) -> &[IsolationViolation] {
        &self.violations
    }
}

impl Default for IsolationValidator {
    fn default() -> Self {
        Self::new()
    }
}
