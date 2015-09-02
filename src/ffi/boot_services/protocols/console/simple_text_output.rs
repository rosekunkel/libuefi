use ffi::types::RawStatusCode;

#[repr(C)]
pub struct SimpleTextOutput {
    pub reset: unsafe extern "win64" fn(*mut SimpleTextOutput,
                                        bool)
                                        -> RawStatusCode,
    pub output_string: unsafe extern "win64" fn(*mut SimpleTextOutput,
                                                *const u16)
                                                -> RawStatusCode,
    pub test_string: unsafe extern "win64" fn(*mut SimpleTextOutput,
                                              *const u16)
                                              -> RawStatusCode,
    pub query_mode: unsafe extern "win64" fn(*mut SimpleTextOutput,
                                             usize,
                                             *mut usize,
                                             *mut usize)
                                             -> RawStatusCode,
    pub set_mode: unsafe extern "win64" fn(*mut SimpleTextOutput,
                                           usize)
                                           -> RawStatusCode,
    pub set_attribute: unsafe extern "win64" fn(*mut SimpleTextOutput,
                                                usize)
                                                -> RawStatusCode,
    pub clear_screen: unsafe extern "win64" fn(*mut SimpleTextOutput)
                                               -> RawStatusCode,
    pub set_cursor_position: unsafe extern "win64" fn(*mut SimpleTextOutput,
                                                      usize,
                                                      usize)
                                                      -> RawStatusCode,
    pub enable_cursor: unsafe extern "win64" fn(*mut SimpleTextOutput,
                                                bool)
                                                -> RawStatusCode,
    pub mode: *const SimpleTextOutputMode
}

#[repr(C)]
pub struct SimpleTextOutputMode {
    pub max_mode: i32,
    pub mode: i32,
    pub attribute: i32,
    pub cursor_column: i32,
    pub cursor_row: i32,
    pub cursor_visible: bool
}
