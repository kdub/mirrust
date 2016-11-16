extern crate libc;
use libc::size_t;
use libc::c_void;
use std::ffi::CString;
use std::os::raw::c_char;

pub enum MirConnection {}

#[link(name = "mirclient")]
extern {
    fn mir_connect_sync(server: *const c_char, app_name: *const c_char) -> *const MirConnection;
//    fn snappy_max_compressed_length(source_length: size_t) -> size_t;
}

fn main() {
//    let x = unsafe { snappy_max_compressed_length(100) };
    let x = 8;
    println!("max compressed length of a 100 byte buffer: {}", x);

    let con = CString::new("/tmp/a").unwrap();
    let name = CString::new("map").unwrap();

    let connection = unsafe { mir_connect_sync(con.as_ptr(), name.as_ptr()) };

    
}
