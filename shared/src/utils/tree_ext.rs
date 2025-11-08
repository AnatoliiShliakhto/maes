use crate::models::TreeNode;
use ::indexmap::IndexMap;
use ::std::collections::{HashMap, VecDeque};

pub trait TreeExt {
    fn add_node(&mut self, node: TreeNode);
    fn root_nodes(&self) -> Vec<String>;
    fn remove_node(&mut self, id: impl AsRef<str>);
    fn node_parent(&self, id: impl AsRef<str>) -> String;
    fn node_path(&self, id: impl AsRef<str>) -> String;
    fn node_descendants(&self, id: impl AsRef<str>) -> Vec<String>;
    fn populate_children(&mut self);
    fn sort_by_name(&mut self);
    fn node_path_ids(&self, id: impl AsRef<str>) -> Vec<String>;
}

#[inline]
fn build_children_index<'a, I>(nodes: I) -> HashMap<&'a str, Vec<&'a str>>
where
    I: IntoIterator<Item = &'a TreeNode>,
{
    let mut children: HashMap<&str, Vec<&str>> = HashMap::new();
    for n in nodes {
        children
            .entry(n.parent.as_str())
            .or_default()
            .push(n.id.as_str());
    }
    children
}

#[inline]
fn bfs_descendants_with_index(
    root: String,
    index: &HashMap<&str, Vec<&str>>,
    upper_bound: usize,
) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(upper_bound.max(1));
    let mut queue: VecDeque<String> = VecDeque::with_capacity(upper_bound.max(1));

    result.push(root.clone());
    queue.push_back(root);

    while let Some(current_id) = queue.pop_front() {
        if let Some(children) = index.get(current_id.as_str()) {
            for &child_id in children {
                let owned = child_id.to_owned();
                result.push(owned.clone());
                queue.push_back(owned);
            }
        }
    }

    result
}

#[inline]
fn collect_path_with_get<'a, F>(id: impl AsRef<str>, mut get: F) -> String
where
    F: FnMut(&str) -> Option<&'a TreeNode>,
{
    let mut path: Vec<String> = Vec::with_capacity(8);
    let mut current_id = id.as_ref().to_owned();

    let mut hops = 0usize;
    while let Some(node) = get(current_id.as_str()) {
        path.push(node.name.clone());
        if node.parent.is_empty() {
            break;
        }
        current_id = node.parent.clone();
        hops += 1;
        if hops > 10_000 {
            break;
        }
    }

    path.join(" ")
}

#[inline]
fn collect_path_ids_with_get<'a, F>(id: impl AsRef<str>, mut get: F) -> Vec<String>
where
    F: FnMut(&str) -> Option<&'a TreeNode>,
{
    let mut path: Vec<String> = Vec::with_capacity(8);
    let mut current_id = id.as_ref().to_owned();

    let mut hops = 0usize;
    while let Some(node) = get(current_id.as_str()) {
        path.push(node.id.clone());
        if node.parent.is_empty() {
            break;
        }
        current_id = node.parent.clone();
        hops += 1;
        if hops > 10_000 {
            break;
        }
    }

    path.iter().rev().map(|s| s.to_owned()).collect()
}

impl TreeExt for Vec<TreeNode> {
    fn add_node(&mut self, node: TreeNode) {
        let id = node.id.clone();
        let parent_id = if node.parent.is_empty() {
            None
        } else {
            Some(node.parent.clone())
        };

        self.push(node);

        if let Some(pid) = parent_id {
            if let Some(parent) = self.iter_mut().find(|n| n.id == pid) {
                if !parent.children.iter().any(|c| c == &id) {
                    parent.children.push(id);
                }
            }
        }
    }

    fn root_nodes(&self) -> Vec<String> {
        self.iter()
            .filter(|n| n.parent.is_empty())
            .map(|n| n.id.clone())
            .collect()
    }

    fn remove_node(&mut self, id: impl AsRef<str>) {
        let id_str = id.as_ref();

        let mut parent_id: Option<String> = None;
        let mut pos: Option<usize> = None;
        for (i, n) in self.iter().enumerate() {
            if n.id == id_str {
                parent_id = if n.parent.is_empty() {
                    None
                } else {
                    Some(n.parent.clone())
                };
                pos = Some(i);
                break;
            }
        }

        if let Some(i) = pos {
            let removed_id = self.remove(i).id;
            if let Some(pid) = parent_id {
                if let Some(parent) = self.iter_mut().find(|n| n.id == pid) {
                    parent.children.retain(|child| child != &removed_id);
                }
            }
        }
    }

    fn node_parent(&self, id: impl AsRef<str>) -> String {
        self.iter()
            .find(|n| n.id == id.as_ref())
            .map(|n| n.parent.clone())
            .unwrap_or_default()
    }

    fn node_path(&self, id: impl AsRef<str>) -> String {
        let mut map: HashMap<&str, &TreeNode> = HashMap::with_capacity(self.len());
        for n in self {
            map.insert(n.id.as_str(), n);
        }
        collect_path_with_get(id, |key| map.get(key).copied())
    }

    fn node_descendants(&self, id: impl AsRef<str>) -> Vec<String> {
        let root = id.as_ref().to_owned();
        let index = build_children_index(self.iter());
        bfs_descendants_with_index(root, &index, self.len())
    }

    fn populate_children(&mut self) {
        let mut index: HashMap<String, Vec<String>> = HashMap::with_capacity(self.len());
        for n in self.iter() {
            index
                .entry(n.parent.clone())
                .or_default()
                .push(n.id.clone());
        }

        for i in 0..self.len() {
            let node_id = self[i].id.clone();
            let children = index.remove(&node_id).unwrap_or_default();
            self[i].children = children;
        }
    }

    fn sort_by_name(&mut self) {
        self.sort_by(|a, b|
            a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    }

    fn node_path_ids(&self, id: impl AsRef<str>) -> Vec<String> {
        let mut map: HashMap<&str, &TreeNode> = HashMap::with_capacity(self.len());
        for n in self {
            map.insert(n.id.as_str(), n);
        }
        collect_path_ids_with_get(id, |key| map.get(key).copied())
    }
}

impl TreeExt for HashMap<String, TreeNode> {
    fn add_node(&mut self, node: TreeNode) {
        let id = node.id.clone();
        let parent_id = if node.parent.is_empty() {
            None
        } else {
            Some(node.parent.clone())
        };

        self.insert(id.clone(), node);

        if let Some(pid) = parent_id {
            if let Some(parent) = self.get_mut(&pid) {
                if !parent.children.iter().any(|c| c == &id) {
                    parent.children.push(id);
                }
            }
        }
    }

    fn root_nodes(&self) -> Vec<String> {
        self.values()
            .filter(|n| n.parent.is_empty())
            .map(|n| n.id.clone())
            .collect()
    }

    fn remove_node(&mut self, id: impl AsRef<str>) {
        let key = id.as_ref();

        let parent_id = self.get(key).map(|n| n.parent.clone());

        if self.remove(key).is_none() {
            return;
        }

        if let Some(pid) = parent_id {
            if let Some(parent) = self.get_mut(&pid) {
                parent.children.retain(|child| child != key);
            }
        }
    }

    fn node_parent(&self, id: impl AsRef<str>) -> String {
        self.get(id.as_ref())
            .map(|n| n.parent.clone())
            .unwrap_or_default()
    }

    fn node_path(&self, id: impl AsRef<str>) -> String {
        collect_path_with_get(id, |key| self.get(key))
    }

    fn node_descendants(&self, id: impl AsRef<str>) -> Vec<String> {
        let root = id.as_ref().to_owned();
        let index = build_children_index(self.values());
        bfs_descendants_with_index(root, &index, self.len())
    }

    fn populate_children(&mut self) {
        let mut index: HashMap<String, Vec<String>> = HashMap::with_capacity(self.len());
        for (key, n) in self.iter() {
            index.entry(n.parent.clone()).or_default().push(key.clone());
        }

        let keys: Vec<String> = self.keys().cloned().collect();
        for node_id in keys {
            if let Some(node) = self.get_mut(&node_id) {
                let children = index.remove(&node_id).unwrap_or_default();
                node.children = children;
            }
        }
    }

    fn sort_by_name(&mut self) {}

    fn node_path_ids(&self, id: impl AsRef<str>) -> Vec<String> {
        collect_path_ids_with_get(id, |key| self.get(key))
    }
}

impl TreeExt for IndexMap<String, TreeNode> {
    fn add_node(&mut self, node: TreeNode) {
        let id = node.id.clone();
        let parent_id = if node.parent.is_empty() {
            None
        } else {
            Some(node.parent.clone())
        };

        self.insert(id.clone(), node);

        if let Some(pid) = parent_id {
            if let Some(parent) = self.get_mut(&pid) {
                if !parent.children.iter().any(|c| c == &id) {
                    parent.children.push(id);
                }
            }
        }
    }

    fn root_nodes(&self) -> Vec<String> {
        self.values()
            .filter(|n| n.parent.is_empty())
            .map(|n| n.id.clone())
            .collect()
    }

    fn remove_node(&mut self, id: impl AsRef<str>) {
        let key = id.as_ref();

        let parent_id = self.get(key).map(|n| n.parent.clone());

        if self.shift_remove(key).is_none() {
            return;
        }

        if let Some(pid) = parent_id
            && let Some(parent) = self.get_mut(&pid)
        {
            parent.children.retain(|child| child != key);
        }
    }

    fn node_parent(&self, id: impl AsRef<str>) -> String {
        self.get(id.as_ref())
            .map(|n| n.parent.clone())
            .unwrap_or_default()
    }

    fn node_path(&self, id: impl AsRef<str>) -> String {
        collect_path_with_get(id, |key| self.get(key))
    }

    fn node_descendants(&self, id: impl AsRef<str>) -> Vec<String> {
        let root = id.as_ref().to_owned();
        let index = build_children_index(self.values());
        bfs_descendants_with_index(root, &index, self.len())
    }

    fn populate_children(&mut self) {
        let mut index: HashMap<String, Vec<String>> = HashMap::with_capacity(self.len());
        for (key, n) in self.iter() {
            index.entry(n.parent.clone()).or_default().push(key.clone());
        }

        let keys: Vec<String> = self.keys().cloned().collect();
        for node_id in keys {
            if let Some(node) = self.get_mut(&node_id) {
                let children = index.remove(&node_id).unwrap_or_default();
                node.children = children;
            }
        }
    }

    fn sort_by_name(&mut self) {
        let mut order: Vec<(String, String)> = self
            .iter()
            .map(|(k, v)| (k.clone(), v.name.clone()))
            .collect();

        order.sort_by(|a, b| {
            let (an, bn) = (&a.1, &b.1);
            let alc = an.to_lowercase();
            let blc = bn.to_lowercase();
            alc.cmp(&blc)
                .then_with(|| an.cmp(bn))
                .then_with(|| a.0.cmp(&b.0))
        });

        let mut tmp = IndexMap::with_capacity(self.len());
        for (k, _) in order {
            if let Some(v) = self.shift_remove(&k) {
                tmp.insert(k, v);
            }
        }
        *self = tmp;
    }

    fn node_path_ids(&self, id: impl AsRef<str>) -> Vec<String> {
        collect_path_ids_with_get(id, |key| self.get(key))
    }
}
