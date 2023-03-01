#![feature(generic_arg_infer)]
#![allow(non_upper_case_globals,dead_code)]

#[link(name="Microsoft.Holographic.AppRemoting.OpenXr")]
extern "C" { #[link_name="xrNegotiateLoaderRuntimeInterface"] fn negotiate_loader_runtime_interface(loader_info: *const NegotiateLoaderInfo, runtime_request: *mut NegotiateRuntimeRequest) -> Result; }
use std::{ptr::null, mem::size_of, ffi::{c_char as char,c_void as void}};
#[derive(Debug)] #[derive(PartialEq)] #[repr(C)] enum Result { Success = 0 }
type Instance = u64;

#[repr(C)] enum InterfaceStructs { Uninitialized = 0, LoaderInfo, ApiLayerRequest, RuntimeRequest, ApiLayerCreateInfo, ApiLayerNextInfo }

#[repr(C)] struct NegotiateLoaderInfo { struct_type: InterfaceStructs, version: u32, size: usize, min_interface_version: u32, max_interface_version: u32, min_api_version: u64, max_api_version: u64 }
impl NegotiateLoaderInfo { const version : u32 = 1; }
impl Default for NegotiateLoaderInfo { fn default() -> Self { Self{struct_type: InterfaceStructs::LoaderInfo, version: Self::version, size: size_of::<Self>(),
    min_interface_version: 1, max_interface_version: 1, min_api_version:  0x1_0000_0000_0000, max_api_version: 0x1_03FF_0000_0FFF }}}

#[repr(C)] struct NegotiateRuntimeRequest{ty: InterfaceStructs, version: u32, size: usize, runtime_interface_version: u32, runtime_api_version: u64,
    get_instance_proc_addr: Option<extern "C" fn(instance: Instance, name: *const char, function: *mut Option<extern "C" fn()>) -> Result>}
impl NegotiateRuntimeRequest { const version : u32 = 1; }
impl Default for NegotiateRuntimeRequest { fn default() -> Self { Self{ty: InterfaceStructs::RuntimeRequest, version: Self::version, size: size_of::<Self>(),
    runtime_interface_version: 0, runtime_api_version: 0, get_instance_proc_addr: None}}}

#[derive(Debug)] #[repr(C)] enum StructureType { Unknown = 0, ApiLayerProperties, ExtensionProperties, }
//struct LoaderInitInfoBaseHeader { struct_type: StructureType, next: *const c_void }
#[derive(Debug)] #[repr(C)] struct ExtensionProperties{ty: StructureType, next: *const void, extension_name: [u8; 128], extension_version: u32}
impl Default for ExtensionProperties { fn default() -> Self { Self{ty: StructureType::ExtensionProperties, next: null(), extension_name: [0; _], extension_version: 0}}}

fn main() {
    let ref mut runtime_request = NegotiateRuntimeRequest::default();
    assert!(unsafe{negotiate_loader_runtime_interface(&NegotiateLoaderInfo::default() as *const _, runtime_request as *mut _)} == Result::Success);
    let get_instance_proc_addr = runtime_request.get_instance_proc_addr.unwrap();
    /*let mut initialize_loader : Option<extern "C" fn(*const LoaderInitInfoBaseHeader)->Result> = None;
    get_instance_proc_addr(0, b"xrInitializeLoaderKHR\0" as *const _ as *const _, &mut initialize_loader as *mut _ as *mut _);
    assert!(initialize_loader.is_none());*/
    let mut enumerate_instance_extension_properties : Option<extern "C" fn(layer_name: *const char, property_capacity_input: u32, property_count_output: *mut u32, properties: *mut ExtensionProperties)->Result> = None;
    get_instance_proc_addr(0, b"xrEnumerateInstanceExtensionProperties\0" as *const _ as *const _, &mut enumerate_instance_extension_properties as *mut _ as *mut _);
    let enumerate_instance_extension_properties = enumerate_instance_extension_properties.unwrap();
    fn array<T:Default>(capacity_len_buffer: impl Fn(u32, &mut u32, *mut T)->Result) -> Box<[T]> {
        let mut len = 0;
        assert_eq!(capacity_len_buffer(0, &mut len, std::ptr::null_mut()), Result::Success);
        let mut buffer = std::iter::from_fn(|| Some(T::default())).take(len as usize).collect::<Box<_>>();
        let mut len = 0;
        assert_eq!(capacity_len_buffer(buffer.len() as u32, &mut len, buffer.as_mut_ptr()), Result::Success);
        assert_eq!(buffer.len(), len as usize);
        buffer
    }
    let extensions = array(|capacity, len, buffer| enumerate_instance_extension_properties(null(), capacity, len, buffer));
    for extension in &*extensions { println!("{:?} {:?}", extension, std::ffi::CStr::from_bytes_until_nul(&extension.extension_name).unwrap()); }
    let any = |name:&[u8]| extensions.iter().any(|ExtensionProperties{extension_name,..}| &extension_name[0..name.len()] == name);
    assert!(any(b"khr_d3d12_enable\0") && any(b"msft_holographic_remoting\0"));

}
