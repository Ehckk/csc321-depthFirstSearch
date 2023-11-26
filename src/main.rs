/*
	Eric Miguel
	CSC-321 
	HW10 - DFS
*/
use std::collections::{HashMap, HashSet};

// Types
type Process = usize;
type Graph = HashMap<Process, HashSet<Process>>;

// Graph vertex color
#[derive(Debug)]
enum Color { White, Gray, Black }
impl Default for Color {
    fn default() -> Self { Color::White }
}

fn main() {
	// Process list
	let process_pairs = [
		(0, 1), (0, 2), (1, 3), (2, 1), (3, 2)
	];
	
	// Build graph 
	let mut graph = Graph::new();
	let mut color: Vec<Color> = Vec::new();

	for (parent, child) in process_pairs {
		graph.entry(parent).or_insert(HashSet::new()).insert(child);
	}


	for _ in 0..graph.keys().len() {
		color.push(Color::White);
	}
	
	// Modified DFS for cycle check
	fn dfs_cycle_check(current: Process, graph: &Graph, color: &mut Vec<Color>) -> bool {
		// If we are visiting a gray node, we have found a cycle
		if matches!(color[current], Color::Gray) {
			return true;
		}
		color[current] = Color::Gray;
		// Check adjacent nodes
		for adj in graph.get(&current).unwrap() {
			// Ignore completely visited vertices
			if matches!(color[*adj], Color::Black) { continue; }
			
			// Check next adj for cycle
			if !dfs_cycle_check(*adj, graph, color) {
				return true;
			}
		} 
		
		color[current] = Color::Black;
		return false;
	}

	// Perform a cycle check for each vertex
	for process in graph.keys() {
		if !dfs_cycle_check(*process, &graph, &mut color) { continue; }
		println!("Deadlock Found");
		return;
	}
	println!("No Deadlocks Found");

}

