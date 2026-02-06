use std::ffi::{CString, c_char};

mod ffi {
    use std::ffi::{c_char, c_int};
    #[repr(C)]
    #[allow(improper_ctypes)]
    pub struct TCCState;

    #[allow(improper_ctypes)]
    unsafe extern "C" {
        // Functions you want to call from libtcc
        pub fn tcc_new() -> *mut TCCState;
        pub fn tcc_delete(s: *mut TCCState);
        pub fn tcc_compile_string(s: *mut TCCState, buf: *const c_char) -> c_int;
        pub fn tcc_output_file(s: *mut TCCState, filename: *const c_char) -> c_int;
        pub fn tcc_add_library_path(s: *mut TCCState, pathname: *const c_char) -> c_int;
        pub fn tcc_add_library(s: *mut TCCState, libraryname: *const c_char) -> c_int;
        pub fn tcc_set_lib_path(s: *mut TCCState, path: *const c_char);
        pub fn tcc_add_file(s: *mut TCCState, filename: *const c_char) -> c_int;
        pub fn tcc_set_output_type(s: *mut TCCState, output_type: c_int) -> c_int;
        pub fn tcc_run(s: *mut TCCState, argc: c_int, argv: *const *const c_char) -> c_int;
    }
}

pub struct TCCState {
    state: *mut ffi::TCCState,
}

#[allow(dead_code)]
pub enum OutputType {
    // #define TCC_OUTPUT_MEMORY     1 /* output will be run in memory (default) */
    // #define TCC_OUTPUT_EXE        2 /* executable file */
    // #define TCC_OUTPUT_DLL        3 /* dynamic library */
    // #define TCC_OUTPUT_OBJ        4 /* object file */
    // #define TCC_OUTPUT_PREPROCESS 5 /* only preprocess (used internally) */
    Memory,
    Executable,
    DynamicLibrary,
    Object,
    Preprocess,
}

#[allow(dead_code)]
impl TCCState {
    pub fn new() -> Self {
        let state = unsafe { ffi::tcc_new() };
        if state.is_null() {
            panic!("could not initialize Tiny C Compiler state");
        }
        TCCState { state }
    }

    pub fn set_output_type(&self, output_type: OutputType) {
        let output_type = match output_type {
            OutputType::Memory => 1,
            OutputType::Executable => 2,
            OutputType::DynamicLibrary => 3,
            OutputType::Object => 4,
            OutputType::Preprocess => 5,
        };
        unsafe {
            ffi::tcc_set_output_type(self.state, output_type);
        }
    }

    pub fn add_file(&self, filename: &str) {
        let filename = CString::new(filename).unwrap();
        unsafe {
            ffi::tcc_add_file(self.state, filename.as_ptr());
        }
    }

    pub fn set_lib_path(&self, path: &str) {
        let path = CString::new(path).unwrap();
        unsafe {
            ffi::tcc_set_lib_path(self.state, path.as_ptr());
        }
    }

    pub fn add_library_path(&self, pathname: &str) {
        let pathname = CString::new(pathname).unwrap();
        unsafe {
            ffi::tcc_add_library_path(self.state, pathname.as_ptr());
        }
    }

    pub fn add_library(&self, libraryname: &str) {
        let libraryname = CString::new(libraryname).unwrap();
        unsafe {
            ffi::tcc_add_library(self.state, libraryname.as_ptr());
        }
    }

    pub fn output_file(&self, filename: &str) {
        let filename = CString::new(filename).unwrap();
        unsafe {
            ffi::tcc_output_file(self.state, filename.as_ptr());
        }
    }

    pub fn compile_string(&mut self, code: &str) -> Result<(), ()> {
        let c_code = match CString::new(code) {
            Ok(s) => s,
            Err(_) => return Err(()),
        };

        let ret = unsafe { ffi::tcc_compile_string(self.state, c_code.as_ptr()) };
        if ret == 0 {
            return Ok(());
        }
        Err(())
    }

    pub fn run(&self, args: &[&str]) -> i32 {
        let argc = args.len() as i32;

        let argv: Vec<CString> = args.iter().map(|arg| CString::new(*arg).unwrap()).collect();

        let argv: Vec<*const c_char> = argv.iter().map(|s| s.as_ptr()).collect();

        let argv = argv.as_ptr();

        return unsafe {
            ffi::tcc_run(self.state, argc, argv)
        };
    }
}

impl Drop for TCCState {
    fn drop(&mut self) {
        unsafe {
            ffi::tcc_delete(self.state);
        }
    }
}
