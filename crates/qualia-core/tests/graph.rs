// clippy.toml only covers #[test] functions, not their shared fixtures.
#![allow(clippy::unwrap_used)]

use qualia_core::{GraphError, NodeId, PortType, Timing, WorldGraph};

fn stress_lattice() -> WorldGraph {
    serde_json::from_str(include_str!("../../../worlds/stress-lattice.json")).unwrap()
}

fn ids(names: &[&str]) -> Vec<NodeId> {
    names.iter().map(|n| NodeId((*n).to_owned())).collect()
}

#[test]
fn delayed_feedback_schedules_in_dependency_order() {
    let schedule = stress_lattice().schedule().unwrap();
    assert_eq!(schedule.order, ids(&["events", "lattice", "fragments"]));
}

#[test]
fn immediate_feedback_is_an_instantaneous_cycle() {
    let mut graph = stress_lattice();
    graph.edges[2].timing = Timing::Immediate;
    assert_eq!(
        graph.schedule(),
        Err(GraphError::InstantaneousCycle(ids(&[
            "lattice",
            "fragments"
        ])))
    );
}

#[test]
fn ports_never_convert_implicitly() {
    let mut graph = stress_lattice();
    graph.operators[1].inputs[1].ty = PortType::Material;
    assert!(matches!(
        graph.schedule(),
        Err(GraphError::TypeMismatch {
            from_ty: PortType::Field,
            to_ty: PortType::Material,
            ..
        })
    ));
}

#[test]
fn an_input_has_one_driver() {
    let mut graph = stress_lattice();
    let mut extra = graph.edges[0].clone();
    extra.timing = Timing::Delayed;
    graph.edges.push(extra);
    assert!(matches!(
        graph.schedule(),
        Err(GraphError::MultipleDrivers(_))
    ));
}

#[test]
fn unknown_ports_are_rejected() {
    let mut graph = stress_lattice();
    graph.edges[0].to.port = "nope".into();
    assert!(matches!(graph.schedule(), Err(GraphError::UnknownInput(_))));
}
