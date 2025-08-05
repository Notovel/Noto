use slab_tree::{NextSiblings, NodeId, NodeRef, Tree};
use crate::editor::action::{Action};

#[derive(Debug)]
pub struct History {
    tree: Tree<Action>,
    current: NodeId,
}

#[derive(Debug)]
pub enum HistoryError {
    InvalidCurrentNode,
    NoParent,
}

impl History {
    pub fn new() -> Self {
        let mut tree = Tree::new();
        let root = tree.set_root(Action::DocumentInstantiation);
        Self {
            tree,
            current: root,
        }
    }

    pub fn append(&mut self, action: Action) -> Result<(), HistoryError> {
        if let Some(mut current_node) = self.tree.get_mut(self.current) {
            self.current = current_node.append(action).node_id();
            return Ok(());
        }
        Err(HistoryError::InvalidCurrentNode)
    }

    pub fn get_current(&self) -> Result<&Action, HistoryError> {
        if let Some(current_node) = self.tree.get(self.current) {
            return Ok(current_node.data());
        }
        Err(HistoryError::InvalidCurrentNode)
    }

    pub fn undo(&mut self) -> Result<&Action, HistoryError> {
        if let Some(current_node) = self.tree.get(self.current) {
            if let Some(parent) = current_node.parent() {
                self.current = parent.node_id();
                return Ok(current_node.data());
            }
            return Err(HistoryError::NoParent);
        }
        Err(HistoryError::InvalidCurrentNode)
    }

    pub fn get_next(&self) -> Result<Vec<NodeRef<Action>>, HistoryError> {
        if let Some(current) = self.tree.get(self.current) {
            let children = current.children();
            let mut result = Vec::with_capacity(current.children().count());
            for child in children {
                result.push(child);
            }
            return Ok(result);
        }
        Err(HistoryError::InvalidCurrentNode)
    }

    pub fn get_id (&self) -> NodeId {
        self.current
    }

    pub fn get_tree (&self) -> &Tree<Action> {
        &self.tree
    }
}