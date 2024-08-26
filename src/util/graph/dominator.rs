use std::{cmp::Ordering, collections::{HashMap, HashSet}};
use super::search::postorder_dfs;

pub fn dominator_tree(graph: &[Vec<usize>], root: usize) -> Vec<Vec<usize>> {
    let n = graph.len();
    
    let (post_order, mut predecessor_sets) = dominator_post_order(&graph, root);
    let mut node_to_post_order_idx = vec![0; n];
    for (i, &j) in post_order.iter().enumerate() {
        node_to_post_order_idx[j] = i;
    }

    let idx_to_predecessor_vec = predecessor_sets_to_idx_vecs(&post_order, &node_to_post_order_idx, &mut predecessor_sets);
    let mut dominators = vec![usize::MAX; n];
    dominators[n - 1] = n - 1;
    let mut changed = true;
    while changed {
        changed = false;

        for idx in (0..n - 1).rev() {
            let new_idom_idx = {
                let mut predecessors = idx_to_predecessor_vec[idx]
                    .iter()
                    .filter(|&&p| dominators[p] != usize::MAX);
                let new_idom_idx = predecessors.next().unwrap();
                predecessors.fold(*new_idom_idx, |new_idom_idx, &predecessor_idx| {
                    intersect(&dominators, new_idom_idx, predecessor_idx)
                })
            };
            if new_idom_idx != dominators[idx] {
                dominators[idx] = new_idom_idx;
                changed = true;
            }
        }
    }

    let mut tree = vec![vec!(); n];

    for (idx, dom_idx) in dominators.into_iter().enumerate() {
        tree[post_order[dom_idx]].push(post_order[idx]);
    }
    tree
}



fn intersect(dominators: &[usize], mut finger1: usize, mut finger2: usize) -> usize {
    loop {
        match finger1.cmp(&finger2) {
            Ordering::Less => finger1 = dominators[finger1],
            Ordering::Greater => finger2 = dominators[finger2],
            Ordering::Equal => return finger1,
        }
    }
}

fn dominator_post_order(graph: &[Vec<usize>], root: usize) -> (Vec<usize>, HashMap<usize, HashSet<usize>>) {
    let post_order = postorder_dfs(&graph, root);
    let mut predecessor_sets = HashMap::new();

    for node in postorder_dfs(&graph, root) {
        for &successor in &graph[node] {
            predecessor_sets
                .entry(successor)
                .or_insert_with(HashSet::new)
                .insert(node);
        }
    }

    (post_order, predecessor_sets)
}

fn predecessor_sets_to_idx_vecs(
    post_order: &[usize],
    node_to_post_order_idx: &[usize],
    predecessor_sets: &mut HashMap<usize, HashSet<usize>>,
) -> Vec<Vec<usize>> {
    post_order
        .iter()
        .map(|node| {
            predecessor_sets
                .remove(node)
                .map(|predecessors| {
                    predecessors
                        .into_iter()
                        .map(|p| node_to_post_order_idx[p])
                        .collect()
                })
                .unwrap_or_default()
        })
        .collect()
}


#[test]
fn dominator_tree_test() {
    let graph = vec!(vec!(1, 4), vec!(2), vec!(3), vec!(5), vec!(3), vec!());
    let mut tree = dominator_tree(&graph, 0);
    for x in tree.iter_mut() {
        x.sort();
    }
    let tree2 = vec!(vec!(0, 1, 3, 4), vec!(2), vec!(), vec!(5), vec!(), vec!());
    assert_eq!(tree, tree2);
}