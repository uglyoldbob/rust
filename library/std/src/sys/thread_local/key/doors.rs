#[derive(Copy, Clone)]
pub struct Key;

pub struct LazyKey;

impl LazyKey {
    pub const fn new(_a : Option<unsafe extern "C" fn(*mut u8)>) -> Self {
        Self
    }

    #[inline]
    pub fn force(&self) -> super::Key {
        todo!()
    }
}

#[inline]
pub unsafe fn set(_key: Key, _value: *mut u8) {
    todo!()
}

#[inline]
pub unsafe fn get(_key: Key) -> *mut u8 {
    todo!()
}
