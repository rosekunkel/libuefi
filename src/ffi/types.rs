use core::mem;
use types::Status;

/// A raw status code returned by a UEFI function.
pub type RawStatusCode = usize;

/// Convert a `RawStatusCode` to a UEFI status code.
///
/// Convert `RawStatusCode` to its corresponding UEFI status enum variant,
/// unless it is not a valid UEFI status code, in which case return `None`.
pub fn to_status(code: RawStatusCode) -> Option<Status> {
    // Check that the code is within the range of allowable status codes.
    if (code >= Status::Success as RawStatusCode &&
        code < Status::_LastWarning as RawStatusCode) ||
       (code >= Status::LoadError as RawStatusCode &&
        code < Status::_LastError as RawStatusCode) {
        // Status is repr(usize), so transmuting is safe once we've determined
        // the value is in range.
        Some(unsafe { mem::transmute(code) })
    } else {
        None
    }
}
