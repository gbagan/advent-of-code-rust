pub fn postorder_dfs(graph: &[Vec<usize>], source: usize) -> Vec<usize> {
    let mut output = vec!();
    let mut visited = vec![false; graph.len()];
    postorder_dfs_aux(graph, &mut visited, &mut output, source); 
    output
}

fn postorder_dfs_aux(graph: &[Vec<usize>], visited: &mut Vec<bool>, output: &mut Vec<usize>, v: usize) {
    if visited[v] {
        return;
    }
    visited[v] = true;
    for &u in &graph[v] {
        postorder_dfs_aux(graph, visited, output, u);
    }
    output.push(v);
}

#[test]
fn postorder_dfs_test() {
    let graph = vec!(vec!(1, 2), vec!(3), vec!(3), vec!());
    assert_eq!(postorder_dfs(&graph, 0), vec!(3, 1, 2, 0));
}