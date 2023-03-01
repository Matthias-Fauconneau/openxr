#![allow(non_upper_case_globals)]
//#![feature(raw_dylib)]

//#[link(name="Microsoft.Holographic.AppRemoting.OpenXr", kind="raw-dylib")]
#[link(name="Microsoft.Holographic.AppRemoting.OpenXr")]
extern "C" { #[link_name="xrNegotiateLoaderRuntimeInterface"] fn negotiate_loader_runtime_interface(loader_info: *const NegotiateLoaderInfo, runtime_request: *mut NegotiateRuntimeRequest) -> Result; }
use std::{ptr::null, mem::size_of, ffi::c_char};

type Instance = u64;
//type get_instance_proc_addr = ;

#[allow(dead_code)] #[repr(C)] enum InterfaceStructs { Uninitialized = 0, LoaderInfo, ApiLayerRequest, RuntimeRequest, ApiLayerCreateInfo, ApiLayerNextInfo }

#[repr(C)] struct NegotiateLoaderInfo { struct_type: InterfaceStructs, version: u32, size: usize, min_interface_version: u32, max_interface_version: u32, min_api_version: u64, max_api_version: u64 }
impl NegotiateLoaderInfo { const version : u32 = 1; }
impl Default for NegotiateLoaderInfo { fn default() -> Self { Self{struct_type: InterfaceStructs::LoaderInfo, version: Self::version, size: size_of::<Self>(),
    min_interface_version: 1, max_interface_version: 1, min_api_version:  0x1_0000_0000_0000, max_api_version: 0x1_03FF_0000_0FFF }}}

#[repr(C)] struct NegotiateRuntimeRequest { struct_type: InterfaceStructs, version: u32, size: usize, runtime_interface_version: u32, runtime_api_version: u64,
    get_instance_proc_addr: *const extern "C" fn(instance: Instance, name: *const c_char, function: *const fn()) -> Result}
impl NegotiateRuntimeRequest { const version : u32 = 1; }
impl Default for NegotiateRuntimeRequest { fn default() -> Self { Self{struct_type: InterfaceStructs::RuntimeRequest, version: Self::version, size: size_of::<Self>(),
    runtime_interface_version: 0, runtime_api_version: 0, get_instance_proc_addr: null()}}}

#[allow(dead_code)] #[repr(C)] enum Result { Success = 0 }

fn main() {
    let ref mut runtime_request = NegotiateRuntimeRequest::default();
    println!("{:?} {:?}", unsafe{negotiate_loader_runtime_interface(&NegotiateLoaderInfo::default() as *const _, runtime_request as *mut _)} as i32, runtime_request.get_instance_proc_addr);
}
