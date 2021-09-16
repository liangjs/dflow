use petgraph::graph::NodeIndex;
use petgraph::{data as graph_data, visit as graph_visit};

pub trait CFG<Node>
where
    Self: graph_visit::GraphBase<NodeId = NodeIndex>,
    Self: graph_visit::Data<NodeWeight = Node>,
    Self: graph_visit::GraphProp<EdgeType = petgraph::Directed>,
    Self: graph_visit::IntoNeighborsDirected,
    Self: graph_visit::IntoNodeReferences,
    Self: graph_data::DataMap,
{
}

pub trait FlowState
where
    Self: Clone + Eq,
{
    fn empty() -> Self;
    fn merge(&mut self, other: &Self);
}

pub trait Analysis {
    type State: FlowState;

    fn entry_influx(&self) -> Self::State;

    fn flow_through<NodeRef>(&self, node: NodeRef, influx: &Self::State) -> Self::State
    where
        NodeRef: graph_visit::NodeRef;

    fn init_outflux<NodeRef>(&self, #[allow(unused_variables)] node: NodeRef) -> Self::State
    where
        NodeRef: graph_visit::NodeRef,
    {
        Self::State::empty()
    }

    fn update_state(&self, old: &mut Self::State, new: Self::State) -> bool {
        if new.eq(old) {
            false
        } else {
            *old = new;
            true
        }
    }
}

pub struct FlowResult<State: FlowState> {
    pub influx: State,
    pub outflux: State,
}

mod flow;
pub use flow::analyse;

#[cfg(test)]
mod test;
