#![feature(start, no_std, lang_items, core_slice_ext, asm)]
#![no_std]

enum Void {}

#[repr(C)]
struct EfiSimpleTextOutputProtocol {
    reset: *const Void,
    output_string: extern "win64" fn(*mut EfiSimpleTextOutputProtocol, *const u16) -> usize,
    test_string: *const Void,
    query_mode: *const Void,
    set_mode: *const Void,
    set_attribute: *const Void,
    clear_screen: extern "win64" fn(*mut EfiSimpleTextOutputProtocol) -> usize,
    set_cursor_position: *const Void,
    enable_cursor: *const Void,
    mode: *const Void
}

#[repr(C)]
struct EfiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    checksum: u32,
    reserved: u32
}

#[repr(C)]
pub struct EfiSystemTable {
    header: EfiTableHeader,
    firmware_vendor: *const u16,
    firmware_revision: u32,
    console_in_handle: *const Void,
    con_in: *const Void,
    console_out_handle: *const Void,
    con_out: *mut EfiSimpleTextOutputProtocol,
    standard_error_handle: *const Void,
    std_err: *const Void,
    runtime_services: *const Void,
    boot_services: *const Void,
    number_of_table_entries: usize,
    configuration_table: *const Void
}

#[start]
#[no_mangle]
pub extern "win64" fn efi_main(_image_handle: *const (), system_table: *mut EfiSystemTable) -> usize {
    let hello = ['H' as u16,
                 'e' as u16,
                 'l' as u16,
                 'l' as u16,
                 'o' as u16,
                 ',' as u16,
                 ' ' as u16,
                 'W' as u16,
                 'o' as u16,
                 'r' as u16,
                 'l' as u16,
                 'd' as u16,
                 '\r' as u16,
                 '\n' as u16,
                 0u16];
    unsafe {
        let ref system_table = *system_table;
        let ref mut con_out = *system_table.con_out;
        let output_string = con_out.output_string;

        output_string(con_out, hello.as_ptr());
    };

    halt()
}

#[test]
fn it_works() {
}
