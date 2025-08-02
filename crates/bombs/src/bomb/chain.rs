//! Chain reaction logic for bombs.

use std::collections::{HashMap, HashSet};

use petgraph::Undirected;
use petgraph::graph::Graph;
use serde::{Deserialize, Serialize};

use super::entity::{Bomb, BombId};

/// Identifier for a bomb chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BombChainId(pub u32);

/// Group of bombs that will explode together due to chain reactions.
#[derive(Debug, Clone)]
pub struct BombChain {
    /// Chain identifier.
    pub id: BombChainId,
    /// Bombs contained in this chain.
    pub bombs: Vec<BombId>,
    /// The bomb whose timer triggers the chain.
    pub trigger_bomb: BombId,
    /// Tick at which the chain explodes.
    pub explosion_time: u8,
}

/// Determines if `b1`'s explosion reaches `b2` ignoring obstacles.
fn bombs_in_range(b1: &Bomb, b2: &Bomb) -> bool {
    if b1.position.0 == b2.position.0 {
        let dist = b1.position.1.abs_diff(b2.position.1);
        dist <= b1.power as u16
    } else if b1.position.1 == b2.position.1 {
        let dist = b1.position.0.abs_diff(b2.position.0);
        dist <= b1.power as u16
    } else {
        false
    }
}

/// Build an undirected graph linking bombs that can trigger each other.
fn build_bomb_graph(bombs: &HashMap<BombId, Bomb>) -> Graph<BombId, (), Undirected> {
    let mut graph = Graph::<BombId, (), Undirected>::new_undirected();
    let mut nodes = HashMap::<BombId, petgraph::graph::NodeIndex>::new();
    for &id in bombs.keys() {
        let idx = graph.add_node(id);
        nodes.insert(id, idx);
    }
    let ids: Vec<_> = bombs.keys().copied().collect();
    for i in 0..ids.len() {
        for j in (i + 1)..ids.len() {
            let b1 = &bombs[&ids[i]];
            let b2 = &bombs[&ids[j]];
            if bombs_in_range(b1, b2) || bombs_in_range(b2, b1) {
                graph.add_edge(nodes[&ids[i]], nodes[&ids[j]], ());
            }
        }
    }
    graph
}

/// Find bomb chains based on adjacency.
pub fn find_bomb_chains(bombs: &HashMap<BombId, Bomb>) -> Vec<BombChain> {
    let graph = build_bomb_graph(bombs);
    let mut chains = Vec::new();
    let mut visited = HashSet::new();
    let mut chain_id_counter = 0u32;

    for node in graph.node_indices() {
        if visited.contains(&node) {
            continue;
        }
        let mut stack = vec![node];
        let mut component = Vec::new();
        while let Some(n) = stack.pop() {
            if !visited.insert(n) {
                continue;
            }
            component.push(graph[n]);
            for neigh in graph.neighbors(n) {
                if !visited.contains(&neigh) {
                    stack.push(neigh);
                }
            }
        }
        let trigger = component
            .iter()
            .copied()
            .min_by_key(|id| bombs[id].timer)
            .expect("component not empty");
        let chain = BombChain {
            id: BombChainId(chain_id_counter),
            bombs: component,
            trigger_bomb: trigger,
            explosion_time: bombs[&trigger].timer,
        };
        chains.push(chain);
        chain_id_counter += 1;
    }

    chains
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_chain_reactions() {
        let mut bombs = HashMap::new();
        let b1 = Bomb::new(BombId(1), 0, (1, 1), 1, 2);
        let b2 = Bomb::new(BombId(2), 0, (3, 1), 5, 2);
        let b3 = Bomb::new(BombId(3), 0, (0, 4), 3, 1);
        bombs.insert(b1.id, b1);
        bombs.insert(b2.id, b2);
        bombs.insert(b3.id, b3);

        let chains = find_bomb_chains(&bombs);
        assert_eq!(chains.len(), 2);
        let chain = chains
            .iter()
            .find(|c| c.bombs.contains(&BombId(1)))
            .unwrap();
        assert_eq!(chain.bombs.len(), 2);
        assert_eq!(chain.trigger_bomb, BombId(1));
        assert_eq!(chain.explosion_time, 1);
    }
}
