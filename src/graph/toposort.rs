use std::collections::VecDeque;

use crate::model::*;

pub struct KahnTopoSort;

impl KahnTopoSort {
    pub fn new() -> Self {
        Self
    }

    pub fn sort(graph: &DirectedGraph<'_>) -> Result<Vec<NodeIndex>, Vec<NodeIndex>> {
        let n = graph.node_count();
        if n == 0 {
            return Ok(Vec::new());
        }

        let mut in_degree: Vec<usize> = (0..n).map(|i| graph.in_degree(i)).collect();
        let mut queue = VecDeque::with_capacity(n);

        for i in 0..n {
            if in_degree[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut result = Vec::with_capacity(n);

        while let Some(u) = queue.pop_front() {
            result.push(u);

            for &v in &graph.adjacency[u] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }

        if result.len() == n {
            Ok(result)
        } else {
            let remaining: Vec<NodeIndex> = (0..n)
                .filter(|&i| in_degree[i] > 0)
                .collect();
            Err(remaining)
        }
    }

    pub fn has_topological_order(graph: &DirectedGraph<'_>) -> bool {
        Self::sort(graph).is_ok()
    }
}

impl Default for KahnTopoSort {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TimingAnalyzer;

impl TimingAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn classify_ied_role(ied_name: &str, node_type: NodeType) -> IEDRole {
        if matches!(node_type, NodeType::Switch) {
            return IEDRole::Switch;
        }

        let upper = ied_name.to_uppercase();

        if upper.contains("PROT") || upper.contains("PROTECTION") || upper.contains("保护") {
            IEDRole::ProtectionRelay
        } else if upper.contains("MU") || upper.contains("MERGING") || upper.contains("合并单元") {
            IEDRole::MergingUnit
        } else if upper.contains("BCU") || upper.contains("CONTROL") || upper.contains("控制") || upper.contains("BAY") {
            IEDRole::BayControlUnit
        } else if upper.contains("CB") || upper.contains("BREAKER") || upper.contains("断路器") || upper.contains("TRIP") {
            IEDRole::CircuitBreaker
        } else if upper.contains("MONITOR") || upper.contains("MON") || upper.contains("监视") {
            IEDRole::MonitoringDevice
        } else {
            IEDRole::Unknown
        }
    }

    pub fn get_delay_profile(role: &IEDRole) -> DelayProfile {
        match role {
            IEDRole::ProtectionRelay => DelayProfile::protection_device(),
            IEDRole::MergingUnit => DelayProfile::merging_unit(),
            IEDRole::BayControlUnit => DelayProfile::bay_control(),
            IEDRole::CircuitBreaker => DelayProfile::circuit_breaker(),
            IEDRole::Switch => DelayProfile::switch_node(),
            IEDRole::MonitoringDevice => DelayProfile {
                internal_processing_us: 180,
                bus_forwarding_us: 45,
                switch_backplane_us: 100,
                network_propagation_us: 50,
            },
            IEDRole::Unknown => DelayProfile::default(),
        }
    }

    pub fn analyze(
        graph: &DirectedGraph<'_>,
        threshold_ms: u64,
    ) -> TimingAnalysisResult {
        let n = graph.node_count();

        let node_roles: Vec<IEDRole> = graph
            .nodes
            .iter()
            .map(|node| Self::classify_ied_role(node.ied_name, node.node_type))
            .collect();

        let node_delay_profiles: Vec<DelayProfile> = node_roles
            .iter()
            .map(Self::get_delay_profile)
            .collect();

        let protection_triggers: Vec<usize> = (0..n)
            .filter(|&i| {
                matches!(node_roles[i], IEDRole::ProtectionRelay)
                    && graph.out_degree(i) > 0
            })
            .collect();

        let breaker_terminals: Vec<usize> = (0..n)
            .filter(|&i| {
                matches!(node_roles[i], IEDRole::CircuitBreaker | IEDRole::BayControlUnit)
                    && graph.in_degree(i) > 0
            })
            .collect();

        let topo_result = KahnTopoSort::sort(graph);
        let has_cycles = topo_result.is_err();

        let topological_order = match topo_result {
            Ok(order) => order,
            Err(cyclic_nodes) => {
                let mut visited = vec![false; n];
                let mut fallback_order = Vec::new();
                for &c in &cyclic_nodes {
                    if !visited[c] {
                        let mut stack = vec![c];
                        while let Some(u) = stack.pop() {
                            if visited[u] {
                                continue;
                            }
                            visited[u] = true;
                            fallback_order.push(u);
                            for &v in &graph.adjacency[u] {
                                if !visited[v] {
                                    stack.push(v);
                                }
                            }
                        }
                    }
                }
                for i in 0..n {
                    if !visited[i] {
                        fallback_order.push(i);
                    }
                }
                fallback_order
            }
        };

        let threshold_us = threshold_ms * 1000;
        let mut node_arrival_times = vec![0u64; n];

        for &u in &topological_order {
            let base_delay = if graph.in_degree(u) == 0 {
                0
            } else {
                node_arrival_times[u]
            };

            for &v in &graph.adjacency[u] {
                let edge_delay = node_delay_profiles[u].total_us();
                let arrival_via_u = base_delay.saturating_add(edge_delay);
                if arrival_via_u > node_arrival_times[v] {
                    node_arrival_times[v] = arrival_via_u;
                }
            }
        }

        let mut predecessors: Vec<Vec<(usize, u64)>> = vec![Vec::new(); n];
        for u in 0..n {
            for &v in &graph.adjacency[u] {
                let edge_delay = node_delay_profiles[u].total_us();
                predecessors[v].push((u, edge_delay));
            }
        }

        let mut all_paths = Vec::new();
        let mut critical_paths = Vec::new();
        let mut violations = Vec::new();

        for &trigger in &protection_triggers {
            for &terminal in &breaker_terminals {
                if let Some(paths) = Self::find_all_paths(
                    graph,
                    &node_delay_profiles,
                    trigger,
                    terminal,
                    threshold_us,
                ) {
                    for path in paths {
                        let exceeds = path.total_delay_us > threshold_us;
                        if exceeds {
                            let excess = path.total_delay_us - threshold_us;
                            let severity = if excess > 5000 {
                                ViolationSeverity::Critical
                            } else if excess > 2000 {
                                ViolationSeverity::High
                            } else {
                                ViolationSeverity::Medium
                            };
                            violations.push(TimingViolation {
                                path: path.clone(),
                                excess_us: excess,
                                severity,
                            });
                            critical_paths.push(path.clone());
                        }
                        all_paths.push(path);
                    }
                }
            }
        }

        TimingAnalysisResult {
            threshold_ms,
            topological_order,
            node_arrival_times,
            node_delay_profiles,
            critical_paths,
            all_paths,
            protection_triggers,
            breaker_terminals,
            violations,
            has_cycles,
        }
    }

    fn find_all_paths(
        graph: &DirectedGraph<'_>,
        delay_profiles: &[DelayProfile],
        start: usize,
        end: usize,
        _threshold_us: u64,
    ) -> Option<Vec<TimingPath>> {
        if start == end {
            return None;
        }

        let mut result = Vec::new();
        let mut stack = vec![(start, vec![start], 0u64, Vec::<(String, DelayProfile, u64)>::new())];

        while let Some((current, path, accumulated, per_node)) = stack.pop() {
            if current == end {
                let node_names: Vec<String> = path
                    .iter()
                    .map(|&idx| {
                        let n = &graph.nodes[idx];
                        format!("{}/{}", n.ied_name, n.ap_name)
                    })
                    .collect();

                let total = accumulated + delay_profiles[end].total_us();
                let mut final_per_node = per_node.clone();
                final_per_node.push((
                    node_names.last().unwrap().clone(),
                    delay_profiles[end],
                    delay_profiles[end].total_us(),
                ));

                let exceeds = false;
                result.push(TimingPath {
                    nodes: node_names,
                    total_delay_us: total,
                    per_node_delays: final_per_node,
                    is_critical: exceeds,
                    exceeds_threshold: exceeds,
                });
                continue;
            }

            let mut visited_in_path = std::collections::HashSet::new();
            for &p in &path {
                visited_in_path.insert(p);
            }

            for &neighbor in &graph.adjacency[current] {
                if !visited_in_path.contains(&neighbor) {
                    let mut new_path = path.clone();
                    new_path.push(neighbor);

                    let current_node_name = {
                        let n = &graph.nodes[current];
                        format!("{}/{}", n.ied_name, n.ap_name)
                    };

                    let mut new_per_node = per_node.clone();
                    new_per_node.push((
                        current_node_name,
                        delay_profiles[current],
                        delay_profiles[current].total_us(),
                    ));

                    let new_accum = accumulated.saturating_add(delay_profiles[current].total_us());

                    if new_path.len() < 64 {
                        stack.push((neighbor, new_path, new_accum, new_per_node));
                    }
                }
            }
        }

        if result.is_empty() {
            None
        } else {
            result.sort_by(|a, b| b.total_delay_us.cmp(&a.total_delay_us));
            Some(result)
        }
    }

    pub fn print_report(result: &TimingAnalysisResult) {
        println!();
        println!("{}", "=".repeat(70));
        println!("      继电保护信号传输时序动力学分析报告");
        println!("{}", "=".repeat(70));
        println!();

        println!("【 基础参数 】");
        println!("  安全阈值:                 {:>8} ms", result.threshold_ms);
        println!("  保护触发端数量:           {:>8}", result.protection_triggers.len());
        println!("  断路器终端数量:           {:>8}", result.breaker_terminals.len());
        println!("  检测到环路:               {:>8}", if result.has_cycles { "是 ⚠️" } else { "否" });
        println!();

        println!("【 节点延时分布 (µs) 】");
        println!("  {:<24} {:>10} {:>10} {:>10} {:>10} {:>10}",
            "节点", "内部处理", "总线转发", "背板", "网络传播", "合计");
        println!("  {}", "-".repeat(78));

        let unique_display: std::collections::HashSet<(String, DelayProfile)> = result
            .node_delay_profiles
            .iter()
            .enumerate()
            .map(|(i, dp)| {
                let n = format!("{}/{}", result.topological_order.get(i).map_or("", |&_| ""), i);
                (n, *dp)
            })
            .collect();

        for (role_name, dp) in [
            ("保护装置(PROT)", DelayProfile::protection_device()),
            ("合并单元(MU)", DelayProfile::merging_unit()),
            ("间隔控制(BCU)", DelayProfile::bay_control()),
            ("断路器(CB)", DelayProfile::circuit_breaker()),
            ("交换机(SW)", DelayProfile::switch_node()),
            ("默认(DEFAULT)", DelayProfile::default()),
        ].iter() {
            println!("  {:<24} {:>10} {:>10} {:>10} {:>10} {:>10}",
                role_name,
                dp.internal_processing_us,
                dp.bus_forwarding_us,
                dp.switch_backplane_us,
                dp.network_propagation_us,
                dp.total_us()
            );
        }
        let _ = unique_display;
        println!();

        if result.all_paths.is_empty() {
            println!("【 时序路径 】");
            println!("  未检测到从保护触发端到断路器跳闸端的有效路径");
            println!();
        } else {
            println!("【 保护时序路径统计 】");
            println!("  有效路径总数:             {:>8}", result.all_paths.len());
            let avg_delay: u64 = if !result.all_paths.is_empty() {
                result.all_paths.iter().map(|p| p.total_delay_us).sum::<u64>() / result.all_paths.len() as u64
            } else {
                0
            };
            let max_delay = result.all_paths.iter().map(|p| p.total_delay_us).max().unwrap_or(0);
            let min_delay = result.all_paths.iter().map(|p| p.total_delay_us).min().unwrap_or(0);
            println!("  平均传输延迟:             {:>8.2} ms", avg_delay as f64 / 1000.0);
            println!("  最大传输延迟:             {:>8.2} ms", max_delay as f64 / 1000.0);
            println!("  最小传输延迟:             {:>8.2} ms", min_delay as f64 / 1000.0);
            println!();
        }

        if result.violations.is_empty() {
            println!("{}", "=".repeat(70));
            println!("  ✅ 时序校验通过 - 所有保护虚回路延迟均在 {}ms 安全阈值内", result.threshold_ms);
            println!("{}", "=".repeat(70));
        } else {
            Self::print_violations_with_blink(result);
        }
    }

    fn print_violations_with_blink(result: &TimingAnalysisResult) {
        let red = "\x1b[1;31m";
        let blink = "\x1b[5m";
        let bold = "\x1b[1m";
        let yellow = "\x1b[1;33m";
        let bg_red = "\x1b[41m";
        let reset = "\x1b[0m";

        println!();
        println!("{bg_red}{blink}{bold}{red}{}", "⚠".repeat(70));
        println!("  ⚠⚠⚠  时序超限严重违规 - 保护动作延迟超过 {}ms 安全门槛  ⚠⚠⚠", result.threshold_ms);
        println!("{}{reset}", "⚠".repeat(70));
        println!();

        println!("  违规路径数量: {}{}{}", red, result.violations.len(), reset);
        println!();

        for (idx, violation) in result.violations.iter().enumerate() {
            let sev_str = match violation.severity {
                ViolationSeverity::Critical => format!("{}{}🔴 致命{}", red, blink, reset),
                ViolationSeverity::High => format!("{}{}🟠 高危{}", red, bold, reset),
                ViolationSeverity::Medium => format!("{}{}🟡 中危{}", yellow, bold, reset),
                ViolationSeverity::Low => "🟢 低危".to_string(),
            };

            println!("  {bg_red}{blink}【违规 #{:02}】{reset}  {}  超时: {}{:.2} ms{}",
                idx + 1,
                sev_str,
                red,
                violation.excess_us as f64 / 1000.0,
                reset
            );

            println!("  {}总延迟: {}{:.2} ms{}  (阈值: {} ms)",
                " ".repeat(8),
                red,
                violation.path.total_delay_us as f64 / 1000.0,
                reset,
                result.threshold_ms
            );

            println!("  {}路径链路:", " ".repeat(8));
            for (node_idx, (name, dp, delay)) in violation.path.per_node_delays.iter().enumerate() {
                let arrow = if node_idx < violation.path.per_node_delays.len() - 1 { " →" } else { "" };
                println!("    {bg_red}  [{:02}] {}{reset}  {}{:<30}{}  节点延时: {:>6} µs{}",
                    node_idx + 1,
                    arrow,
                    red,
                    name,
                    reset,
                    delay,
                    reset
                );
                let _ = dp;
            }

            println!("    {}└──────────── 累计: {}{}{:.2} ms {}{}超限{}{:.2} ms{reset}",
                " ".repeat(8),
                red,
                bold,
                violation.path.total_delay_us as f64 / 1000.0,
                blink,
                red,
                violation.excess_us as f64 / 1000.0,
                reset
            );
            println!();
        }

        println!("{bg_red}{blink}{bold}{red}{}", "=".repeat(70));
        println!("  ⚠⚠⚠  以上违规可能导致继电保护拒动或误动，请立即整改  ⚠⚠⚠");
        println!("{}{reset}", "=".repeat(70));
        println!();
    }
}

impl Default for TimingAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
