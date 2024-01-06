use std::collections::{BinaryHeap, HashSet, HashMap};
use std::hash::Hash;
use std::iter::Sum;
use petgraph::algo::Measure;
use petgraph::{Graph, Undirected, graph::{IndexType, NodeIndex}};
use petgraph::visit::{EdgeRef, IntoEdges, IntoNodeIdentifiers, IntoEdgeReferences};

pub fn minimum_cut<G, F, K>(
    graph: G,
    weight_function: F,
) -> (Vec<G::EdgeId>, K)
where
    G: IntoEdges + IntoNodeIdentifiers + IntoEdgeReferences,
    F: Fn(G::EdgeRef) -> K,
    G::NodeId: Eq + Hash,
    K: Measure + Sum + Ord + Copy,
{
    let mut node_to_node = HashMap::new();
    let mut graph2 = Graph::new_undirected();
    for node in graph.node_identifiers() {
        let node2 = graph2.add_node(node);
        node_to_node.insert(node, node2);
    }

    for edge in graph.edge_references() {
        graph2.add_edge(
            node_to_node[&edge.source()],
            node_to_node[&edge.target()],
            (edge.id(), weight_function(edge))
        );
    }

    /*
    for node in graph.node_identifiers() {
        
        for edge in graph.edges(node) {
            let source = node_to_node[&edge.source()];
            let target = node_to_node[&edge.target()];
            if source < target {
                graph2.add_edge(
                    source,
                    target,
                    ((edge.source(), edge.target()), weight_function(edge))
                );
            }
        }
    }*/

    minimum_cut_aux(&mut graph2)
}


fn merge_vertices<E, Node, Ix>(
    graph: &mut Graph<Node, E, Undirected, Ix>,
    node1: NodeIndex<Ix>,
    node2: NodeIndex<Ix>,
)
where
    Ix: IndexType,
    E: Copy,
{
    let edges: Vec<_> = graph.edges(node2).map(|e| (node1, e.target(), *e.weight())).collect();
    for (s, t, w) in edges {
        if t != node1 && t != node2 {
            graph.add_edge(s, t, w);
        }
    }

    graph.remove_node(node2);
}

fn minimum_cut_step<E, Edge, Node, Ix>(
    graph: &Graph<Node, (Edge, E), Undirected, Ix>,
    start: NodeIndex<Ix>,
) -> (Vec<Edge>, E, NodeIndex<Ix>, NodeIndex<Ix>) 
where
    Ix: IndexType,
    E: Measure + Sum + Ord + Copy,
    Edge: Copy,
{
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    let mut seen_list = Vec::new();
    let mut weight_map: HashMap<NodeIndex<Ix>, E> = HashMap::new();
    let start_weight: E = graph.edges(start).map(|e| e.weight().1).sum();
    queue.push((start_weight, start));

    while let Some((_, node)) = queue.pop() {
        if !seen.contains(&node) {
            seen.insert(node);
            seen_list.push(node);
            for edge in graph.edges(node) {
                let target = edge.target();
                let weight = weight_map
                                .get(&edge.target())
                                .map_or_else(|| edge.weight().1
                                            , |&w| w + edge.weight().1);
                weight_map.insert(target, weight);
                queue.push((weight, target));
            }
        }
    }

    let len = seen_list.len();
    let last = seen_list[len-1];
    let before_last = seen_list[len-2];
    let edges: Vec<_> = graph.edges(last).map(|e| e.weight().0).collect();
    let weight: E = graph.edges(last).map(|e| e.weight().1).sum();
    
    (edges, weight, last, before_last)
}

fn minimum_cut_aux<E, Node, Edge, Ix>(
    graph: &mut Graph<Node, (Edge, E), Undirected, Ix>
) -> (Vec<Edge>, E)
where
    Ix: IndexType,
    E: Measure + Sum + Ord + Copy,
    Edge: Copy,
{
    let mut best_cut = vec![];
    let mut best_weight = graph.edge_weights().map(|e| e.1).sum();

    while graph.node_count() > 1 {
        let a = graph.node_indices().next().unwrap();
        let (cut, weight, node1, node2) = minimum_cut_step(&graph, a);
        if weight < best_weight {
            best_cut = cut;
            best_weight = weight;
        }
        merge_vertices(graph, node1, node2);
    }
    (best_cut, best_weight)
}