
pub use nanondef::tag::Version;
use crate::cmp::Ordering;

#[no_mangle]
pub extern "C" fn version_size() -> usize {
    size_of::<Version>()
}

#[no_mangle]
pub extern "C" fn version_align() -> usize {
    align_of::<Version>()
}

#[no_mangle]
pub extern "C" fn tag_version_new(major: u8, minor: u8) -> Version {
    Version::new(major, minor)
}

#[no_mangle]
pub extern "C" fn tag_version_encode(v: Version) -> u8 {
    v.encode()
}

#[no_mangle]
pub extern "C" fn tag_version_decode(b: u8) -> Version {
    Version::decode(b)
}

#[no_mangle]
pub extern "C" fn tag_version_cmp(a: Version, b: Version) -> Ordering {
    a.cmp(&b).into()
}
