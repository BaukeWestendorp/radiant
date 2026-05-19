use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use gpui::{
    App, Bounds, Context, Entity, FocusHandle, Focusable, Pixels, UniformListScrollHandle, Window,
};

use crate::table::TableDelegate;

pub struct TableState<D: TableDelegate> {
    delegate: D,

    pub(crate) rows: RowRegistry<D>,

    selection: Entity<Vec<D::RowId>>,
    pub(crate) selected_column_ix: usize,
    pub(crate) is_selecting: bool,
    pub(crate) is_subtracting: bool,
    pub(crate) range_selection_anchor: Option<D::RowId>,
    pub(crate) range_selection_head: Option<D::RowId>,

    pub(crate) focus_handle: FocusHandle,
    pub(crate) vertical_scroll_handle: UniformListScrollHandle,
    pub(crate) bounds: Bounds<Pixels>,
    pub(crate) column_widths: Vec<Pixels>,
}

impl<D: TableDelegate + 'static> TableState<D> {
    pub fn new(
        delegate: D,
        selection: Entity<Vec<D::RowId>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let rows = RowRegistry::from_delegate(&delegate, cx);

        let col_count = delegate.column_count(cx);
        let mut column_widths = Vec::new();
        for col_ix in 0..col_count {
            let col = delegate.column(col_ix, cx);
            column_widths.push(col.min_width());
        }

        cx.observe(&selection, |this, selection, cx| {
            // When selection changes externally, ensure any selected rows that are nested
            // are visible by expanding their ancestor path(s).
            let selected_ids = selection.read(cx).clone();
            for row_id in selected_ids.iter() {
                this.rows.expand_path_to(row_id);
            }
            cx.notify();
        })
        .detach();

        Self {
            delegate,
            rows,

            selection,
            selected_column_ix: 0,
            is_selecting: false,
            is_subtracting: false,
            range_selection_anchor: None,
            range_selection_head: None,

            focus_handle: cx.focus_handle(),
            vertical_scroll_handle: UniformListScrollHandle::new(),
            bounds: Bounds::default(),
            column_widths,
        }
    }

    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn is_tree(&self) -> bool {
        self.rows.is_tree()
    }

    pub fn visible_rows(&self, cx: &App) -> Vec<(D::RowId, usize)> {
        let _ = cx;
        self.rows.visible_rows().to_vec()
    }

    pub fn is_collapsible(&self, row_id: &D::RowId) -> bool {
        self.rows.is_collapsible(row_id)
    }

    pub fn selection(&self) -> Entity<Vec<D::RowId>> {
        self.selection.clone()
    }

    pub fn selected_row_ids(&self, cx: &App) -> Vec<D::RowId> {
        self.selection.read(cx).clone()
    }

    pub fn selection_contains(&self, row_id: &D::RowId, cx: &App) -> bool {
        self.selection.read(cx).contains(row_id)
    }

    pub fn set_selection(&mut self, row_ids: Vec<D::RowId>, cx: &mut Context<Self>) {
        self.selection.update(cx, move |selection, cx| {
            *selection = row_ids;
            cx.notify();
        });
        cx.notify();
    }

    pub fn select_all(&mut self, cx: &mut Context<Self>) {
        let all_visible_ids: Vec<D::RowId> =
            self.rows.visible_rows().iter().map(|(id, _depth)| id.clone()).collect();

        self.range_selection_anchor = all_visible_ids.first().cloned();
        self.range_selection_head = all_visible_ids.last().cloned();
        self.is_subtracting = false;

        self.set_selection(all_visible_ids, cx);
        cx.notify();
    }

    pub fn clear_selection(&mut self, cx: &mut Context<Self>) {
        self.selection.update(cx, |selection, cx| {
            selection.clear();
            cx.notify();
        });
        cx.notify();
    }

    pub fn toggle_selected(&mut self, row_id: &D::RowId, cx: &mut Context<Self>) {
        let row_id = row_id.clone();
        self.selection.update(cx, move |selection, _| {
            if let Some(ix) = selection.iter().position(|id| id == &row_id) {
                selection.remove(ix);
            } else {
                selection.push(row_id.clone());
            }
        });
        cx.notify();
    }

    pub(crate) fn move_selection_by(&mut self, delta: isize, extend: bool, cx: &mut Context<Self>) {
        let visible = self.rows.visible_rows();
        if visible.is_empty() {
            return;
        }

        let current = if extend {
            self.range_selection_head.clone().or(self.selection.read(cx).last().cloned())
        } else {
            self.selection.read(cx).last().cloned()
        };

        // Empty selection behavior:
        // - Down selects the first row
        // - Up selects the last row
        let current_ix = current
            .as_ref()
            .and_then(|id| self.rows.visible_ix_from_id(id))
            .map(|ix| ix as isize)
            .unwrap_or(if delta < 0 { visible.len().saturating_sub(1) as isize + 1 } else { -1 });

        let mut target_ix = current_ix + delta;
        let max_ix = visible.len().saturating_sub(1) as isize;
        if target_ix < 0 {
            target_ix = 0;
        }
        if target_ix > max_ix {
            target_ix = max_ix;
        }

        let target_id = visible[target_ix as usize].0.clone();

        if !extend {
            self.range_selection_anchor = Some(target_id.clone());
            self.range_selection_head = Some(target_id.clone());
            self.set_selection(vec![target_id], cx);
            return;
        }

        if self.range_selection_anchor.is_none() {
            if let Some(head) = self.range_selection_head.clone().or(current) {
                self.range_selection_anchor = Some(head);
            } else {
                self.range_selection_anchor = Some(target_id.clone());
            }
        }

        self.range_selection_head = Some(target_id);

        self.is_subtracting = false;
        self.recompute_range_selection(false, cx);
    }

    pub(crate) fn edit_selection(&mut self, cx: &mut Context<Self>) {
        let row_ids = self.selection.read(cx).clone();
        if row_ids.is_empty() {
            return;
        }

        self.delegate.edit_rows(&row_ids, cx);
        cx.notify();
    }

    pub(crate) fn delete_selection(&mut self, cx: &mut Context<Self>) {
        let row_ids = self.selection.read(cx).clone();
        if row_ids.is_empty() {
            return;
        }

        self.delegate.delete_rows(&row_ids, cx);
        self.clear_selection(cx);
        cx.notify();
    }

    pub(crate) fn toggle_expand_selected_rows(&mut self, cx: &mut Context<Self>) {
        let selected = self.selection.read(cx).clone();
        if selected.is_empty() {
            return;
        }

        let collapsible: Vec<D::RowId> =
            selected.into_iter().filter(|row_id| self.is_collapsible(row_id)).collect();

        if collapsible.is_empty() {
            return;
        }

        let all_expanded = collapsible.iter().all(|row_id| self.is_expanded(row_id));

        for row_id in collapsible {
            self.rows.set_expanded(row_id, !all_expanded);
        }

        cx.notify();
    }

    pub fn selected_column_ix(&self) -> usize {
        self.selected_column_ix
    }

    pub fn set_selected_column_ix(&mut self, ix: usize, cx: &mut Context<Self>) {
        self.selected_column_ix = ix;
        cx.notify();
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

    pub(crate) fn reset_column_widths(&mut self, cx: &mut Context<Self>) {
        let col_count = self.delegate.column_count(cx);
        self.column_widths.clear();
        if col_count == 0 {
            return;
        }
        let mut width_left_over = self.bounds.size.width;
        for col_ix in 0..col_count - 1 {
            let col = self.delegate.column(col_ix, cx);
            self.column_widths.push(col.min_width());
            width_left_over -= col.min_width();
        }
        let last_col_min_width = self.delegate.column(col_count - 1, cx).min_width();
        self.column_widths.push(last_col_min_width.max(width_left_over));
    }

    pub(crate) fn range_selection(&mut self) -> Vec<D::RowId> {
        let (Some(anchor), Some(head)) =
            (self.range_selection_anchor.clone(), self.range_selection_head.clone())
        else {
            return Vec::new();
        };

        let (Some(anchor_ix), Some(head_ix)) =
            (self.rows.visible_ix_from_id(&anchor), self.rows.visible_ix_from_id(&head))
        else {
            return Vec::new();
        };

        let (start, end) = (anchor_ix.min(head_ix), anchor_ix.max(head_ix));

        self.rows
            .visible_rows()
            .iter()
            .skip(start)
            .take(end - start + 1)
            .map(|(id, _)| id.clone())
            .collect()
    }

    fn end_range_selection(&mut self, cx: &mut Context<Self>) {
        self.is_selecting = false;
        self.range_selection_anchor = None;
        self.range_selection_head = None;
        self.is_subtracting = false;
        cx.notify();
    }

    fn recompute_range_selection(&mut self, expand_selection: bool, cx: &mut Context<Self>) {
        let new_range = self.range_selection();

        if !expand_selection {
            self.set_selection(new_range, cx);
            return;
        }

        let previous_selection = self.selection.read(cx).clone();

        let next_selection = if self.is_subtracting {
            let mut next = previous_selection;
            next.retain(|id| !new_range.contains(id));
            next
        } else {
            let mut merged = previous_selection;
            for id in new_range {
                if !merged.contains(&id) {
                    merged.push(id);
                }
            }
            merged
        };

        self.set_selection(next_selection, cx);
    }

    pub(crate) fn on_cell_mouse_down(
        &mut self,
        row_id: D::RowId,
        col_ix: usize,
        expand_selection: bool,
        cx: &mut Context<Self>,
    ) {
        self.selected_column_ix = col_ix;

        self.is_selecting = true;
        self.range_selection_anchor = Some(row_id.clone());
        self.range_selection_head = Some(row_id.clone());

        // With secondary modifier dragging, if it starts on an already-selected cell,
        // subtract the range from the existing selection instead of adding it.
        self.is_subtracting = expand_selection && self.selection_contains(&row_id, cx);

        self.recompute_range_selection(expand_selection, cx);
        cx.notify();
    }

    pub(crate) fn on_cell_mouse_move(
        &mut self,
        row_id: D::RowId,
        expand_selection: bool,
        cx: &mut Context<Self>,
    ) {
        if !self.is_selecting {
            return;
        }

        if self.range_selection_anchor.is_none() {
            if let Some(head) = self.range_selection_head.clone() {
                self.range_selection_anchor = Some(head);
            } else {
                self.range_selection_anchor = Some(row_id.clone());
            }
        }

        if self.range_selection_head != Some(row_id.clone()) {
            self.range_selection_head = Some(row_id);

            self.recompute_range_selection(expand_selection, cx);
            cx.notify();
        }
    }

    pub(crate) fn on_cell_mouse_up(&mut self, cx: &mut Context<Self>) {
        self.end_range_selection(cx);
    }

    pub(crate) fn on_cell_mouse_up_out(&mut self, cx: &mut Context<Self>) {
        self.end_range_selection(cx);
    }
}

impl<D: TableDelegate + 'static> Focusable for TableState<D> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

/// Registry that maintains a flattened view of the tree of rows along with
/// expansion state and quick lookup from id -> index.
pub(crate) struct RowRegistry<D: TableDelegate> {
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
        self.recompute_visible();
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
