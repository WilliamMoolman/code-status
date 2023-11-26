use std::collections::VecDeque;

use slab_tree::{NodeId, Tree, TreeBuilder};

pub struct RepoTree {
    tree: Tree<String>,
    root_id: NodeId,
}

impl RepoTree {
    pub fn new(root: String) -> RepoTree {
        let tree = TreeBuilder::new().with_root(root).build();
        let root_id = tree.root_id().expect("root doesn't exist?");
        RepoTree { tree, root_id }
    }

    pub fn add_path(&mut self, path: String) {
        let folders: VecDeque<String> = path.split('/').map(|x| x.to_string()).collect();
        self.add_branch(folders, self.root_id)
    }

    fn add_branch(&mut self, mut branch: VecDeque<String>, join_id: NodeId) {
        let children = self.children(join_id);
        // let children_names: Vec<&str> = children
        //     .iter()
        //     .map(|x| *self.tree.get(*x).unwrap().data())
        //     .collect();
        if let Some(data) = branch.pop_front() {
            for node_id in children {
                if &data == self.tree.get(node_id).unwrap().data() {
                    return self.add_branch(branch, node_id);
                }
            }
            let new_id = self.tree.get_mut(join_id).unwrap().append(data).node_id();
            self.add_branch(branch, new_id)
        }
    }

    fn children(&self, parent_id: NodeId) -> Vec<NodeId> {
        let mut nodes = vec![];
        if let None = self.tree.get(parent_id).unwrap().first_child() {
            return nodes;
        }
        let first_child_id = self
            .tree
            .get(parent_id)
            .unwrap()
            .first_child()
            .unwrap()
            .node_id();

        let mut node_id = first_child_id;

        loop {
            let sibling = self.tree.get(node_id).unwrap();

            nodes.push(sibling.node_id());

            if let Some(sibling) = sibling.next_sibling() {
                node_id = sibling.node_id();
            } else {
                // No more siblings
                break;
            }
        }

        nodes
    }

    pub fn print(&self) {
        self.print_children(self.root_id, "")
    }

    fn print_children(&self, parent_id: NodeId, prefix: &str) {
        const PIPE: &str = "│    "; // prefix: pipe
        const TEE: &str = "├── "; // connector: tee
        const SPACE: &str = "     "; // prefix: no more siblings
        const ELBOW: &str = "└── "; // connector: elbow

        let children = self.children(parent_id);
        if children.len() == 0 {
            return;
        }
        let last_id = *children.last().unwrap();
        for child_id in children {
            let sibling = self.tree.get(child_id).unwrap();
            if child_id != last_id {
                println!("{}{}{}", prefix, TEE, sibling.data());
                let new_prefix = String::from(prefix) + PIPE;
                self.print_children(child_id, &new_prefix)
            } else {
                println!("{}{}{}", prefix, ELBOW, sibling.data());
                let new_prefix = String::from(prefix) + SPACE;
                self.print_children(child_id, &new_prefix)
            }
        }
    }
}
