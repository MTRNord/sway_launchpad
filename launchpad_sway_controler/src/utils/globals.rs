use std::sync::atomic::{AtomicBool, AtomicI64};

pub static SHOWING_NUMBER: AtomicBool = AtomicBool::new(false);
pub static PRESELECTED_LAYER_NUMBER: AtomicI64 = AtomicI64::new(0);
pub static LAYER_NUMBER: AtomicI64 = AtomicI64::new(0);
pub static CURRENT_WORKSPACE: AtomicI64 = AtomicI64::new(0);
