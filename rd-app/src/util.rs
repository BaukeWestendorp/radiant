pub trait Incrementable {
    fn increment_by(&self, n: usize) -> Self;
}

macro_rules! impl_incrementable_for_int {
    ($($t:ty),*) => {
        $(
            impl Incrementable for $t {
                fn increment_by(&self, n: usize) -> Self {
                    *self + (n as $t)
                }
            }
        )*
    };
}

impl_incrementable_for_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl Incrementable for String {
    fn increment_by(&self, n: usize) -> Self {
        let mut digit_start = self.len();

        for (i, c) in self.char_indices().rev() {
            if c.is_ascii_digit() {
                digit_start = i;
            } else {
                break;
            }
        }

        if digit_start == self.len() {
            self.clone()
        } else {
            let prefix = &self[..digit_start];
            let num_str = &self[digit_start..];

            if let Ok(num) = num_str.parse::<usize>() {
                format!("{}{}", prefix, num + n)
            } else {
                self.clone()
            }
        }
    }
}

impl Incrementable for rd_dmx::Address {
    fn increment_by(&self, n: usize) -> Self {
        let new_address = self.to_absolute().saturating_add(n as u32);
        rd_dmx::Address::from_absolute(new_address).unwrap_or(*self)
    }
}

impl Incrementable for rd_dmx::Channel {
    fn increment_by(&self, n: usize) -> Self {
        let new_channel = self.as_u16().saturating_add(n as u16);
        rd_dmx::Channel::new(new_channel).unwrap_or(*self)
    }
}

impl Incrementable for rd_dmx::UniverseId {
    fn increment_by(&self, n: usize) -> Self {
        let new_universe = self.as_u16().saturating_add(n as u16);
        rd_dmx::UniverseId::new(new_universe).unwrap_or(*self)
    }
}
