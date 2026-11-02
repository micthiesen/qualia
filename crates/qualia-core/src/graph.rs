use std::collections::{BTreeSet, HashMap};
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NodeId(pub String);

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Port families. Connections never convert between them implicitly; a
/// conversion is an explicit adapter operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PortType {
    Signal,
    Event,
    Field,
    EntitySet,
    Geometry,
    Material,
    State,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Port {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: PortType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Operator {
    pub id: NodeId,
    /// Operator kind in the catalogue, for example `elastic-lattice`.
    pub kind: String,
    #[serde(default)]
    pub inputs: Vec<Port>,
    #[serde(default)]
    pub outputs: Vec<Port>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Endpoint {
    pub node: NodeId,
    pub port: String,
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.node, self.port)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Timing {
    /// Read within the same step; orders the consumer after the producer.
    #[default]
    Immediate,
    /// Read the producer's value from the previous step.
    Delayed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Edge {
    pub from: Endpoint,
    pub to: Endpoint,
    #[serde(default)]
    pub timing: Timing,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldGraph {
    pub operators: Vec<Operator>,
    #[serde(default)]
    pub edges: Vec<Edge>,
}

/// Operator execution order for one simulation step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schedule {
    pub order: Vec<NodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum GraphError {
    #[error("operator id `{0}` is used more than once")]
    DuplicateOperator(NodeId),
    #[error("edge refers to unknown operator `{0}`")]
    UnknownOperator(NodeId),
    #[error("`{0}` is not an output port")]
    UnknownOutput(Endpoint),
    #[error("`{0}` is not an input port")]
    UnknownInput(Endpoint),
    #[error("cannot connect {from_ty:?} `{from}` to {to_ty:?} `{to}` without an adapter")]
    TypeMismatch {
        from: Endpoint,
        from_ty: PortType,
        to: Endpoint,
        to_ty: PortType,
    },
    #[error("input `{0}` has more than one incoming edge")]
    MultipleDrivers(Endpoint),
    #[error("immediate edges form a cycle through {0:?}; use a delayed edge for feedback")]
    InstantaneousCycle(Vec<NodeId>),
}

impl WorldGraph {
    /// Checks the graph's contracts and returns the step order. Ties keep
    /// declaration order so the schedule is deterministic.
    pub fn schedule(&self) -> Result<Schedule, GraphError> {
        let mut index = HashMap::with_capacity(self.operators.len());
        for (i, op) in self.operators.iter().enumerate() {
            if index.insert(&op.id, i).is_some() {
                return Err(GraphError::DuplicateOperator(op.id.clone()));
            }
        }

        let mut driven = BTreeSet::new();
        let mut successors = vec![Vec::new(); self.operators.len()];
        let mut in_degree = vec![0usize; self.operators.len()];
        for edge in &self.edges {
            let from = lookup(&index, &edge.from.node)?;
            let to = lookup(&index, &edge.to.node)?;
            let from_ty = port_type(&self.operators[from].outputs, &edge.from)
                .ok_or_else(|| GraphError::UnknownOutput(edge.from.clone()))?;
            let to_ty = port_type(&self.operators[to].inputs, &edge.to)
                .ok_or_else(|| GraphError::UnknownInput(edge.to.clone()))?;
            if from_ty != to_ty {
                return Err(GraphError::TypeMismatch {
                    from: edge.from.clone(),
                    from_ty,
                    to: edge.to.clone(),
                    to_ty,
                });
            }
            if !driven.insert((to, edge.to.port.as_str())) {
                return Err(GraphError::MultipleDrivers(edge.to.clone()));
            }
            if edge.timing == Timing::Immediate {
                successors[from].push(to);
                in_degree[to] += 1;
            }
        }

        let mut ready: BTreeSet<usize> = (0..self.operators.len())
            .filter(|&i| in_degree[i] == 0)
            .collect();
        let mut order = Vec::with_capacity(self.operators.len());
        while let Some(i) = ready.pop_first() {
            order.push(self.operators[i].id.clone());
            for &next in &successors[i] {
                in_degree[next] -= 1;
                if in_degree[next] == 0 {
                    ready.insert(next);
                }
            }
        }

        if order.len() < self.operators.len() {
            let stuck = (0..self.operators.len())
                .filter(|&i| in_degree[i] > 0)
                .map(|i| self.operators[i].id.clone())
                .collect();
            return Err(GraphError::InstantaneousCycle(stuck));
        }
        Ok(Schedule { order })
    }
}

fn lookup(index: &HashMap<&NodeId, usize>, id: &NodeId) -> Result<usize, GraphError> {
    index
        .get(id)
        .copied()
        .ok_or_else(|| GraphError::UnknownOperator(id.clone()))
}

fn port_type(ports: &[Port], endpoint: &Endpoint) -> Option<PortType> {
    ports
        .iter()
        .find(|port| port.name == endpoint.port)
        .map(|port| port.ty)
}
