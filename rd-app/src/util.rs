pub trait EnumerateValue: Clone {
    fn enumerated_value(&self, offset: usize) -> Self;
}

pub struct ValueEnumerator;

impl ValueEnumerator {
    pub fn enumerate<'a, I, T>(iter: I, step: usize)
    where
        I: IntoIterator<Item = &'a mut T>,
        T: EnumerateValue + 'a,
    {
        let mut iter = iter.into_iter();

        if let Some(first) = iter.next() {
            let base = first.clone();

            for (i, item) in iter.enumerate() {
                let total_offset = step * (i + 1);
                *item = base.enumerated_value(total_offset);
            }
        }
    }

    pub fn copy_first<'a, I, T>(iter: I)
    where
        I: IntoIterator<Item = &'a mut T>,
        T: Clone + 'a,
    {
        let mut iter = iter.into_iter();

        if let Some(first) = iter.next() {
            let base = first.clone();

            for item in iter {
                *item = base.clone();
            }
        }
    }
}

impl EnumerateValue for String {
    fn enumerated_value(&self, offset: usize) -> Self {
        let mut digit_start = self.len();
        for (idx, c) in self.char_indices().rev() {
            if c.is_ascii_digit() {
                digit_start = idx;
            } else {
                break;
            }
        }

        let (base_name, start_num) = if digit_start < self.len() {
            (&self[..digit_start], self[digit_start..].parse::<usize>().ok())
        } else {
            (&self[..], None)
        };

        match start_num {
            Some(num) => format!("{}{}", base_name, num + offset),
            None => self.clone(),
        }
    }
}

impl EnumerateValue for gpui::SharedString {
    fn enumerated_value(&self, offset: usize) -> Self {
        let s: String = self.to_string();
        gpui::SharedString::from(s.enumerated_value(offset))
    }
}

macro_rules! impl_enumerate_number {
    ($($t:ty),*) => {
        $(
            impl EnumerateValue for $t {
                fn enumerated_value(&self, offset: usize) -> Self {
                    *self + (offset as $t)
                }
            }
        )*
    };
}

impl_enumerate_number!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
