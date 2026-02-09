use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use gpui::{
    App, Bounds, Context, FocusHandle, Focusable, Pixels, UniformListScrollHandle, Window, px,
};

use crate::table::TableDelegate;

pub struct TableState<D: TableDelegate> {
    delegate: D,

    pub(crate) rows: RowRegistry<D>,
    pub(crate) selection: TableSelection,

    pub(crate) focus_handle: FocusHandle,
    pub(crate) vertical_scroll_handle: UniformListScrollHandle,
    pub(crate) bounds: Bounds<Pixels>,
    pub(crate) column_widths: Vec<Pixels>,
}

impl<D: TableDelegate + 'static> TableState<D> {
    pub fn new(delegate: D, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let rows = RowRegistry::from_delegate(&delegate, cx);
        let selection = TableSelection::new();

        let col_count = delegate.column_count(cx);
        let mut column_widths = Vec::new();
        for col_ix in 0..col_count {
            let col = delegate.column(col_ix, cx);
            column_widths.push(col.min_width());
        }

        cx.on_next_frame(window, |this, _, cx| {
            this.reset_column_widths(cx);
        });

        Self {
            delegate,
            rows,
            selection,

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

    pub fn select_cell(&mut self, col_ix: usize, row_id: &D::RowId, cx: &mut Context<Self>) {
        self.expand_parents(row_id, cx);
        if let Some(row_ix) = self.rows.visible_ix_from_id(row_id) {
            self.selection.select_cell(col_ix, row_ix);
        }
    }

    pub fn select_all(&mut self, cx: &mut Context<Self>) {
        let total = self.rows.visible_rows().len();
        if total == 0 {
            return;
        }
        self.selection.start(self.selection.selected_column_ix, 0);
        self.selection.extend_to(total - 1);
        self.selection.finish();
        cx.notify();
    }

    pub fn clear_selection(&mut self, cx: &mut Context<Self>) {
        self.selection.clear();
        cx.notify();
    }

    pub fn selected_column(&self) -> usize {
        self.selection.selected_column_ix
    }

    pub fn selection_contains(&self, row_id: &D::RowId) -> bool {
        match self.rows.visible_rows().iter().position(|(id, _)| id == row_id) {
            Some(ix) => self.selection.contains(ix),
            None => false,
        }
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
        let visible = self.rows.visible_rows();
        let selected_ixs = self.selection.selected_indices();
        if selected_ixs.is_empty() {
            return;
        }
        let row_ids: Vec<_> = selected_ixs
            .into_iter()
            .filter_map(|i| visible.get(i).map(|(id, _)| id.clone()))
            .collect();

        if !row_ids.is_empty() {
            self.delegate_mut().edit_rows(&row_ids, cx);
            cx.notify();
        }
    }

    pub fn delete_selection(&mut self, cx: &mut Context<Self>) {
        let visible = self.rows.visible_rows();
        let selected_ixs = self.selection.selected_indices();
        if selected_ixs.is_empty() {
            return;
        }
        let row_ids: Vec<_> = selected_ixs
            .into_iter()
            .filter_map(|i| visible.get(i).map(|(id, _)| id.clone()))
            .collect();

        if !row_ids.is_empty() {
            self.delegate_mut().delete_rows(&row_ids, cx);

            self.selection.clear();
            self.rows.recompute_visible();
            cx.notify();
        }
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

#[derive(Clone, Debug)]
pub struct TableSelection {
    pub(crate) anchor: Option<usize>,
    pub(crate) head: Option<usize>,

    groups: Vec<(usize, usize)>,

    selected_lookup: HashSet<usize>,

    pub selected_column_ix: usize,
    pub is_selecting: bool,
}

impl TableSelection {
    pub fn new() -> Self {
        Self {
            anchor: None,
            head: None,
            groups: Vec::new(),
            selected_lookup: HashSet::new(),
            selected_column_ix: 0,
            is_selecting: false,
        }
    }

    pub fn clear(&mut self) {
        self.anchor = None;
        self.head = None;
        self.groups.clear();
        self.selected_lookup.clear();
        self.selected_column_ix = 0;
        self.is_selecting = false;
    }

    pub fn start(&mut self, col_ix: usize, row_ix: usize) {
        self.anchor = Some(row_ix);
        self.head = Some(row_ix);
        self.selected_column_ix = col_ix;
        self.is_selecting = true;
    }

    pub fn extend_to(&mut self, row_ix: usize) {
        if self.anchor.is_none() {
            self.anchor = Some(row_ix);
        }
        self.head = Some(row_ix);
    }

    pub fn finish(&mut self) {
        if let (Some(a), Some(h)) = (self.anchor, self.head) {
            let (s, e) = if a <= h { (a, h) } else { (h, a) };
            self.groups.push((s, e));
            for i in s..=e {
                self.selected_lookup.insert(i);
            }
        }
        self.anchor = None;
        self.head = None;
        self.is_selecting = false;
    }

    pub fn range(&self) -> Option<(usize, usize)> {
        match (self.anchor, self.head) {
            (Some(a), Some(h)) => {
                if a <= h {
                    Some((a, h))
                } else {
                    Some((h, a))
                }
            }
            _ => self.groups.last().cloned(),
        }
    }

    pub fn contains(&self, row_ix: usize) -> bool {
        if self.selected_lookup.contains(&row_ix) {
            return true;
        }
        if let (Some(a), Some(h)) = (self.anchor, self.head) {
            let (s, e) = if a <= h { (a, h) } else { (h, a) };
            return (s..=e).contains(&row_ix);
        }
        false
    }

    pub fn selected_indices(&self) -> Vec<usize> {
        let mut vec: Vec<usize> = self.selected_lookup.iter().cloned().collect();
        if let (Some(a), Some(h)) = (self.anchor, self.head) {
            let (s, e) = if a <= h { (a, h) } else { (h, a) };
            for i in s..=e {
                vec.push(i);
            }
        }
        vec.sort_unstable();
        vec.dedup();
        vec
    }

    pub fn current_head_or_last(&self) -> Option<usize> {
        self.head.or_else(|| self.groups.last().map(|&(_, e)| e))
    }

    pub fn select_cell(&mut self, col_ix: usize, row_ix: usize) {
        self.start(col_ix, row_ix);
        self.finish();
    }

    pub fn select_column(&mut self, col_ix: usize) {
        self.selected_column_ix = col_ix;
    }
}
