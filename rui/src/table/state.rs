use std::collections::{HashMap, HashSet};

use gpui::{App, UniformListScrollHandle, Window, prelude::*};

use crate::table::TableDelegate;

pub struct TableState<D: TableDelegate> {
    delegate: D,

    registry: RowRegistry<D>,

    pub(crate) vertical_scroll_handle: UniformListScrollHandle,
}

impl<D: TableDelegate + 'static> TableState<D> {
    pub fn new(delegate: D, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        let registry = RowRegistry::new(&delegate, cx);

        Self { delegate, registry, vertical_scroll_handle: UniformListScrollHandle::new() }
    }

    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn delegate_mut(&mut self) -> &mut D {
        &mut self.delegate
    }

    pub fn row_registry(&self) -> &RowRegistry<D> {
        &self.registry
    }

    pub fn row_registry_mut(&mut self) -> &mut RowRegistry<D> {
        &mut self.registry
    }
}

pub struct RowRegistry<D: TableDelegate> {
    nodes: Vec<RowNode<D::RowId>>,
    index_by_id: HashMap<D::RowId, usize>,
    visible_cache: Vec<(D::RowId, usize)>,
    expanded_rows: HashSet<D::RowId>,
    max_tree_depth: usize,
}

#[derive(Clone)]
struct RowNode<Id> {
    id: Id,
    parent: Option<usize>,
    children: Vec<usize>,
    depth: usize,
}

impl<D: TableDelegate> RowRegistry<D>
where
    D::RowId: Eq + std::hash::Hash + Clone,
{
    pub fn new(delegate: &D, cx: &App) -> Self {
        let mut nodes: Vec<RowNode<D::RowId>> = Vec::new();
        let mut index_by_id: HashMap<D::RowId, usize> = HashMap::new();
        let mut max_tree_depth: usize = 0;

        fn add_subtree<D: TableDelegate>(
            delegate: &D,
            cx: &App,
            id: &D::RowId,
            parent_idx: Option<usize>,
            depth: usize,
            nodes: &mut Vec<RowNode<D::RowId>>,
            index_by_id: &mut HashMap<D::RowId, usize>,
            max_tree_depth: &mut usize,
        ) where
            D::RowId: Eq + std::hash::Hash + Clone,
        {
            if depth > *max_tree_depth {
                *max_tree_depth = depth;
            }

            let idx = nodes.len();
            nodes.push(RowNode { id: id.clone(), parent: parent_idx, children: Vec::new(), depth });
            index_by_id.insert(id.clone(), idx);

            let child_ids = delegate.row_children(id, cx);
            for child_id in child_ids {
                add_subtree::<D>(
                    delegate,
                    cx,
                    &child_id,
                    Some(idx),
                    depth + 1,
                    nodes,
                    index_by_id,
                    max_tree_depth,
                );
                let child_idx = *index_by_id.get(&child_id).expect("child just inserted");
                nodes[idx].children.push(child_idx);
            }
        }

        let root_ids = delegate.root_row_ids(cx);
        for root_id in root_ids.iter() {
            add_subtree::<D>(
                delegate,
                cx,
                root_id,
                None,
                0,
                &mut nodes,
                &mut index_by_id,
                &mut max_tree_depth,
            );
        }

        let mut registry = Self {
            nodes,
            index_by_id,
            visible_cache: Vec::new(),
            expanded_rows: HashSet::new(),
            max_tree_depth,
        };

        registry.recompute_visible();

        registry
    }

    /// Recompute the visible cache based on current expansion state.
    fn recompute_visible(&mut self) {
        self.visible_cache.clear();

        fn visit<Id: Clone + Eq + std::hash::Hash>(
            nodes: &Vec<RowNode<Id>>,
            idx: usize,
            expanded: &HashSet<Id>,
            visible: &mut Vec<(Id, usize)>,
        ) where
            Id: Clone,
        {
            let node = &nodes[idx];
            visible.push((node.id.clone(), node.depth));
            // If node is expanded, visit children.
            if expanded.contains(&node.id) {
                for &child_idx in &node.children {
                    visit(nodes, child_idx, expanded, visible);
                }
            }
        }

        // Find roots in original order (nodes were pushed in root-subtree order).
        for (i, node) in self.nodes.iter().enumerate() {
            if node.parent.is_none() {
                visit(&self.nodes, i, &self.expanded_rows, &mut self.visible_cache);
            }
        }
    }

    pub fn sorted_visible_row_ids(&self) -> &[(D::RowId, usize)] {
        &self.visible_cache
    }

    pub fn max_tree_depth(&self) -> usize {
        self.max_tree_depth
    }

    pub fn is_tree(&self) -> bool {
        self.max_tree_depth > 0
    }

    pub fn is_row_collapsible(&self, row_id: &D::RowId) -> bool {
        match self.index_by_id.get(row_id) {
            Some(&idx) => !self.nodes[idx].children.is_empty(),
            None => false,
        }
    }

    pub fn row_expanded(&self, row_id: &D::RowId) -> bool {
        self.expanded_rows.contains(row_id)
    }

    pub fn set_row_expanded(&mut self, row_id: D::RowId, expanded: bool) {
        if expanded {
            self.expanded_rows.insert(row_id);
        } else {
            self.expanded_rows.remove(&row_id);
        }
        self.recompute_visible();
    }
}
