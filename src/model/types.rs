use ahash::AHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServiceType {
    GOOSE,
    SV,
    MMS,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VirtualTerminal<'a> {
    pub ied_name: &'a str,
    pub ap_name: &'a str,
    pub ld_inst: &'a str,
    pub cb_name: &'a str,
    pub service_type: ServiceType,
    pub mac_address: Option<&'a str>,
    pub app_id: Option<&'a str>,
    pub vlan_id: Option<&'a str>,
    pub vlan_priority: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessPoint<'a> {
    pub name: &'a str,
    pub ied_name: &'a str,
    pub goose_pubs: Vec<VirtualTerminal<'a>>,
    pub sv_pubs: Vec<VirtualTerminal<'a>>,
    pub goose_subs: Vec<VirtualTerminal<'a>>,
    pub sv_subs: Vec<VirtualTerminal<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IED<'a> {
    pub name: &'a str,
    pub ied_type: Option<&'a str>,
    pub manufacturer: Option<&'a str>,
    pub access_points: Vec<AccessPoint<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubNetwork<'a> {
    pub name: &'a str,
    pub type_attr: Option<&'a str>,
    pub access_points: Vec<(&'a str, &'a str)>,
}

#[derive(Debug, Default, Clone)]
pub struct SCDModel<'a> {
    pub ieds: Vec<IED<'a>>,
    pub sub_networks: Vec<SubNetwork<'a>>,
    pub goose_connections: Vec<(VirtualTerminal<'a>, Vec<VirtualTerminal<'a>>)>,
    pub sv_connections: Vec<(VirtualTerminal<'a>, Vec<VirtualTerminal<'a>>)>,
    pub header_info: Option<HeaderInfo<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderInfo<'a> {
    pub id: Option<&'a str>,
    pub version: Option<&'a str>,
    pub name_history: Vec<&'a str>,
}

#[derive(Debug, Clone)]
pub struct TopologyStats {
    pub total_ieds: usize,
    pub total_access_points: usize,
    pub total_goose_pubs: usize,
    pub total_goose_subs: usize,
    pub total_sv_pubs: usize,
    pub total_sv_subs: usize,
    pub total_goose_connections: usize,
    pub total_sv_connections: usize,
    pub graph_nodes: usize,
    pub graph_edges: usize,
    pub scc_count: usize,
    pub largest_scc_size: usize,
    pub isolated_nodes: usize,
    pub avg_fan_out: f64,
    pub max_fan_out: usize,
    pub avg_fan_in: f64,
    pub max_fan_in: usize,
}

#[derive(Debug, Clone)]
pub struct IsolationViolation {
    pub description: String,
    pub severity: ViolationSeverity,
    pub involved_nodes: Vec<String>,
    pub violation_type: ViolationType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationType {
    CrossZoneConnection,
    LoopDetected,
    UnauthorizedSubscription,
    RedundantPath,
    VlanMismatch,
}

pub type NodeIndex = usize;

#[derive(Debug, Clone)]
pub struct GraphNode<'a> {
    pub index: NodeIndex,
    pub ied_name: &'a str,
    pub ap_name: &'a str,
    pub node_type: NodeType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeType {
    Publisher,
    Subscriber,
    Switch,
    IED,
}

pub struct DirectedGraph<'a> {
    pub nodes: Vec<GraphNode<'a>>,
    pub adjacency: Vec<Vec<NodeIndex>>,
    pub reverse_adjacency: Vec<Vec<NodeIndex>>,
    pub node_map: AHashMap<(&'a str, &'a str), NodeIndex>,
}

impl<'a> DirectedGraph<'a> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            adjacency: Vec::new(),
            reverse_adjacency: Vec::new(),
            node_map: AHashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: GraphNode<'a>) -> NodeIndex {
        let key = (node.ied_name, node.ap_name);
        if let Some(&idx) = self.node_map.get(&key) {
            return idx;
        }
        let idx = self.nodes.len();
        self.node_map.insert(key, idx);
        self.nodes.push(node);
        self.adjacency.push(Vec::new());
        self.reverse_adjacency.push(Vec::new());
        idx
    }

    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        if from >= self.nodes.len() || to >= self.nodes.len() {
            return;
        }
        if !self.adjacency[from].contains(&to) {
            self.adjacency[from].push(to);
            self.reverse_adjacency[to].push(from);
        }
    }

    pub fn out_degree(&self, idx: NodeIndex) -> usize {
        self.adjacency.get(idx).map_or(0, |v| v.len())
    }

    pub fn in_degree(&self, idx: NodeIndex) -> usize {
        self.reverse_adjacency.get(idx).map_or(0, |v| v.len())
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.adjacency.iter().map(|v| v.len()).sum()
    }
}

impl<'a> Default for DirectedGraph<'a> {
    fn default() -> Self {
        Self::new()
    }
}
