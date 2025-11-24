
pub use nanondef::tag::Features;

#[no_mangle]
pub extern "C" fn tag_features_has(f: Features, bit: u8) -> bool {
    Features(f.0).has(bit)
}

#[no_mangle]
pub extern "C" fn tag_features_with(f: Features, bit: u8) -> Features {
    Features(f.0).with(bit).into()
}

/// test
#[no_mangle]
pub extern "C" fn tag_features_without(f: Features, bit: u8) -> Features {
    Features(f.0).without(bit).into()
}
