#![feature(default_free_fn, generic_arg_infer)]
#![allow(non_upper_case_globals,non_snake_case,dead_code)]

#[link(name="Microsoft.Holographic.AppRemoting.OpenXr")]
extern "C" { #[link_name="xrNegotiateLoaderRuntimeInterface"] fn negotiate_loader_runtime_interface(loader_info: *const NegotiateLoaderInfo, runtime_request: *mut NegotiateRuntimeRequest) -> Result; }
use std::{default::default, ptr::null, mem::size_of, ffi::{c_void as void, c_char}}; #[allow(non_camel_case_types)] type char = u8;
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

#[derive(Debug)] #[repr(C)] enum StructureType { Unknown = 0, ApiLayerProperties, ExtensionProperties, InstanceCreateInfo, SystemGetInfo,
    GraphicsRequirementsD3D12 = 1000028002,
    RemotingConnectInfo = 1000065001,
}
//struct LoaderInitInfoBaseHeader { struct_type: StructureType, next: *const c_void }
#[derive(Debug)] #[repr(C)] struct ExtensionProperties{ty: StructureType, next: *const void, extension_name: [u8; 128], extension_version: u32}
impl Default for ExtensionProperties { fn default() -> Self { Self{ty: StructureType::ExtensionProperties, next: null(), extension_name: [0; _], extension_version: 0}}}

#[repr(C)] struct ApplicationInfo { name: [u8; 128], version: u32, engine: [u8; 128], engine_version: u32, api: u64}
#[repr(C)] struct InstanceCreateInfo{ty: StructureType, next: *const void, create_flags: u64, application_info: ApplicationInfo,
    api_layer_count: u32, api_layer_names: *const *const u8, extension_count: u32, extension_names: *const *const char}
impl Default for InstanceCreateInfo { fn default() -> Self { Self{ty: StructureType::InstanceCreateInfo, ..unsafe{std::mem::zeroed()}}}}

#[derive(Default)] #[repr(C)] enum FormFactor { #[default] HeadMounted=1, Handheld }
#[repr(C)] struct SystemGetInfo{ty: StructureType, next: *const void, form_factor: FormFactor}
impl Default for SystemGetInfo { fn default() -> Self { Self{ty: StructureType::SystemGetInfo, next: null(), form_factor: default()}}}

#[repr(C)] struct LUID {low: u32, high: i32}
#[repr(C)] struct GraphicsRequirementsD3D12{ty: StructureType, next: *const void, adapter: LUID, min_feature_level: i32}
impl Default for GraphicsRequirementsD3D12 { fn default() -> Self { Self{ty: StructureType::GraphicsRequirementsD3D12, ..unsafe{std::mem::zeroed()}}}}

#[repr(C)] struct RemotingConnectInfo{ty: StructureType, next: *const void, remote_host: *const c_char, remote_port: u16, secure_connection: u32 }
impl Default for RemotingConnectInfo { fn default() -> Self { Self{ty: StructureType::RemotingConnectInfo, ..unsafe{std::mem::zeroed()}}}}

fn main() {
    let ref mut runtime_request = NegotiateRuntimeRequest::default();
    assert!(unsafe{negotiate_loader_runtime_interface(&NegotiateLoaderInfo::default() as *const _, runtime_request as *mut _)} == Result::Success);
    let get_instance_proc_addr = runtime_request.get_instance_proc_addr.unwrap();
    /*let mut enumerate_instance_extension_properties : Option<extern "C" fn(layer_name: *const char, property_capacity_input: u32, property_count_output: *mut u32, properties: *mut ExtensionProperties)->Result> = None;
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
    let any = |name:&[u8]| extensions.iter().any(|ExtensionProperties{extension_name,..}| &extension_name[0..name.len()] == name);
    assert!(any(b"XR_KHR_D3D12_enable\0") && any(b"XR_MSFT_holographic_remoting\0"));*/

    let mut create_instance : Option<extern "C" fn(create_info: *const InstanceCreateInfo, *mut Instance)->Result> = None;
    get_instance_proc_addr(0, b"xrCreateInstance\0" as *const _, &mut create_instance as *mut _ as *mut _);
    let create_instance = create_instance.unwrap();

    let mut xr = 0;
    assert_eq!(create_instance(&InstanceCreateInfo{application_info: ApplicationInfo{
        name: {let mut s = [0; _]; s[..3].copy_from_slice(b"IR\0"); s}, version: 0,
        engine: {let mut s = [0; _]; s[..5].copy_from_slice(b"Rust\0"); s}, engine_version: 0,
        api: 0x1_0000_0000_0025 },
        extension_count: 2, extension_names: &[b"XR_KHR_D3D12_enable\0" as *const u8, b"XR_MSFT_holographic_remoting\0" as *const _] as *const _, ..default()}, &mut xr), Result::Success);
    dbg!(xr);

    let mut get_system : Option<extern "C" fn(instance: Instance, get_info: *const SystemGetInfo, *mut u64)->Result> = None;
    get_instance_proc_addr(xr, b"xrGetSystem\0" as *const _, &mut get_system as *mut _ as *mut _);
    let get_system = get_system.unwrap();

    let mut system = 0;
    assert_eq!(get_system(xr, &SystemGetInfo{form_factor: FormFactor::HeadMounted, ..default()}, &mut system), Result::Success);

    let mut get_D3D12_graphics_requirements : Option<extern "C" fn(instance: Instance, system: u64, requirements: *mut GraphicsRequirementsD3D12)->Result> = None;
    dbg!("xrGetD3D12GraphicsRequirementsKHR");
    get_instance_proc_addr(xr, b"xrGetD3D12GraphicsRequirementsKHR\0" as *const _, &mut get_D3D12_graphics_requirements as *mut _ as *mut _);
    dbg!("xrGetD3D12GraphicsRequirementsKHR");
    let get_D3D12_graphics_requirements = get_D3D12_graphics_requirements.unwrap();
    dbg!("xrGetD3D12GraphicsRequirementsKHR");

    let mut requirements = default();
    dbg!("get_D3D12_graphics_requirements");
    get_D3D12_graphics_requirements(xr, system, &mut requirements); // Microsoft Holographic Remoting implementation fails to create session without this call

    dbg!("remoting_host");
    let remote_host = std::ffi::CString::new(std::env::args().skip(1).next().as_ref().map(|s| s.as_str()).unwrap_or("192.168.0.101")).unwrap();

    let mut remoting_connect : Option<extern "C" fn(instance: Instance, system: u64, info: *const RemotingConnectInfo)->Result> = None;
    get_instance_proc_addr(0, b"xrRemotingConnectMSFT\0" as *const _, &mut remoting_connect as *mut _ as *mut _);
    let remoting_connect = remoting_connect.unwrap();

    dbg!("remoting_connect");
    remoting_connect(xr, system, &RemotingConnectInfo{remote_host: remote_host.as_ptr(), remote_port: 8265, ..default()});
    dbg!("remoting_connect: OK");
    /*use pollster::FutureExt as _;
    let adapter = wgpu::Instance::new(wgpu::Backend::Dx12.into()).request_adapter(&default()).block_on().unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor{features: wgpu::Features::TEXTURE_FORMAT_16BIT_NORM|wgpu::Features::MULTIVIEW, ..default()}, None).block_on().unwrap();
    use wgpu_hal::api::Dx12;
    let (session, mut frame_wait, mut frame_stream) = unsafe {
        let (device, queue) = device.as_hal::<Dx12, _, _>(|device| (device.unwrap().raw_device().as_mut_ptr(), device.unwrap().raw_queue().as_mut_ptr()));
        xr.create_session::<xr::D3D12>(system, &xr::d3d12::SessionCreateInfo{device: device.cast(), queue: queue.cast()})
    }?;
    let vert_shader = device.create_shader_module(wgpu::include_wgsl!("fullscreen.wgsl"));
    let frag_shader = device.create_shader_module(wgpu::include_wgsl!("sample.wgsl"));
    let ref layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label: None, entries: &[
        wgpu::BindGroupLayoutEntry{binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture{multisampled: false, view_dimension: wgpu::TextureViewDimension::D2, sample_type: wgpu::TextureSampleType::Float{filterable: true}}, count: None},
        wgpu::BindGroupLayoutEntry{binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None}
    ]});
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label: None, bind_group_layouts: &[layout], push_constant_ranges: &[]});
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let view_type = xr::ViewConfigurationType::PRIMARY_STEREO;
    let views = xr.enumerate_view_configuration_views(system, view_type)?;
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState{module: &vert_shader, entry_point: "fullscreen_vertex_shader", buffers: &[]},
        fragment: Some(wgpu::FragmentState{module: &frag_shader, entry_point: "sample_fragment_shader", targets: &[Some(format.into())]}),
        primitive: default(),
        depth_stencil: None,
        multisample: default(),
        multiview: None//(views.len() > 1).then(|| (views.len() as u32).try_into().ok().unwrap()),
    });
    if views.len() == 2 { assert_eq!(views[0], views[1]); } else { assert!(views.len()==1); }
    let xr::ViewConfigurationView{recommended_image_rect_width: width, recommended_image_rect_height: height, ..} = views[0];
    let mut swapchain = session.create_swapchain(&xr::SwapchainCreateInfo{
        create_flags: xr::SwapchainCreateFlags::EMPTY,
        usage_flags: xr::SwapchainUsageFlags::COLOR_ATTACHMENT | xr::SwapchainUsageFlags::SAMPLED,
        format: winapi::shared::dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
        sample_count: 1,
        width, height,
        face_count: 1,
        array_size: 1,//views.len() as u32,
        mip_count: 1,
    })?;
    let images = swapchain.enumerate_images()?.into_iter().map(|image| {
        let desc = wgpu::TextureDescriptor {label: None, size: wgpu::Extent3d{width, height, depth_or_array_layers: 1/*views.len() as u32*/}, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format, usage: wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING};
        unsafe{device.create_texture_from_hal::<Dx12>(<Dx12 as wgpu_hal::Api>::Device::texture_from_raw(d3d12::Resource::from_raw(image.cast()), desc.format, desc.dimension, desc.size, desc.mip_level_count, desc.sample_count), &desc)}
    }).collect::<Box<_>>();
    let reference_space = session.create_reference_space(xr::ReferenceSpaceType::VIEW, xr::Posef::IDENTITY)?;
    println!("{}", local_ip_address::local_ip()?);
    let ref camera = (std::env::args().skip(2).next().map(|interface| interface.parse().unwrap()).unwrap_or(std::net::Ipv4Addr::UNSPECIFIED),6666);
    let camera = std::net::UdpSocket::bind(camera)?;
    loop {
        let mut event_storage = xr::EventDataBuffer::new();
        while let Some(event) = xr.poll_event(&mut event_storage)? {
            use xr::Event::*; match event {
                SessionStateChanged(e) => {use xr::SessionState as o; match e.state() {
                    o::IDLE|o::SYNCHRONIZED|o::VISIBLE|o::FOCUSED => {},
                    o::READY => { session.begin(view_type)?; println!("Ready"); }
                    o::STOPPING => { session.end()?; println!("Stopping"); return Ok(()); }
                    o::EXITING|o::LOSS_PENDING => { println!("Exiting|LossPending"); return Ok(()); }
                    _ => panic!("{:?}", e.state())
                }}
                InstanceLossPending(_) => { return Ok(()); }
                _ => {dbg!()}
            }
        }
        let frame_state = frame_wait.wait()?;
        frame_stream.begin()?;
        let environment_blend_mode = xr::EnvironmentBlendMode::ADDITIVE;
        if !frame_state.should_render { dbg!(); frame_stream.end(frame_state.predicted_display_time, environment_blend_mode, &[])?; continue; }
        let index = swapchain.acquire_image()? as usize;
        swapchain.wait_image(xr::Duration::INFINITE)?;
        let mut image = vec![0u16; 160*120];
        println!("receive");
        let (len, _sender) = camera.recv_from(bytemuck::cast_slice_mut(&mut image))?;
        println!("received");
        assert!(len == image.len()*std::mem::size_of::<u16>());
        let min = *image.iter().min().unwrap();
        let max = *image.iter().max().unwrap();
        for value in image.iter_mut() { *value = (*value as u32 * ((1<<16)-1) / (max - min) as u32) as u16; } // Remap to full range. FIXME: does linear output get gamma compressed or wrongly interpreted as sRGB ?
        let size = wgpu::Extent3d{width: 160, height: 120, depth_or_array_layers: 1};
        let gpu_image = device.create_texture(&wgpu::TextureDescriptor{size, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R16Unorm, usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, label: None});
        queue.write_texture(wgpu::ImageCopyTexture{texture: &gpu_image, mip_level: 0,origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All},
                    bytemuck::cast_slice(&image), wgpu::ImageDataLayout {offset: 0, bytes_per_row: std::num::NonZeroU32::new(2 * size.width), rows_per_image: std::num::NonZeroU32::new(size.height)},
                    size);
        let image_view = gpu_image.create_view(&default());
        let sampler = device.create_sampler(&default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{label: None, layout, entries: &[
                    wgpu::BindGroupEntry{binding: 0, resource: wgpu::BindingResource::TextureView(&image_view)},
                    wgpu::BindGroupEntry{binding: 1, resource: wgpu::BindingResource::Sampler(&sampler)}]});

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {let ref view = images[index].create_view(&wgpu::TextureViewDescriptor{base_array_layer: 0, array_layer_count: 1/*(views.len() as u32)*/.try_into().ok(), ..default()});
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label: None,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment{view, resolve_target: None, ops: wgpu::Operations{load: wgpu::LoadOp::Clear(wgpu::Color::GREEN), store: true}})], depth_stencil_attachment: None});
        pass.set_pipeline(&render_pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.draw(0..3, 0..1);}
        queue.submit(Some(encoder.finish()));
        swapchain.release_image()?;
        let (_, views) = session.locate_views(view_type, frame_state.predicted_display_time, &reference_space)?;
        frame_stream.end(frame_state.predicted_display_time, environment_blend_mode, &[&xr::CompositionLayerProjection::new().space(&reference_space).views(&[0,1].map(|i|
            xr::CompositionLayerProjectionView::new().pose(views[i].pose).fov(views[i].fov).sub_image(xr::SwapchainSubImage::new().swapchain(&swapchain).image_array_index(/*i as u32*/0).image_rect(xr::Rect2Di {offset: xr::Offset2Di{x: 0, y: 0}, extent: xr::Extent2Di{width: width as i32, height: height as i32}}))))])?;
    }*/
}
