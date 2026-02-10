use atomic_float::AtomicF32;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref VOICE: AtomicF32 = AtomicF32::new(0.0);
}