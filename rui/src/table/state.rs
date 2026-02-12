use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use gpui::{
    App, Bounds, Context, Entity, FocusHandle, Focusable, Pixels, UniformListScrollHandle, Window,
    px,
};

use crate::table::TableDelegate;

pub struct TableState<D: TableDelegate> {
    delegate: D,

    pub(crate) rows: RowRegistry<D>,

    pub selected_rows: Entity<Vec<D::RowId>>,
    pub(crate) selected_column: usize,

    pub(crate) focus_handle: FocusHandle,
    pub(crate) vertical_scroll_handle: UniformListScrollHandle,
    pub(crate) bounds: Bounds<Pixels>,
    pub(crate) column_widths: Vec<Pixels>,
}

impl<D: TableDelegate + 'static> TableState<D> {
    pub fn new(
        delegate: D,
        selection: Entity<Vec<D::RowId>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let rows = RowRegistry::from_delegate(&delegate, cx);

        let col_count = delegate.column_count(cx);
        let mut column_widths = Vec::new();
        for col_ix in 0..col_count {
            let col = delegate.column(col_ix, cx);
            column_widths.push(col.min_width());
        }

        cx.on_next_frame(window, |this, _, cx| {
            this.reset_column_widths(cx);
        });

        cx.observe(&selection, |_, _, _| {
            eprintln!("TODO: Handle changed selection");
        })
        .detach();

        Self {
            delegate,
            rows,

            selected_rows: selection,
            selected_column: 0,

            focus_handle: cx.focus_handle(),
            vertical_scroll_handle: UniformListScrollHandle::new(),
            bounds: Bounds::default(),
            column_widths,
        }
    }

    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn delegate_mut(&mut self) -> &mut D {
        &mut self.delegate
    }

    pub fn is_collapsible(&self, row_id: &D::RowId) -> bool {
        self.rows.is_collapsible(row_id)
    }

    pub fn is_expanded(&self, row_id: &D::RowId) -> bool {
        self.rows.is_expanded(row_id)
    }

    pub fn set_expanded(&mut self, row_id: D::RowId, expanded: bool, cx: &mut Context<Self>) {
        self.rows.set_expanded(row_id, expanded);
        cx.notify();
    }

    pub fn toggle_expanded(&mut self, row_id: D::RowId, cx: &mut Context<Self>) {
        self.rows.toggle_expanded(row_id);
        cx.notify();
    }

    pub fn expand_parents(&mut self, row_id: &D::RowId, cx: &mut Context<Self>) {
        self.rows.expand_parents(row_id);
        cx.notify();
    }

    pub fn expand_all(&mut self, cx: &mut Context<Self>) {
        self.rows.expand_all();
        cx.notify();
    }

    pub fn collapse_all(&mut self, cx: &mut Context<Self>) {
        self.rows.collapse_all();
        cx.notify();
    }

    pub fn edit_selection(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let row_ids = self.selected_rows.read(cx).clone();
        if row_ids.is_empty() {
            return;
        }

        self.delegate_mut().edit_rows(&row_ids, cx);
        cx.notify();
    }

    pub fn delete_selection(&mut self, cx: &mut Context<Self>) {
        let row_ids = self.selected_rows.read(cx).clone();
        if row_ids.is_empty() {
            return;
        }

        self.delegate_mut().delete_rows(&row_ids, cx);

        self.selected_rows.update(cx, |rows, cx| {
            rows.clear();
            cx.notify();
        });

        self.rows.recompute_visible();
        cx.notify();
    }

    pub fn reset_column_widths(&mut self, cx: &mut Context<Self>) {
        let col_count = self.delegate.column_count(cx);
        self.column_widths.clear();
        let mut taken_width = px(0.0);
        for col_ix in 0..col_count - 1 {
            let col = self.delegate.column(col_ix, cx);
            self.column_widths.push(col.min_width());
            taken_width += col.min_width();
        }
        self.column_widths.push(self.bounds.size.width - taken_width);
    }
}

impl<D: TableDelegate + 'static> Focusable for TableState<D> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
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
            if indices.contains_key(id) {
                return;
            }

            if depth > *max_depth {
                *max_depth = depth;
            }

            let ix = nodes.len();
            nodes.push(RowNode { id: id.clone(), parent, children: Vec::new(), depth });
            indices.insert(id.clone(), ix);

            let children = delegate.row_children(id, cx);
            for child in children {
                if indices.contains_key(&child) {
                    continue;
                }

                add_subtree::<D>(
                    delegate,
                    cx,
                    &child,
                    Some(ix),
                    depth + 1,
                    nodes,
                    indices,
                    max_depth,
                );
                let child_ix = *indices.get(&child).expect("child just inserted");
                nodes[ix].children.push(child_ix);
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

    pub fn visible_rows(&self) -> &[(D::RowId, usize)] {
        &self.visible_depths_cache
    }

    pub fn expand_path_to(&mut self, row_id: &D::RowId) {
        let Some(mut ix) = self.indices.get(row_id).copied() else { return };

        while let Some(parent_ix) = self.nodes.get(ix).and_then(|n| n.parent) {
            if let Some(parent_id) = self.nodes.get(parent_ix).map(|n| n.id.clone()) {
                self.expanded.insert(parent_id);
            }
            ix = parent_ix;
        }
    }

    pub fn is_tree(&self) -> bool {
        self.max_depth > 0
    }

    pub fn is_collapsible(&self, row_id: &D::RowId) -> bool {
        self.indices.get(row_id).map(|&ix| !self.nodes[ix].children.is_empty()).unwrap_or(false)
    }

    pub fn is_expanded(&self, row_id: &D::RowId) -> bool {
        self.expanded.contains(row_id)
    }

    pub fn set_expanded(&mut self, row_id: D::RowId, expanded: bool) {
        if expanded {
            self.expanded.insert(row_id);
        } else {
            self.expanded.remove(&row_id);
        }
        self.recompute_visible();
    }

    pub fn toggle_expanded(&mut self, row_id: D::RowId) {
        if self.is_expanded(&row_id) {
            self.set_expanded(row_id, false);
        } else {
            self.set_expanded(row_id, true);
        }
    }

    pub fn expand_parents(&mut self, row_id: &D::RowId) {
        let Some(node) = self.node(row_id) else { return };
        let Some(parent_ix) = node.parent.as_ref() else { return };
        let Some(parent_id) = self.nodes.get(*parent_ix).map(|n| n.id.clone()) else {
            return;
        };
        self.set_expanded(parent_id.clone(), true);
        self.expand_parents(&parent_id);
    }

    pub fn expand_all(&mut self) {
        self.expanded = self
            .nodes
            .iter()
            .filter_map(|n| if !n.children.is_empty() { Some(n.id.clone()) } else { None })
            .collect();
        self.recompute_visible();
    }

    pub fn collapse_all(&mut self) {
        self.expanded.clear();
        self.recompute_visible();
    }

    fn node(&self, row_id: &D::RowId) -> Option<&RowNode<D::RowId>> {
        let ix = self.indices.get(row_id)?;
        self.nodes.get(*ix)
    }

    fn visible_ix_from_id(&self, row_id: &D::RowId) -> Option<usize> {
        self.visible_depths_cache.iter().position(|(id, _)| id == row_id)
    }

    fn recompute_visible(&mut self) {
        self.visible_depths_cache.clear();

        fn visit<Id: Clone + Eq + Hash>(
            nodes: &Vec<RowNode<Id>>,
            ix: usize,
            expanded: &HashSet<Id>,
            out: &mut Vec<(Id, usize)>,
        ) {
            let node = &nodes[ix];
            out.push((node.id.clone(), node.depth));
            if expanded.contains(&node.id) {
                for &child_ix in &node.children {
                    visit(nodes, child_ix, expanded, out);
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
