#[cfg(target_endian = "little")]
#[derive(Clone, Copy)]
#[repr(C)]
struct Inner {
    b: u8, a: u8
}

#[cfg(target_endian = "big")]
#[derive(Clone, Copy)]
#[repr(C)]
struct Inner {
    a: u8, b: u8
}

#[derive(Clone, Copy)]
#[repr(C)]
union Union {
    inner: Inner,
    value: u16
}

#[derive(Clone, Copy)]
pub struct SafeU16Union {
    value: Union
}
/// Safe union wrapper
impl SafeU16Union {
    pub fn new(from: u16) -> Self {
        Self {
           value: Union { value: from } 
        }
    }
    #[inline(always)]
    pub fn set_union_value(&mut self, value: u16) {
        self.value.value = value;
    }
    #[allow(unsafe_code)]
    #[inline(always)]
    pub fn get_union_value(&self) -> u16 {
        unsafe { self.value.value }
    }
    #[allow(unsafe_code)]
    #[inline(always)]
    pub fn get_inner_a(&self) -> u8 {
        unsafe { self.value.inner.a }
    }
    #[allow(unsafe_code)]
    #[inline(always)]
    pub fn get_inner_b(&self) -> u8 {
        unsafe { self.value.inner.b }
    }
    #[inline(always)]
    pub fn set_inner_a(&mut self, value: u8) {
        self.value.inner.a = value;
    }
    #[inline(always)]
    pub fn set_inner_b(&mut self, value: u8) {
        self.value.inner.b = value;
    }
}
impl Default for SafeU16Union {
    fn default() -> Self { Self::new(0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value() {
        let mut union = SafeU16Union::new(0x1234);
        assert_eq!(union.get_union_value(), 0x1234);
        union.set_union_value(0x5678);
        assert_eq!(union.get_union_value(), 0x5678);
    }

    #[test]
    fn inner() {
        let mut union = SafeU16Union::new(0x1234);
        assert_eq!(union.get_inner_a(), 0x12);
        assert_eq!(union.get_inner_b(), 0x34);
        assert_eq!(union.get_union_value(), 0x1234);
        union.set_inner_a(0x56);
        union.set_inner_b(0x78);
        assert_eq!(union.get_inner_a(), 0x56);
        assert_eq!(union.get_inner_b(), 0x78);
        assert_eq!(union.get_union_value(), 0x5678);
    }
}
