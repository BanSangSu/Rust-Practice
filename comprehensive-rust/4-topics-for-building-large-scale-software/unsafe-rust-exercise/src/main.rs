// Unsafe Rust https://google.github.io/comprehensive-rust/unsafe-rust/unsafe.html
fn unsafe_rust() {
    let mut num = 5;

    // let r1 = &raw const num;
    // let r2 = &raw mut num;
    let r1 = &num as *const i32; // Create a raw const pointer
    let r2 = &mut num as *mut i32; // Create a raw mutable pointer

    println!("r1={:p}, r2={:p}", r1, r2);
    unsafe {
        println!("Value through r1={}, r2={}", *r1, *r2);
    }
}

// Dereferencing Raw Pointers https://google.github.io/comprehensive-rust/unsafe-rust/dereferencing.html
fn dereferencing_raw_pointers() {
    let mut s = String::from("careful!");

    // let r1 = &raw mut s; // experimental feature
    let r1 = &mut s as *mut String;
    let r2 = r1 as *const String;

    // SAFETY: r1 and r2 were obtained from references and so are guaranteed to
    // be non-null and properly aligned, the objects underlying the references
    // from which they were obtained are live throughout the whole unsafe
    // block, and they are not accessed either through the references or
    // concurrently through any other pointers.
    unsafe {
        println!("r1 is: {}", *r1);
        *r1 = String::from("uhoh");
        println!("r2 is: {}", *r2);
    }

    // NOT SAFE. DO NOT DO THIS.
    /*
    let r3: &String = unsafe { &*r1 };
    drop(s);
    println!("r3 is: {}", *r3);
    */
}


// Mutable Static Variables https://google.github.io/comprehensive-rust/unsafe-rust/mutable-static.html
fn mutable_static_variables() {
    // static HELLO_WORLD: &str = "Hello, world!";
    
    // println!("HELLO_WORLD: {HELLO_WORLD}");

    static mut COUNTER: u32 = 0;

    fn add_to_counter(inc: u32) {
        // SAFETY: There are no other threads which could be accessing `COUNTER`.
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_counter(42);

    // SAFETY: There are no other threads which could be accessing `COUNTER`.
    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}


// Unions https://google.github.io/comprehensive-rust/unsafe-rust/unions.html
fn unions() {
    #[repr(C)]
    union MyUnion {
        i: u8,
        b: bool,
    }

    let u = MyUnion { i: 42};
    println!("int: {}", unsafe { u.i });
    println!("bool: {}", unsafe { u.b }); // Undefined behavior!
}


// Unsafe Functions https://google.github.io/comprehensive-rust/unsafe-rust/unsafe-functions.html
// no code

// Unsafe Rust Functions https://google.github.io/comprehensive-rust/unsafe-rust/unsafe-functions/rust.html

// #[deny(unsafe_op_in_unsafe_fn)]
// #[warn(unsafe_op_in_unsafe_fn)] // only warning
fn unsafe_rust_functions() {
    /// Swaps the values pointed to by the given pointers.
    ///
    /// # Safety
    ///
    /// The pointers must be valid, properly aligned, and not otherwise accessed for
    /// the duration of the function call.
    unsafe fn swap(a: *mut u8, b: *mut u8) {
        // unsafe { // do it when we use #[deny(unsafe_op_in_unsafe_fn)].
            let temp = *a;
            *a = *b;
            *b = temp;
        // }
    }

    let mut a = 42;
    let mut b = 66;

    // SAFETY: The pointers must be valid, aligned and unique because they came
    // from references.
    unsafe {
        swap(&mut a, &mut b);
    }

    println!("a = {}, b = {}", a, b);
}

// Unsafe External Functions https://google.github.io/comprehensive-rust/unsafe-rust/unsafe-functions/extern-c.html

fn unsafe_external_functions () {
    use std::ffi::c_char;
    
    unsafe extern "C" {
        // `abs` doesn't deal with pointers and doesn't have any safety requirements.
        safe fn abs(input: i32) -> i32;
    
        /// # Safety
        ///
        /// `s` must be a pointer to a NUL-terminated C string which is valid and
        /// not modified for the duration of this function call.
        unsafe fn strlen(s: *const c_char) -> usize;
    }

    println!("Absolute value of -3 according to C: {}", abs(-3));

    unsafe {
        // SAFETY: We pass a pointer to a C string literal which is valid for
        // the duration of the program.
        println!("String length: {}", strlen(c"String".as_ptr()));
    }
}

// Calling Unsafe Functions https://google.github.io/comprehensive-rust/unsafe-rust/unsafe-functions/calling.html
fn calling_unsafe_functions() {
    
    #[derive(Debug)]
    #[repr(C)]
    struct KeyPair {
        pk: [u16; 4], // 8 bytes
        sk: [u16; 4], // 8 bytes
    }
    
    const PK_BYTE_LEN: usize = 8;

    fn log_public_key(pk_ptr: *const u16) {
        let pk: &[u16] = unsafe { std::slice::from_raw_parts(pk_ptr, PK_BYTE_LEN)};
        println!("{pk:?}");
    }

    let key_pair = KeyPair{ pk: [1, 2, 3, 4], sk: [0, 0, 42, 0] };
    log_public_key(key_pair.pk.as_ptr());
}


// Implementing Unsafe Traits https://google.github.io/comprehensive-rust/unsafe-rust/unsafe-traits.html
use std::{mem, slice};

fn implementing_unsafe_traits() {
    
    /// ...
    /// # Safety
    /// The type must have a defined representation and no padding.
    pub unsafe trait IntoBytes {
        fn as_bytes(&self) -> &[u8] {
            let len = mem::size_of_val(self);
            let slf: *const Self = self;
            unsafe { slice::from_raw_parts(slf.cast::<u8>(), len) }
        }
    }

    // SAFETY: `u32` has a defined representation and no padding.
    unsafe impl IntoBytes for u32 {}

    let num: u32 = 0x12345678;
    println!("u32 as bytes: {:?}", num.as_bytes());

}


//////
// Exercise: FFI Wrapper
// Safe FFI Wrapper https://google.github.io/comprehensive-rust/unsafe-rust/exercise.html
// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

mod ffi {
    use std::os::raw::{c_char, c_int};
    #[cfg(not(target_os = "macos"))]
    use std::os::raw::{c_long, c_uchar, c_ulong, c_ushort};

    // Opaque type. See https://doc.rust-lang.org/nomicon/ffi.html.
    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // Layout according to the Linux man page for readdir(3), where ino_t and
    // off_t are resolved according to the definitions in
    // /usr/include/x86_64-linux-gnu/{sys/types.h, bits/typesizes.h}.
    #[cfg(not(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_ulong,
        pub d_off: c_long,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    // Layout according to the macOS man page for dir(5).
    #[cfg(all(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_fileno: u64,
        pub d_seekoff: u64,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [c_char; 1024],
    }

    unsafe extern "C" {
        pub unsafe fn opendir(s: *const c_char) -> *mut DIR;

        #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
        pub unsafe fn readdir(s: *mut DIR) -> *const dirent;

        // See https://github.com/rust-lang/libc/issues/414 and the section on
        // _DARWIN_FEATURE_64_BIT_INODE in the macOS man page for stat(2).
        //
        // "Platforms that existed before these updates were available" refers
        // to macOS (as opposed to iOS / wearOS / etc.) on Intel and PowerPC.
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        #[link_name = "readdir$INODE64"]
        pub unsafe fn readdir(s: *mut DIR) -> *const dirent;

        pub unsafe fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // Call opendir and return a Ok value if that worked,
        // otherwise return Err with a message.
        if path =
            CString::new(path).map_err(|err| format!("Invalid path: {err}"))?;
        // SAFETY: path.as_ptr() cannot be NULL.
        let dir = unsafe { ffi::opendir(path.as_ptr()) };
        if dir.is_null() {
            Err(format!("Could not open {path:?}"))
        }else {
            Ok(DirectoryIterator { path, dir})
        }
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        // Keep calling readdir until we get a NULL pointer back.
        // SAFETY: self.dir is never NULL.
        let dirent = unsafe { ffi::readdir(self.dir) };
        if dirent.is_null() {
            // We have reached the end of the directory.
            return None;
        }
        // SAFETY: dirent is not NULL and dirent.d_name is NULL
        // terminated.
        let d_name = unsafe { CStr::from_ptr((*dirent).d_name.as_ptr()) };
        let os_str = OSStr::from_bytes(d_name.to_bytes());
        Some(os_str.to_owned())
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // Call closedir as needed.
        // SAFETY: self.dir is never NULL.

    }
}

fn safe_ffi_wrapper_exercise() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    println!("files: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}


fn main() {
    safe_ffi_wrapper_exercise();

        
    // implementing_unsafe_traits();
    // calling_unsafe_functions();
    // unsafe_external_functions();
    // unsafe_rust_functions();
    // unions();
    // mutable_static_variables();
    // dereferencing_raw_pointers(); 
    // unsafe_rust();
}
