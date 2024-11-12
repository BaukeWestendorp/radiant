use flow_gpui::NodeCategory;
use flow_gpui_macros::NodeCategory;

#[derive(Debug, Clone, Copy, PartialEq, NodeCategory)]
pub enum Category {
    Math,
    Output,
    #[node_category(name = "Custom Name")]
    CategoryWithCustomName,
}

#[test]
fn name() {
    assert_eq!(Category::Math.to_string(), "Math");
    assert_eq!(Category::Output.to_string(), "Output");
    assert_eq!(Category::CategoryWithCustomName.to_string(), "Custom Name");
}

#[test]
fn all() {
    let all = Category::all().collect::<Vec<_>>();
    assert_eq!(
        all,
        vec![
            Category::Math,
            Category::Output,
            Category::CategoryWithCustomName
        ]
    );
}
