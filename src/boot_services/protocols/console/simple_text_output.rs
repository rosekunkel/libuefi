use ffi::boot_services::protocols::console::simple_text_output as ffi;
use ffi::types::to_status;
use types::Status;

pub struct SimpleTextOutput<'a> {
    simple_text_output: &'a mut ffi::SimpleTextOutput
}

impl<'a> SimpleTextOutput<'a> {
    pub unsafe fn new(simple_text_output: *mut ffi::SimpleTextOutput) -> SimpleTextOutput<'static> {
        SimpleTextOutput {
            simple_text_output: &mut *simple_text_output
        }
    }

    pub fn reset(&mut self, extended_verification: bool) -> Status {
        let return_code = unsafe {
            (self.simple_text_output.reset)(self.simple_text_output,
                                            extended_verification)
        };

        to_status(return_code).unwrap()
    }

    pub fn output_string(&mut self, utf8_str: &str) -> Status {
        const BUFFER_SIZE: usize = 4096;
        let mut buffer = [0u16; BUFFER_SIZE];
        let mut i = 0;
        let mut status = Status::Success;

        for utf8_char in utf8_str.chars() {
            // Minus 2 for the last character, minus 1 for the null character.
            if i < BUFFER_SIZE - 3 {
                let char_slice = &mut buffer[i..(i + 2)];
                let bytes = utf8_char.encode_utf16(char_slice);

                // We gave encode_utf16 a slice with two characters, so we know
                // it succeeded.
                i += bytes.unwrap();
            } else {
                // Null-terminate the string.
                buffer[i] = 0;

                let return_code = unsafe {
                    (self.simple_text_output.output_string)(
                        self.simple_text_output,
                        buffer.as_ptr())
                };

                let returned_status = to_status(return_code).unwrap();

                if returned_status.is_error() {
                    return returned_status;
                }

                status = match returned_status {
                    Status::Success => status,
                    Status::WarnUnknownGlyph => returned_status,
                    _ => unreachable!()
                };
            }
        };

        status
    }
}
