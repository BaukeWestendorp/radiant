use crate::FormDelegate;

pub enum FormEvent<D: FormDelegate> {
    Submit { data: D::Data },
}
