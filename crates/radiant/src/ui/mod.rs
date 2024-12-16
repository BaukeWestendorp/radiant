pub mod group_selector;

pub enum AssetSelectorEvent<Id> {
    Change(Id),
}
