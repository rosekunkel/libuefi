/// A UEFI status code.
#[derive(Copy, Clone, Debug)]
#[repr(usize)]
pub enum Status {
    // Success and warning return codes.
    Success = 0,
    WarnUnknownGlyph,
    WarnDeleteFailure,
    WarnWriteFailure,
    WarnBufferTooSmall,
    WarnStaleData,
    /// Marker value for the end of valid warnings. Not for external use.
    _LastWarning,

    // Error return codes. Error return codes have their most significant bit
    // set, while success and warning return codes have it unset.
    LoadError = ((!0) ^ (!0 >> 1)) | 1,
    InvalidParameter,
    Unsupported,
    BadBufferSize,
    BufferTooSmall,
    NotReady,
    DeviceError,
    WriteProtected,
    OutOfResources,
    VolumeCorrupted,
    VolumeFull,
    NoMedia,
    MediaChanged,
    NotFound,
    AccessDenied,
    NoResponse,
    NoMapping,
    Timeout,
    NotStarted,
    AlreadyStarted,
    Aborted,
    IcmpError,
    TftpError,
    ProtocolError,
    IncompatibleVersion,
    SecurityViolation,
    CrcError,
    EndOfMedia,
    EndOfFile,
    InvalidLanguage,
    CompromisedData,
    IpAdressConflict,
    /// Marker value for the end of valid errors. Not for external use.
    _LastError
}

impl Status {
    pub fn is_error(&self) -> bool {
        *self as usize ^ ((!0) ^ (!0 >> 1)) != 0
    }

    pub fn is_success(&self) -> bool {
        match *self {
            Status::Success => true,
            _ => false
        }
    }

    pub fn is_warning(&self) -> bool {
        match *self {
            Status::Success => false,
            _ => !self.is_error()
        }
    }
}
