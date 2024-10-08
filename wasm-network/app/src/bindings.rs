// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
const _: () = {
  
  #[doc(hidden)]
  #[export_name = "run"]
  #[allow(non_snake_case)]
  unsafe extern "C" fn __export_run(arg0: i32,arg1: i32,arg2: i32,arg3: i32,) {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
    
    // Before executing any other code, use this function to run all static
    // constructors, if they have not yet been run. This is a hack required
    // to work around wasi-libc ctors calling import functions to initialize
    // the environment.
    //
    // This functionality will be removed once rust 1.69.0 is stable, at which
    // point wasi-libc will no longer have this behavior.
    //
    // See
    // https://github.com/bytecodealliance/preview2-prototyping/issues/99
    // for more details.
    #[cfg(target_arch="wasm32")]
    wit_bindgen::rt::run_ctors_once();
    
    let len0 = arg1 as usize;
    let bytes0 = Vec::from_raw_parts(arg0 as *mut _, len0, len0);
    let len1 = arg3 as usize;
    let bytes1 = Vec::from_raw_parts(arg2 as *mut _, len1, len1);
    <_GuestImpl as Guest>::run(wit_bindgen::rt::string_lift(bytes0), wit_bindgen::rt::string_lift(bytes1));
  }
};
use super::Component as _GuestImpl;
pub trait Guest {
  fn run(uri: wit_bindgen::rt::string::String,filename: wit_bindgen::rt::string::String,);
}
pub mod component {
  pub mod host {
    
    #[allow(clippy::all)]
    pub mod network_provider {
      #[used]
      #[doc(hidden)]
      #[cfg(target_arch = "wasm32")]
      static __FORCE_SECTION_REF: fn() = super::super::super::__link_section;
      #[allow(unused_unsafe, clippy::all)]
      pub fn get(uri: &str,filename: &str,){
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, vec::Vec, string::String};
        unsafe {
          let vec0 = uri;
          let ptr0 = vec0.as_ptr() as i32;
          let len0 = vec0.len() as i32;
          let vec1 = filename;
          let ptr1 = vec1.as_ptr() as i32;
          let len1 = vec1.len() as i32;
          
          #[cfg(target_arch = "wasm32")]
          #[link(wasm_import_module = "component:host/network-provider")]
          extern "C" {
            #[link_name = "get"]
            fn wit_import(_: i32, _: i32, _: i32, _: i32, );
          }
          
          #[cfg(not(target_arch = "wasm32"))]
          fn wit_import(_: i32, _: i32, _: i32, _: i32, ){ unreachable!() }
          wit_import(ptr0, len0, ptr1, len1);
        }
      }
      
    }
    
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:app"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 240] = [3, 0, 3, 97, 112, 112, 0, 97, 115, 109, 13, 0, 1, 0, 7, 123, 1, 65, 2, 1, 65, 4, 1, 66, 2, 1, 64, 2, 3, 117, 114, 105, 115, 8, 102, 105, 108, 101, 110, 97, 109, 101, 115, 1, 0, 4, 0, 3, 103, 101, 116, 1, 0, 3, 1, 31, 99, 111, 109, 112, 111, 110, 101, 110, 116, 58, 104, 111, 115, 116, 47, 110, 101, 116, 119, 111, 114, 107, 45, 112, 114, 111, 118, 105, 100, 101, 114, 5, 0, 1, 64, 2, 3, 117, 114, 105, 115, 8, 102, 105, 108, 101, 110, 97, 109, 101, 115, 1, 0, 4, 0, 3, 114, 117, 110, 1, 1, 4, 1, 17, 99, 111, 109, 112, 111, 110, 101, 110, 116, 58, 97, 112, 112, 47, 97, 112, 112, 4, 0, 11, 9, 1, 0, 3, 97, 112, 112, 3, 0, 0, 0, 16, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
