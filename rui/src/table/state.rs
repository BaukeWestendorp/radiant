use std::collections::{HashMap, HashSet};

use gpui::{
    App, Bounds, Pixels, UniformListScrollHandle, Window, bounds, point, prelude::*, px, size,
};

use crate::table::TableDelegate;

pub struct TableState<D: TableDelegate> {
    delegate: D,

    registry: RowRegistry<D>,

    pub(crate) table_bounds: Option<Bounds<Pixels>>,
    pub(crate) column_bounds: Vec<Bounds<Pixels>>,

    pub(crate) vertical_scroll_handle: UniformListScrollHandle,
}

impl<D: TableDelegate + 'static> TableState<D> {
    pub fn new(delegate: D, window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Arrange to divide columns equally on the next frame once we know the table bounds.
        cx.on_next_frame(window, |this, _, cx| {
            this.divide_columns_equally(cx);
            cx.notify();
        });

        let mut column_bounds = Vec::new();
        let mut x = px(0.0);
        let default_height = px(0.0);
        for col_ix in 0..delegate.column_count(cx) {
            let initial_width = delegate.column(col_ix, cx).min_width();
            let col_bounds = bounds(point(x, px(0.0)), size(initial_width, default_height));
            column_bounds.push(col_bounds);
            x += initial_width;
        }

        let registry = RowRegistry::new(&delegate, cx);

        Self {
            delegate,

            registry,

            table_bounds: None,
            column_bounds,

            vertical_scroll_handle: UniformListScrollHandle::new(),
        }
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

    pub fn divide_columns_equally(&mut self, cx: &App) {
        let column_count = self.delegate.column_count(cx);

        if let Some(table_bounds) = self.table_bounds {
            let total_width = table_bounds.size.width;
            let col_width = total_width / column_count as f32;
            let mut x = table_bounds.origin.x;
            for col_ix in 0..column_count {
                let col_bounds = bounds(
                    point(x, table_bounds.origin.y),
                    size(col_width, table_bounds.size.height),
                );
                if col_ix < self.column_bounds.len() {
                    self.column_bounds[col_ix] = col_bounds;
                } else {
                    self.column_bounds.push(col_bounds);
                }
                x += col_width;
            }
        }
    }

    // FIXME: This function is so cursed. Oef.
    pub fn resize_col(&mut self, col_ix: usize, width: Pixels, cx: &mut Context<Self>) {
        if col_ix >= self.column_bounds.len() {
            return;
        }

        let table_bounds = match self.table_bounds {
            Some(tb) => tb,
            None => return,
        };
        let total_width = table_bounds.size.width;

        let column_count = self.column_bounds.len();
        let mut current_widths: Vec<Pixels> =
            self.column_bounds.iter().map(|b| b.size.width).collect();
        let min_widths: Vec<Pixels> =
            (0..column_count).map(|i| self.delegate.column(i, cx).min_width()).collect();

        let old_width = current_widths[col_ix];
        if width == old_width {
            return;
        }

        let sum_min_others: Pixels = min_widths
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != col_ix)
            .map(|(_, &m)| m)
            .fold(px(0.0), |acc, v| acc + v);

        let min_i = min_widths[col_ix];
        let max_i = if total_width > sum_min_others { total_width - sum_min_others } else { min_i };

        let mut new_width = width;
        if new_width < min_i {
            new_width = min_i;
        } else if new_width > max_i {
            new_width = max_i;
        }
        current_widths[col_ix] = new_width;

        let remaining_width = total_width - new_width;
        let sum_curr_others: Pixels = current_widths
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != col_ix)
            .map(|(_, &w)| w)
            .fold(px(0.0), |acc, v| acc + v);

        let difference = remaining_width - sum_curr_others;

        if difference < px(0.0) {
            let mut need_reduce = -difference;
            for i in (0..column_count).rev() {
                if i == col_ix {
                    continue;
                }
                let reducible = current_widths[i] - min_widths[i];
                if reducible <= px(0.0) {
                    continue;
                }
                let reduce = if reducible >= need_reduce { need_reduce } else { reducible };
                current_widths[i] = current_widths[i] - reduce;
                need_reduce = need_reduce - reduce;
                if need_reduce <= px(0.0) {
                    break;
                }
            }
        } else if difference > px(0.0) {
            if column_count > 1 {
                if let Some(ti) = (0..column_count).rev().find(|&i| i != col_ix) {
                    current_widths[ti] = current_widths[ti] + difference;
                }
            }
        }

        let mut x = table_bounds.origin.x;
        for i in 0..column_count {
            let h = self.column_bounds[i].size.height;
            self.column_bounds[i] =
                bounds(point(x, table_bounds.origin.y), size(current_widths[i], h));
            x = x + current_widths[i];
        }

        self.update_column_widths(cx);
    }

    // FIXME: This function is so cursed. Oef.
    pub(crate) fn update_column_widths(&mut self, cx: &mut Context<Self>) {
        let Some(table_bounds) = self.table_bounds else {
            return;
        };

        let column_count = self.delegate.column_count(cx);
        if column_count == 0 {
            cx.notify();
            return;
        }

        if self.column_bounds.len() < column_count {
            let mut x = if self.column_bounds.is_empty() {
                table_bounds.origin.x
            } else {
                let mut last_x = table_bounds.origin.x;
                for b in &self.column_bounds {
                    last_x = last_x + b.size.width;
                }
                last_x
            };
            for i in self.column_bounds.len()..column_count {
                let min_w = self.delegate.column(i, cx).min_width();
                let b =
                    bounds(point(x, table_bounds.origin.y), size(min_w, table_bounds.size.height));
                self.column_bounds.push(b);
                x = x + min_w;
            }
        } else if self.column_bounds.len() > column_count {
            self.column_bounds.truncate(column_count);
        }

        for i in 0..self.column_bounds.len() {
            let w = self.column_bounds[i].size.width;
            self.column_bounds[i] = bounds(
                point(self.column_bounds[i].origin.x, table_bounds.origin.y),
                size(w, table_bounds.size.height),
            );
        }

        let accumulated: Pixels =
            self.column_bounds.iter().map(|b| b.size.width).fold(px(0.0), |a, b| a + b);

        if accumulated != table_bounds.size.width {
            let diff = table_bounds.size.width - accumulated;
            let last_ix = self.column_bounds.len() - 1;
            let last_min = self.delegate.column(last_ix, cx).min_width();
            let last_new = self.column_bounds[last_ix].size.width + diff;
            let final_last_width = if last_new < last_min { last_min } else { last_new };
            self.column_bounds[last_ix] = bounds(
                point(self.column_bounds[last_ix].origin.x, self.column_bounds[last_ix].origin.y),
                size(final_last_width, self.column_bounds[last_ix].size.height),
            );
        }

        let mut x = table_bounds.origin.x;
        for i in 0..self.column_bounds.len() {
            let w = self.column_bounds[i].size.width;
            self.column_bounds[i] =
                bounds(point(x, table_bounds.origin.y), size(w, self.column_bounds[i].size.height));
            x = x + w;
        }

        cx.notify();
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
