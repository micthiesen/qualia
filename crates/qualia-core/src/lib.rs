//! Qualia's world program representation.
//!
//! A world is a typed graph of operators. Within one simulation step the graph
//! of immediate edges must be acyclic; artistic feedback goes through
//! [`Timing::Delayed`] edges, which read the previous step's state. That keeps
//! update order and replay well defined.

mod graph;

pub use graph::{
    Edge, Endpoint, GraphError, NodeId, Operator, Port, PortType, Schedule, Timing, WorldGraph,
};
