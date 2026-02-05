use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use gpui::{App, UniformListScrollHandle, Window};

use crate::table::TableDelegate;

pub struct TableState<D: TableDelegate> {
    delegate: D,
    rows: RowRegistry<D>,
    selection: Selection,

    pub(crate) vertical_scroll_handle: UniformListScrollHandle,
}

impl<D: TableDelegate + 'static> TableState<D> {
    pub fn new(delegate: D, _window: &mut Window, cx: &App) -> Self {
        let rows = RowRegistry::from_delegate(&delegate, cx);
        Self { delegate, rows, vertical_scroll_handle: UniformListScrollHandle::new() }
    }

    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn delegate_mut(&mut self) -> &mut D {
        &mut self.delegate
    }

    pub fn rows(&self) -> &RowRegistry<D> {
        &self.rows
    }

    pub fn rows_mut(&mut self) -> &mut RowRegistry<D> {
        &mut self.rows
    }
}

/// Registry that maintains a flattened view of the tree of rows along with
/// expansion state and quick lookup from id -> index.
pub struct RowRegistry<D: TableDelegate> {
    nodes: Vec<RowNode<D::RowId>>,
    indices: HashMap<D::RowId, usize>,
    visible_depths_cache: Vec<(D::RowId, usize)>,
    expanded: HashSet<D::RowId>,
    max_depth: usize,
}

#[derive(Clone, Debug)]
struct RowNode<Id> {
    id: Id,
    parent: Option<usize>,
    children: Vec<usize>,
    depth: usize,
}

impl<D: TableDelegate> RowRegistry<D> {
    pub fn from_delegate(delegate: &D, cx: &App) -> Self {
        let mut nodes = Vec::new();
        let mut indices = HashMap::new();
        let mut max_depth = 0usize;

        // Recursive insertion keeps root/subtree order.
        fn add_subtree<D: TableDelegate>(
            delegate: &D,
            cx: &App,
            id: &D::RowId,
            parent: Option<usize>,
            depth: usize,
            nodes: &mut Vec<RowNode<D::RowId>>,
            indices: &mut HashMap<D::RowId, usize>,
            max_depth: &mut usize,
        ) {
            if depth > *max_depth {
                *max_depth = depth;
            }

            let idx = nodes.len();
            nodes.push(RowNode { id: id.clone(), parent, children: Vec::new(), depth });
            indices.insert(id.clone(), idx);

            let children = delegate.row_children(id, cx);
            for child in children {
                add_subtree::<D>(
                    delegate,
                    cx,
                    &child,
                    Some(idx),
                    depth + 1,
                    nodes,
                    indices,
                    max_depth,
                );
                let child_idx = *indices.get(&child).expect("child just inserted");
                nodes[idx].children.push(child_idx);
            }
        }

        let roots = delegate.root_row_ids(cx);
        for root in roots.iter() {
            add_subtree::<D>(delegate, cx, root, None, 0, &mut nodes, &mut indices, &mut max_depth);
        }

        let mut registry = Self {
            nodes,
            indices,
            visible_depths_cache: Vec::new(),
            expanded: HashSet::new(),
            max_depth,
        };

        registry.recompute_visible();
        registry
    }

    /// Return the flattened list of visible rows as `(row_id, depth)` in order.
    pub fn visible_rows(&self) -> &[(D::RowId, usize)] {
        &self.visible_depths_cache
    }

    /// Maximum observed tree depth (0 if no children anywhere).
    pub fn max_tree_depth(&self) -> usize {
        self.max_depth
    }

    /// True if this table contains any nested rows.
    pub fn is_tree(&self) -> bool {
        self.max_depth > 0
    }

    /// True if the given row has children.
    pub fn is_collapsible(&self, row_id: &D::RowId) -> bool {
        self.indices.get(row_id).map(|&idx| !self.nodes[idx].children.is_empty()).unwrap_or(false)
    }

    /// True if the row is currently expanded.
    pub fn is_expanded(&self, row_id: &D::RowId) -> bool {
        self.expanded.contains(row_id)
    }

    /// Set expansion state for a row and update the visible cache.
    pub fn set_expanded(&mut self, row_id: D::RowId, expanded: bool) {
        if expanded {
            self.expanded.insert(row_id);
        } else {
            self.expanded.remove(&row_id);
        }
        self.recompute_visible();
    }

    /// Toggle expansion for a row and update the visible cache.
    pub fn toggle_expanded(&mut self, row_id: D::RowId) {
        if self.is_expanded(&row_id) {
            self.set_expanded(row_id, false);
        } else {
            self.set_expanded(row_id, true);
        }
    }

    /// Expand all collapsible rows.
    pub fn expand_all(&mut self) {
        self.expanded = self
            .nodes
            .iter()
            .filter_map(|n| if !n.children.is_empty() { Some(n.id.clone()) } else { None })
            .collect();
        self.recompute_visible();
    }

    /// Collapse all rows.
    pub fn collapse_all(&mut self) {
        self.expanded.clear();
        self.recompute_visible();
    }

    /// Recompute visible cache.
    fn recompute_visible(&mut self) {
        self.visible_depths_cache.clear();

        fn visit<Id: Clone + Eq + Hash>(
            nodes: &Vec<RowNode<Id>>,
            idx: usize,
            expanded: &HashSet<Id>,
            out: &mut Vec<(Id, usize)>,
        ) {
            let node = &nodes[idx];
            out.push((node.id.clone(), node.depth));
            if expanded.contains(&node.id) {
                for &child_idx in &node.children {
                    visit(nodes, child_idx, expanded, out);
                }
            }
        }

        for (i, node) in self.nodes.iter().enumerate() {
            if node.parent.is_none() {
                visit(&self.nodes, i, &self.expanded, &mut self.visible_depths_cache);
            }
        }
    }
}
