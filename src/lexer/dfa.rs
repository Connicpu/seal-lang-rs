use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

pub struct Dfa<State, Item: Hash + Eq> {
    nodes: Vec<DfaNode<State, Item>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node(usize);

struct DfaNode<State, Item: Hash + Eq> {
    state: Option<State>,
    default_edge: Option<Node>,
    explicit_edges: HashMap<Item, Option<Node>>,
    complex_edges: Vec<(Box<Fn(&Item) -> bool + Sync>, Node)>,
}

impl<State, Item: Hash + Eq> Dfa<State, Item> {
    pub fn new() -> Self {
        let mut dfa = Dfa { nodes: Vec::with_capacity(32) };
        dfa.create(None);
        dfa
    }

    pub fn root(&self) -> Node {
        Node(0)
    }

    pub fn state(&self, node: Node) -> Option<&State> {
        self.nodes[node.0].state.as_ref()
    }

    pub fn next(&self, state: Node, edge: &Item) -> Option<Node> {
        let state = &self.nodes[state.0];

        if let Some(&node) = state.explicit_edges.get(edge) {
            return node;
        }

        for &(ref func, node) in &state.complex_edges {
            if func(edge) {
                return Some(node);
            }
        }

        state.default_edge
    }

    pub fn create<S: Into<Option<State>>>(&mut self, state: S) -> Node {
        let id = Node(self.nodes.len());
        self.nodes.push(DfaNode {
            state: state.into(),
            default_edge: None,
            explicit_edges: HashMap::new(),
            complex_edges: vec![],
        });
        id
    }

    pub fn transition_default(&mut self, from: Node, to: Node) {
        self.nodes[from.0].default_edge = Some(to);
    }

    pub fn transition<N>(&mut self, from: Node, to: N, edge: Item)
        where N: Into<Option<Node>>
    {
        self.nodes[from.0].explicit_edges.insert(edge, to.into());
    }

    pub fn transition_complex<F>(&mut self, from: Node, to: Node, edge: F)
        where F: Fn(&Item) -> bool + Sync + 'static
    {
        self.nodes[from.0].complex_edges.push((Box::new(edge), to));
    }

    pub fn insert_string<I>(&mut self, start: Node, iter: I, state: State)
        where I: IntoIterator<Item = Item>
    {
        let mut node = start;
        for item in iter {
            let next = {
                let created_node = Node(self.nodes.len());
                let entry = self.nodes[node.0].explicit_edges.entry(item);
                match entry {
                    Entry::Vacant(entry) => {
                        entry.insert(Some(created_node));
                        None
                    }
                    Entry::Occupied(entry) => *entry.get(),
                }
            };

            if let Some(next) = next {
                node = next;
            } else {
                node = self.create(None);
            }
        }

        self.nodes[node.0].state = Some(state);
    }
}
