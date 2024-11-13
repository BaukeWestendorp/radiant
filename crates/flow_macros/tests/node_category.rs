#![cfg(all(feature = "gpui", not(feature = "serde")))]

#[derive(Debug, Clone, Copy, PartialEq, flow::gpui::NodeCategory)]
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
    use flow::gpui::NodeCategory as _;

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
