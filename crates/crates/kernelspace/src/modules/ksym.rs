// #![no_std]
// #![feature(allocator_api, global_asm)]

// use kernel::module;
// use kernel::prelude::*;

// module! {
//     type: ListSymbolsModule,
//     name: b"list_symbols_rust",
//     author: b"Ze Gois",
//     description: b"List all exported kernel symbols",
//     license: b"GPL",
// }

// struct ListSymbolsModule;

// impl KernelModule for ListSymbolsModule {
//     fn init() -> Result<Self> {
//         pr_info!("Loading Rust symbol lister module...\n");

//         // kallsyms_on_each_symbol não está diretamente exposto em Rust,
//         // mas você pode chamar via FFI se habilitado no kernel
//         unsafe {
//             extern "C" {
//                 fn kallsyms_on_each_symbol(
//                     callback: extern "C" fn(*const u8, *mut core::ffi::c_void, usize) -> i32,
//                     data: *mut core::ffi::c_void,
//                 );
//             }

//             extern "C" fn cb(name: *const u8, _data: *mut core::ffi::c_void, addr: usize) -> i32 {
//                 // Converte name de C string
//                 if !name.is_null() {
//                     let c_str = unsafe { core::ffi::CStr::from_ptr(name as *const i8) };
//                     if let Ok(s) = c_str.to_str() {
//                         pr_info!("Symbol: {} at {:p}\n", s, addr as *const ());
//                     }
//                 }
//                 0
//             }

//             kallsyms_on_each_symbol(cb, core::ptr::null_mut());
//         }

//         Ok(ListSymbolsModule)
//     }
// }

// impl Drop for ListSymbolsModule {
//     fn drop(&mut self) {
//         pr_info!("Unloading Rust symbol lister module\n");
//     }
// }
