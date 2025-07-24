use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;

use crate::show::preset::{
    Beam, Color, Control, Dimmer, Focus, Gobo, Position, PresetContent, Shapers, Video,
};

pub trait Object: Debug + Clone
where
    Self: Sized,
{
    fn id(&self) -> ObjectId<Self>;

    fn name(&self) -> &str;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct ObjectId<T>(u32, PhantomData<T>);

impl<T> ObjectId<T> {
    pub fn new(id: u32) -> Self {
        Self(id, PhantomData::default())
    }
}

impl<T> Clone for ObjectId<T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData::default())
    }
}

impl<T> Copy for ObjectId<T> {}

impl<T> From<ObjectId<T>> for u32 {
    fn from(id: ObjectId<T>) -> Self {
        id.0
    }
}

impl<T> From<u32> for ObjectId<T> {
    fn from(id: u32) -> Self {
        Self::new(id)
    }
}

impl<T> Deref for ObjectId<T> {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[macro_export]
macro_rules! define_objects {
    (
        $(
            $obj_vis:vis struct $obj:ident {
                $( $field_vis:vis $field:ident : $field_ty:ty, )*
            }
        )*
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[derive(serde::Deserialize)]
        pub enum AnyObjectId {
            $(
                $obj(u32),
            )*
        }

        #[derive(Debug, Clone)]
        #[derive(serde::Deserialize)]
        pub enum AnyObject {
            $(
                $obj($obj),
            )*
        }

        impl AnyObject {
            pub fn id(&self) -> AnyObjectId {
                match self {
                    $(Self::$obj(obj) => obj.id().into(),)*
                }
            }

            pub fn name(&self) -> &str {
                match self {
                    $(Self::$obj(obj) => obj.name.as_str(),)*
                }
            }
        }

        $(
            #[derive(Debug, Clone)]
            #[derive(serde::Deserialize)]
            $obj_vis struct $obj {
                pub(crate) id: ObjectId<$obj>,
                pub(crate) name: String,

                $( $field_vis $field : $field_ty, )*
            }

            impl Object for $obj {
                fn id(&self) -> ObjectId<Self> {
                    self.id
                }

                fn name(&self) -> &str {
                    &self.name
                }
            }

            impl TryFrom<AnyObjectId> for ObjectId<$obj> {
                type Error = crate::error::Error;

                fn try_from(id: AnyObjectId) -> Result<Self, Self::Error> {
                    match id {
                        AnyObjectId::$obj(id) => Ok(ObjectId::<$obj>::new(id)),
                        _ => eyre::bail!("failed to convert id"),
                    }
                }
            }

            impl From<ObjectId<$obj>> for AnyObjectId {
                fn from(id: ObjectId<$obj>) -> Self {
                    AnyObjectId::$obj(id.into())
                }
            }

            impl TryFrom<AnyObject> for $obj {
                type Error = crate::error::Error;

                fn try_from(object: AnyObject) -> Result<Self, Self::Error> {
                    match object {
                        AnyObject::$obj(obj) => Ok(obj),
                        _ => eyre::bail!("failed to convert object"),
                    }
                }
            }

            impl From<$obj> for AnyObject {
                fn from(object: $obj) -> Self {
                    Self::$obj(object)
                }
            }

            impl<'a> TryFrom<&'a AnyObject> for &'a $obj {
                type Error = crate::error::Error;

                fn try_from(object: &'a AnyObject) -> Result<Self, Self::Error> {
                    match object {
                        AnyObject::$obj(obj) => Ok(obj),
                        _ => eyre::bail!("failed to convert object"),
                    }
                }
            }
        )*
    };
}

define_objects! {
    pub struct Group {
        pub(crate) fids: Vec<u32>,
    }

    pub struct PresetDimmer { pub(crate) content: PresetContent<Dimmer>, }
    pub struct PresetPosition { pub(crate) content: PresetContent<Position>, }
    pub struct PresetGobo { pub(crate) content: PresetContent<Gobo>, }
    pub struct PresetColor { pub(crate) content: PresetContent<Color>, }
    pub struct PresetBeam { pub(crate) content: PresetContent<Beam>, }
    pub struct PresetFocus { pub(crate) content: PresetContent<Focus>, }
    pub struct PresetControl { pub(crate) content: PresetContent<Control>, }
    pub struct PresetShapers { pub(crate) content: PresetContent<Shapers>, }
    pub struct PresetVideo { pub(crate) content: PresetContent<Video>, }
}

impl Group {
    pub fn fids(&self) -> &[u32] {
        &self.fids
    }
}
