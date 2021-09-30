use std::collections::{HashMap, HashSet};

use petgraph::visit::NodeRef;

use crate::*;

pub fn analyse<Graph, Node, Analyser>(
    analyser: &Analyser,
    graph: Graph,
    entry: NodeIndex,
) -> HashMap<NodeIndex, FlowResult<Analyser::State>>
where
    Graph: CFG<Node>,
    Analyser: Analysis<NodeWeight = Node>,
{
    let mut results: HashMap<NodeIndex, FlowResult<Analyser::State>> = HashMap::new();
    let mut queue = HashSet::new();

    // initialize all nodes
    for node in graph.node_references() {
        let id = node.id();
        results.insert(
            id,
            FlowResult {
                influx: Analyser::State::empty(),
                outflux: analyser.init_outflux(node),
            },
        );
        queue.insert(id);
    }

    results.get_mut(&entry).unwrap().influx = analyser.entry_influx();

    // iterate until states converge
    while !queue.is_empty() {
        let node_id = *queue.iter().next().unwrap();
        queue.remove(&node_id);
        let node_weight = graph.node_weight(node_id).unwrap();
        let node = (node_id, node_weight);

        // build influx by merging predecessors
        let mut influx = Analyser::State::empty();
        for (cnt, pred) in graph
            .neighbors_directed(node_id, petgraph::Incoming)
            .enumerate()
        {
            let prev_outflux = &results.get(&pred).unwrap().outflux;
            if cnt == 0 {
                influx = prev_outflux.clone();
            } else {
                influx.merge(prev_outflux);
            }
        }

        // transform
        let outflux = analyser.flow_through(node, &influx);

        // update result
        let result = results.get_mut(&node_id).unwrap();
        result.influx = influx;
        if !analyser.update_state(&mut result.outflux, outflux) {
            // no update
            continue;
        }

        // mark successors to visit
        for succ in graph.neighbors_directed(node_id, petgraph::Outgoing) {
            queue.insert(succ);
        }
    }

    results
}
