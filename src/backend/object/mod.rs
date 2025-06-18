mod executor;
mod fixture_group;
mod preset;
mod sequence;

pub use executor::*;
pub use fixture_group::*;
pub use preset::*;
pub use sequence::*;

#[macro_export]
macro_rules! define_object_id {
    ($name:ident) => {
        #[doc = concat!("A unique identifier for a ", stringify!($name), " object")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        #[derive(
            derive_more::From,
            derive_more::Into,
            derive_more::Add,
            derive_more::Sub,
            derive_more::AddAssign,
            derive_more::SubAssign,
            derive_more::MulAssign,
            derive_more::DivAssign
        )]
        pub struct $name(pub u32);
    };
}

/// Any object.
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Executor(Executor),
    Sequence(Sequence),
    FixtureGroup(FixtureGroup),
    Preset(AnyPreset),
}
