macro_rules! define_preset {
    ($($name:ident, $id:ident, $new_name:literal, $any_name:ident),+ $(,)?) => {
        $(
            crate::define_object_id!($id);

            #[doc = concat!("A ", stringify!($name), " preset")]
            #[derive(Debug, Clone, PartialEq)]
            pub struct $name {
                pub id: $id,
                pub name: String,
            }

            impl $name {
                pub fn new(id: impl Into<$id>) -> Self {
                    Self { id: id.into(), name: $new_name.to_string() }
                }

                pub fn with_name(mut self, name: impl Into<String>) -> Self {
                    self.name = name.into();
                    self
                }
            }
        )+

        /// Any preset id.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[derive(derive_more::From)]
        pub enum AnyPresetId {
            $(
                $any_name($id),
            )+
        }

        /// Any preset.
        #[derive(Debug, Clone, PartialEq)]
        #[derive(derive_more::From)]
        pub enum AnyPreset {
            $(
                $any_name($name),
            )+
        }
    };
}

define_preset!(DimmerPreset, DimmerPresetId, "New Dimmer Preset", Dimmer);
