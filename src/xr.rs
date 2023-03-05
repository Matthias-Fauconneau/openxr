#![allow(non_upper_case_globals,dead_code)]

#[link(name="Microsoft.Holographic.AppRemoting.OpenXr", kind="raw-dylib")]
extern "C" { #[link_name="xrNegotiateLoaderRuntimeInterface"] pub fn negotiate_loader_runtime_interface(loader_info: *const NegotiateLoaderInfo, runtime_request: *mut NegotiateRuntimeRequest) -> Result; }
use std::{default::default, ptr::null, mem::size_of, ffi::{c_void as void, c_char}}; #[allow(non_camel_case_types)] type char = u8;
#[derive(Debug)] #[derive(PartialEq)] #[repr(C)] pub enum Result { Success = 0 }
pub type Instance = u64;
pub type Session = u64;
pub type Space = u64;
pub type Swapchain = u64;

#[repr(C)] pub enum InterfaceStructs { Uninitialized = 0, LoaderInfo, ApiLayerRequest, RuntimeRequest, ApiLayerCreateInfo, ApiLayerNextInfo }

#[repr(C)] pub struct NegotiateLoaderInfo { pub struct_type: InterfaceStructs, pub version: u32, pub size: usize, pub min_interface_version: u32, pub max_interface_version: u32, pub min_api_version: u64, pub max_api_version: u64 }
impl NegotiateLoaderInfo { const version : u32 = 1; }
impl Default for NegotiateLoaderInfo { fn default() -> Self { Self{struct_type: InterfaceStructs::LoaderInfo, version: Self::version, size: size_of::<Self>(),
    min_interface_version: 1, max_interface_version: 1, min_api_version:  0x1_0000_0000_0000, max_api_version: 0x1_03FF_0000_0FFF }}}

#[repr(C)] pub struct NegotiateRuntimeRequest{pub ty: InterfaceStructs, pub version: u32, pub size: usize, pub runtime_interface_version: u32, pub runtime_api_version: u64,
    pub get_instance_proc_addr: Option<extern "C" fn(instance: Instance, name: *const char, function: *mut Option<extern "C" fn()>) -> Result>}
impl NegotiateRuntimeRequest { const version : u32 = 1; }
impl Default for NegotiateRuntimeRequest { fn default() -> Self { Self{ty: InterfaceStructs::RuntimeRequest, version: Self::version, size: size_of::<Self>(),
    runtime_interface_version: 0, runtime_api_version: 0, get_instance_proc_addr: None}}}

#[derive(Debug,PartialEq)] #[repr(C)] pub enum StructureType {
    Unknown = 0,
    ApiLayerProperties, ExtensionProperties, InstanceCreateInfo, SystemGetInfo, ViewLocateInfo=6, View, SessionCreateInfo, SwapchainCreateInfo, SessionBeginInfo, ViewState, FrameEndInfo,
    EventDataBuffer=16, InstanceLossPending, SessionStateChanged,
    FrameWaitInfo=33, CompositionLayerProjection=35,
    ReferenceSpaceCreateInfo=37, ViewConfigurationView=41, FrameState=44, FrameBeginInfo=46, CompositionLayerProjectionView=48,
    SwapchainImageAcquireInfo=55, SwapchainImageWaitInfo, SwapchainImageReleaseInfo,
    GraphicsBindingD3D12=1000028000, SwapchainImageD3D12, GraphicsRequirementsD3D12,
    RemotingConnectInfo=1000065001,
}

#[derive(Debug)] #[repr(C)] struct ExtensionProperties {ty: StructureType, next: *const void, pub extension_name: [u8; 128], pub extension_version: u32}
impl Default for ExtensionProperties { fn default() -> Self { Self{ty: StructureType::ExtensionProperties, next: null(), extension_name: [0; _], extension_version: 0}}}

#[repr(C)] pub struct ApplicationInfo { pub name: [u8; 128], pub version: u32, pub engine: [u8; 128], pub engine_version: u32, pub api: u64}
#[repr(C)] pub struct InstanceCreateInfo {pub ty: StructureType, pub next: *const void, pub create_flags: u64, pub application_info: ApplicationInfo,
    pub api_layer_count: u32, pub api_layer_names: *const *const u8, pub extension_count: u32, pub extension_names: *const *const char}
impl Default for InstanceCreateInfo { fn default() -> Self { Self{ty: StructureType::InstanceCreateInfo, ..unsafe{std::mem::zeroed()}}}}

#[derive(Default)] #[repr(C)] pub enum FormFactor { #[default] HeadMounted=1, Handheld }
#[repr(C)] pub struct SystemGetInfo {pub ty: StructureType, pub next: *const void, pub form_factor: FormFactor}
impl Default for SystemGetInfo { fn default() -> Self { Self{ty: StructureType::SystemGetInfo, next: null(), form_factor: default()}}}

#[derive(Debug)] #[repr(C)] pub struct LUID {pub low: u32, pub high: i32}
#[repr(C)] pub struct GraphicsRequirementsD3D12 {pub ty: StructureType, pub next: *const void, pub adapter: LUID, pub min_feature_level: i32}
impl Default for GraphicsRequirementsD3D12 { fn default() -> Self { Self{ty: StructureType::GraphicsRequirementsD3D12, ..unsafe{std::mem::zeroed()}}}}

#[repr(C)] pub struct RemotingConnectInfo {pub ty: StructureType, pub next: *const void, pub remote_host: *const c_char, pub remote_port: u16, pub secure_connection: u32 }
impl Default for RemotingConnectInfo { fn default() -> Self { Self{ty: StructureType::RemotingConnectInfo, ..unsafe{std::mem::zeroed()}}}}

#[repr(C)] pub struct SessionCreateInfo {pub ty: StructureType, pub next: *const void, pub create_flags: u64, pub system: u64}
impl Default for SessionCreateInfo { fn default() -> Self { Self{ty: StructureType::SessionCreateInfo, ..unsafe{std::mem::zeroed()}}}}

pub struct GraphicsBindingD3D12 {pub ty: StructureType, pub next: *const void, pub device: *const /*wgpu_hal::dx12::native::ID3D12Device*/void, pub queue: *const /*d3d12::native::ID3D12CommandQueue*/void}
impl Default for GraphicsBindingD3D12 { fn default() -> Self { Self{ty: StructureType::GraphicsBindingD3D12, ..unsafe{std::mem::zeroed()}}}}

#[derive(Clone,Copy)] #[repr(C)] pub enum ViewConfigurationType { Mono=1, Stereo=2 }
#[repr(C)] pub struct SessionBeginInfo {pub ty: StructureType, pub next: *const void, pub primary_view_configuration_type: ViewConfigurationType}
impl Default for SessionBeginInfo { fn default() -> Self { Self{ty: StructureType::SessionBeginInfo, ..unsafe{std::mem::zeroed()}}}}

#[repr(C)] pub enum ReferenceSpaceType { View=1, Local, Stage }
#[derive(Clone,Copy)] #[repr(C)] pub struct Vector {pub x: f32, pub y: f32, pub z: f32}
#[derive(Clone,Copy)] #[repr(C)] pub struct Quaternion {pub x: f32, pub y: f32, pub z: f32, pub w: f32}
impl Default for Quaternion { fn default() -> Self { Self{x: 0., y: 0., z: 0., w: 1.} } }
#[derive(Clone,Copy)] #[repr(C)] pub struct Pose {pub orientation: Quaternion, pub position: Vector}

#[repr(C)] pub struct ReferenceSpaceCreateInfo {pub ty: StructureType, pub next: *const void, pub reference_space_type: ReferenceSpaceType, pub pose_in_reference_space: Pose}
impl Default for ReferenceSpaceCreateInfo { fn default() -> Self { Self{ty: StructureType::ReferenceSpaceCreateInfo, ..unsafe{std::mem::zeroed()}}}}

#[derive(PartialEq)] #[repr(C)] pub struct ViewConfigurationView {pub ty: StructureType, pub next: *const void, pub recommended_image_rect_width: u32, pub max_image_rect_width: u32, pub recommended_image_rect_height: u32, pub max_image_rect_height: u32, pub recommended_swapchain_sample_count: u32, pub max_swapchain_sample_count: u32}
impl Default for ViewConfigurationView { fn default() -> Self { Self{ty: StructureType::ViewConfigurationView, ..unsafe{std::mem::zeroed()}}}}

#[repr(C)] pub struct SwapchainUsageFlags(u64);
impl SwapchainUsageFlags { pub const ColorAttachment : u64 = 1<<0; pub const Sampled : u64 = 1<<5; }
#[repr(C)] pub struct SwapchainCreateInfo {pub ty: StructureType, pub next: *const void, pub create_flags: u64, pub usage_flags: u64,
    pub format: i64, pub sample_count: u32, pub width: u32, pub height: u32, pub face_count: u32, pub array_size: u32, pub mip_count: u32}
impl Default for SwapchainCreateInfo { fn default() -> Self { Self{ty: StructureType::SwapchainCreateInfo, ..unsafe{std::mem::zeroed()}}}}

#[repr(C)] pub struct EventDataBuffer {pub ty: StructureType, pub next: *const void, varying: [u8; 4000]}
impl Default for EventDataBuffer { fn default() -> Self { Self{ty: StructureType::EventDataBuffer, ..unsafe{std::mem::zeroed()}}}}

#[repr(C)] pub enum SessionState { Unknown, Idle, Ready, Synchronized, Visible, Focused, Stopping, LossPending, Exiting }
pub struct SessionStateChanged {pub ty: StructureType, pub next: *const void, pub session: Session, pub state: SessionState, pub time: i64}

#[repr(C)] pub struct SwapchainImageD3D12 {pub ty: StructureType, pub next: *mut void, pub texture: *mut /*ID3D12Resource*/void}
impl Default for SwapchainImageD3D12 { fn default() -> Self { Self{ty: StructureType::SwapchainImageD3D12, ..unsafe{std::mem::zeroed()}}}}

#[repr(C)] pub struct FrameWaitInfo {pub ty: StructureType, pub next: *const void}
impl Default for FrameWaitInfo { fn default() -> Self { Self{ty: StructureType::FrameWaitInfo, next: null()}}}

#[repr(C)] pub struct FrameState {pub ty: StructureType, pub next: *mut void, pub predicted_display_time: i64, pub predicted_display_period: i64, pub should_render: u32}
impl Default for FrameState { fn default() -> Self { Self{ty: StructureType::FrameState, ..unsafe{std::mem::zeroed()}}}}

#[repr(C)] pub struct FrameBeginInfo {pub ty: StructureType, pub next: *const void}
impl Default for FrameBeginInfo { fn default() -> Self { Self{ty: StructureType::FrameBeginInfo, next: null()}}}

#[repr(C)] pub enum EnvironmentBlendMode { Opaque=1, Additive, AlphaBlend }

#[repr(C)] pub struct FrameEndInfo {pub ty: StructureType, pub next: *const void, pub display_time: i64, pub environment_blend_mode: EnvironmentBlendMode, pub layer_count: u32, pub layers: *const *const CompositionLayerProjection}
impl Default for FrameEndInfo { fn default() -> Self { Self{ty: StructureType::FrameEndInfo, ..unsafe{std::mem::zeroed()}}}}

#[repr(C)] pub struct SwapchainImageAcquireInfo {pub ty: StructureType, pub next: *const void}
impl Default for SwapchainImageAcquireInfo { fn default() -> Self { Self{ty: StructureType::SwapchainImageAcquireInfo, next: null()}}}

#[repr(C)] pub struct SwapchainImageWaitInfo {pub ty: StructureType, pub next: *const void, pub timeout: i64}
impl Default for SwapchainImageWaitInfo { fn default() -> Self { Self{ty: StructureType::SwapchainImageWaitInfo, ..unsafe{std::mem::zeroed()}}}}

#[repr(C)] pub struct SwapchainImageReleaseInfo {pub ty: StructureType, pub next: *const void}
impl Default for SwapchainImageReleaseInfo { fn default() -> Self { Self{ty: StructureType::SwapchainImageReleaseInfo, next: null()}}}

#[repr(C)] pub struct ViewLocateInfo {pub ty: StructureType, pub next: *const void, pub view_configuration_type: ViewConfigurationType, pub display_time: i64, pub space: Space}
impl Default for ViewLocateInfo { fn default() -> Self { Self{ty: StructureType::ViewLocateInfo, ..unsafe{std::mem::zeroed()}}}}

#[repr(C)] pub struct ViewState {pub ty: StructureType, pub next: *mut void, pub view_state_flags: u32}
impl Default for ViewState { fn default() -> Self { Self{ty: StructureType::ViewState, ..unsafe{std::mem::zeroed()}}}}

#[derive(Clone,Copy)] #[repr(C)] pub struct Fov {pub angle_left: f32, pub angle_right: f32, pub angle_up: f32, pub angle_down: f32}

#[repr(C)] pub struct View {pub ty: StructureType, pub next: *mut void, pub pose: Pose, pub fov: Fov}
impl Default for View { fn default() -> Self { Self{ty: StructureType::View, ..unsafe{std::mem::zeroed()}}}}

#[repr(C)] pub struct Offset2D {pub x: i32, pub y: i32}
#[repr(C)] pub struct Extent2D {pub width: i32, pub height: i32}
#[repr(C)] pub struct Rect2D {pub offset: Offset2D, pub extent: Extent2D}

#[repr(C)] pub struct SwapchainSubImage {pub swapchain: Swapchain, pub image_rect: Rect2D, pub image_array_index: u32}

#[repr(C)] pub struct CompositionLayerProjectionView {pub ty: StructureType, pub next: *const void, pub pose: Pose, pub fov: Fov, pub sub_image: SwapchainSubImage}
impl Default for CompositionLayerProjectionView { fn default() -> Self { Self{ty: StructureType::CompositionLayerProjectionView, ..unsafe{std::mem::zeroed()}}}}

#[repr(C)] pub struct CompositionLayerProjection {pub ty: StructureType, pub next: *const void, pub layer_flags: u32, pub space: Space, pub view_count: u32, pub views: *const CompositionLayerProjectionView}
impl Default for CompositionLayerProjection { fn default() -> Self { Self{ty: StructureType::CompositionLayerProjection, ..unsafe{std::mem::zeroed()}}}}
