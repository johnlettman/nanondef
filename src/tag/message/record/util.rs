use crate::Error;

pub const PAYLOAD_SR_THRESHOLD: usize = 255;

pub const MAX_ID_LEN: usize = 255;
pub const MAX_TY_LEN: usize = 255;

pub const fn id_error(id: Option<&[u8]>) -> Option<Error> {
    match id {
        Some(id) => {
            let id_len = id.len();

            if id_len > MAX_ID_LEN {
                Some(Error::RecordIdTooLong(id_len))
            } else {
                None
            }
        },
        None => None,
    }
}

#[inline]
pub fn valid_id(id: Option<&[u8]>) -> bool {
    id_error(id).is_none()
}

pub const fn ty_error(ty: &str) -> Option<Error> {
    let ty_len = ty.len();

    if ty_len > MAX_TY_LEN {
        return Some(Error::RecordTyTooLong(ty_len));
    }

    None
}

#[inline]
pub fn valid_ty(ty: &str) -> bool {
    ty_error(ty).is_none()
}
