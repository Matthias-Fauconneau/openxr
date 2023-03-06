#![feature(default_free_fn, generic_arg_infer, raw_dylib)]#![allow(non_snake_case)]
use std::{default::default, ptr::null};
mod xr; use xr::{*, Result::Success};

fn main() {
    /*//  ffplay -headers "Authorization: Basic ZA/" 
    let response = attohttpc::get("https://192.168.0.101/api/holographic/stream/live_high.mp4?holo=false&pv=true&mic=false&loopback=false&RenderFromCamera=false")
        .basic_auth("ethnsel@gmail.com",Some("")).send();
    assert!(response.is_success());*/
    use ffmpeg::format::{input, Pixel};
    use ffmpeg::media::Type;
    use ffmpeg::software::scaling::{context::Context, flag::Flags};
    use ffmpeg::util::frame::video::Video;
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;
    ffmpeg::init().unwrap();
    let mut ictx = input("https://192.168.0.101/api/holographic/stream/live_high.mp4?holo=false&pv=true&mic=false&loopback=false&RenderFromCamera=false").unwrap();
    let input = ictx.streams().best(Type::Video).unwrap();
    let video_stream_index = input.index();

        let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
        let mut decoder = context_decoder.decoder().video()?;

        let mut scaler = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            Flags::BILINEAR,
        )?;

        let mut frame_index = 0;

        let mut receive_and_process_decoded_frames =
            |decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
                let mut decoded = Video::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let mut rgb_frame = Video::empty();
                    scaler.run(&decoded, &mut rgb_frame)?;
                    save_file(&rgb_frame, frame_index).unwrap();
                    frame_index += 1;
                }
                Ok(())
            };

        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                decoder.send_packet(&packet)?;
                receive_and_process_decoded_frames(&mut decoder)?;
            }
        }
        decoder.send_eof()?;
        receive_and_process_decoded_frames(&mut decoder)?;
    }

    let ref mut runtime_request = NegotiateRuntimeRequest::default();
    assert!(unsafe{negotiate_loader_runtime_interface(&NegotiateLoaderInfo::default() as *const _, runtime_request as *mut _)} == Success);
    let get_instance_proc_addr = runtime_request.get_instance_proc_addr.unwrap();

    /*let mut enumerate_instance_extension_properties : Option<extern "C" fn(layer_name: *const char, property_capacity_input: u32, property_count_output: *mut u32, properties: *mut ExtensionProperties)->Result> = None;
    get_instance_proc_addr(0, b"xrEnumerateInstanceExtensionProperties\0" as *const _ as *const _, &mut enumerate_instance_extension_properties as *mut _ as *mut _);
    let enumerate_instance_extension_properties = enumerate_instance_extension_properties.unwrap();
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
        extension_count: 2, extension_names: &[b"XR_KHR_D3D12_enable\0" as *const u8, b"XR_MSFT_holographic_remoting\0" as *const _] as *const _, ..default()}, &mut xr), Success);
    
    let mut get_system : Option<extern "C" fn(instance: Instance, get_info: *const SystemGetInfo, *mut u64)->Result> = None;
    get_instance_proc_addr(xr, b"xrGetSystem\0" as *const _, &mut get_system as *mut _ as *mut _);
    let get_system = get_system.unwrap();

    let mut system = 0;
    assert_eq!(get_system(xr, &SystemGetInfo{form_factor: FormFactor::HeadMounted, ..default()}, &mut system), Success);

    let mut get_D3D12_graphics_requirements : Option<extern "C" fn(instance: Instance, system: u64, requirements: *mut GraphicsRequirementsD3D12)->Result> = None;
    get_instance_proc_addr(xr, b"xrGetD3D12GraphicsRequirementsKHR\0" as *const _, &mut get_D3D12_graphics_requirements as *mut _ as *mut _);
    let get_D3D12_graphics_requirements = get_D3D12_graphics_requirements.unwrap();

    let mut requirements = default();
    assert_eq!(get_D3D12_graphics_requirements(xr, system, &mut requirements), Success); // Microsoft Holographic Remoting implementation fails to create session without this call
    
    let remote_host = std::ffi::CString::new(std::env::args().skip(1).next().as_ref().map(|s| s.as_str()).unwrap_or("192.168.0.101")).unwrap();
    println!("Hololens: {remote_host:?}");

    let mut remoting_connect : Option<extern "C" fn(instance: Instance, system: u64, info: *const RemotingConnectInfo)->Result> = None;
    get_instance_proc_addr(xr, b"xrRemotingConnectMSFT\0" as *const _, &mut remoting_connect as *mut _ as *mut _);
    let remoting_connect = remoting_connect.unwrap();

    assert_eq!(remoting_connect(xr, system, &RemotingConnectInfo{remote_host: remote_host.as_ptr(), remote_port: 8265, ..default()}), Success);

    use pollster::FutureExt as _;
    let adapter = wgpu::Instance::new(
        wgpu::InstanceDescriptor{backends: wgpu::Backends::DX12, dx12_shader_compiler: default()/*wgpu::Dx12Compiler::Dxc{dxil_path: None, dxc_path: None}*/}
    ).request_adapter(&default()).block_on().unwrap();
    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor{features: wgpu::Features::TEXTURE_FORMAT_16BIT_NORM/*|wgpu::Features::MULTIVIEW*/, ..default()},
        None).block_on().unwrap();
    use wgpu_hal::api::Dx12;
    let session = unsafe {
        let (device, queue) = device.as_hal::<Dx12, _, _>(|device| (device.unwrap().raw_device().as_mut_ptr(), device.unwrap().raw_queue().as_mut_ptr()));
        
        let mut create_session : Option<extern "C" fn(instance: Instance, info: *const SessionCreateInfo, session: *mut Session)->Result> = None;
        get_instance_proc_addr(xr, b"xrCreateSession\0" as *const _, &mut create_session as *mut _ as *mut _);
        let create_session = create_session.unwrap();

        let mut session = 0;
        assert_eq!(create_session(xr, &SessionCreateInfo{next: &GraphicsBindingD3D12{device, queue, ..default()}, system, create_flags: 0, ..default()}, &mut session), Success);
        session
    };
    let vert_shader = device.create_shader_module(wgpu::include_wgsl!("fullscreen.wgsl"));
    let frag_shader = device.create_shader_module(wgpu::include_wgsl!("sample.wgsl"));
    let ref layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label: None, entries: &[
        wgpu::BindGroupLayoutEntry{binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture{multisampled: false, view_dimension: wgpu::TextureViewDimension::D2, sample_type: wgpu::TextureSampleType::Float{filterable: true}}, count: None},
        wgpu::BindGroupLayoutEntry{binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None}
    ]});
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label: None, bind_group_layouts: &[layout], push_constant_ranges: &[]});

    let mut enumerate_view_configuration_views : Option<extern "C" fn(instance: Instance, system: u64, ty: ViewConfigurationType, capacity: u32, len: *const u32, views: *const ViewConfigurationView)->Result> = None;
    get_instance_proc_addr(xr, b"xrEnumerateViewConfigurationViews\0" as *const _, &mut enumerate_view_configuration_views as *mut _ as *mut _);
    let enumerate_view_configuration_views = enumerate_view_configuration_views.unwrap();

    fn array<T:Default>(capacity_len_buffer: impl Fn(u32, &mut u32, *mut T)->Result) -> Box<[T]> {
        let mut len = 0;
        assert_eq!(capacity_len_buffer(0, &mut len, std::ptr::null_mut()), Success);
        let mut buffer = std::iter::from_fn(|| Some(T::default())).take(len as usize).collect::<Box<_>>();
        let mut len = 0;
        assert_eq!(capacity_len_buffer(buffer.len() as u32, &mut len, buffer.as_mut_ptr()), Success);
        assert_eq!(buffer.len(), len as usize);
        buffer
    }

    let view_configuration_type = ViewConfigurationType::Stereo;
    let views = array(|capacity, len, buffer| enumerate_view_configuration_views(xr, system, view_configuration_type, capacity, len, buffer));
    assert_eq!(views.len(), 2);
    if views.len() == 2 { assert!(views[0] == views[1]); } else { assert_eq!(views.len(), 1); }
    
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState{module: &vert_shader, entry_point: "fullscreen_vertex_shader", buffers: &[]},
        fragment: Some(wgpu::FragmentState{module: &frag_shader, entry_point: "sample_fragment_shader", targets: &[Some(wgpu::TextureFormat::Rgba8UnormSrgb.into())]}),
        primitive: default(),
        depth_stencil: None,
        multisample: default(),
        multiview: None/*(views.len() > 1).then(|| (views.len() as u32).try_into().ok().unwrap())*/,
    });
    let ViewConfigurationView{recommended_image_rect_width: width, recommended_image_rect_height: height, ..} = views[0];

    let mut create_swapchain : Option<extern "C" fn(session: Session, create_info: *const SwapchainCreateInfo, swapchain: *mut Swapchain)->Result> = None;
    get_instance_proc_addr(xr, b"xrCreateSwapchain\0" as *const _, &mut create_swapchain as *mut _ as *mut _);
    let create_swapchain = create_swapchain.unwrap();

    let mut swapchain = 0;
    assert_eq!(create_swapchain(session, &SwapchainCreateInfo{
        create_flags: 0,
        usage_flags: SwapchainUsageFlags::ColorAttachment | SwapchainUsageFlags::Sampled,
        format: winapi::shared::dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM as _,
        sample_count: 1,
        width, height,
        face_count: 1,
        array_size: 1,//views.len() as u32,
        mip_count: 1,
        ..default()
    }, &mut swapchain), Success);

    let mut enumerate_swapchain_images : Option<extern "C" fn(swapchain: Swapchain, capacity: u32, len: *mut u32, images: *mut SwapchainImageD3D12)->Result> = None;
    get_instance_proc_addr(xr, b"xrEnumerateSwapchainImages\0" as *const _, &mut enumerate_swapchain_images as *mut _ as *mut _);
    let enumerate_swapchain_images = enumerate_swapchain_images.unwrap();

    let images = array(|capacity, len, buffer| enumerate_swapchain_images(swapchain, capacity, len, buffer)).into_iter().map(|image| {
        let format = wgpu::TextureFormat::Rgba8UnormSrgb;
        let desc = wgpu::TextureDescriptor {label: None, size: wgpu::Extent3d{width, height, depth_or_array_layers: 1/*views.len() as u32*/}, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format, usage: wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[format]};
        unsafe{device.create_texture_from_hal::<Dx12>(<Dx12 as wgpu_hal::Api>::Device::texture_from_raw(d3d12::Resource::from_raw(image.texture.cast()), desc.format, desc.dimension, desc.size, desc.mip_level_count, desc.sample_count), &desc)}
    }).collect::<Box<_>>();

    let mut create_reference_space : Option<extern "C" fn(session: Session, info: *const ReferenceSpaceCreateInfo, space: *mut Space)->Result> = None;
    get_instance_proc_addr(xr, b"xrCreateReferenceSpace\0" as *const _, &mut create_reference_space as *mut _ as *mut _);
    let create_reference_space = create_reference_space.unwrap();

    let mut space = default();
    assert_eq!(create_reference_space(session, &default(), &mut space), Success);

    println!("Laptop: {}", local_ip_address::local_ip().unwrap());
    let ref camera = (std::env::args().skip(2).next().map(|interface| interface.parse().unwrap()).unwrap_or(std::net::Ipv4Addr::UNSPECIFIED),6666);
    let camera = std::net::UdpSocket::bind(camera).unwrap();

    let mut poll_event : Option<extern "C" fn(instance: Instance, event_data: *mut EventDataBuffer)->Result> = None;
    get_instance_proc_addr(xr, b"xrPollEvent\0" as *const _, &mut poll_event as *mut _ as *mut _);
    let poll_event = poll_event.unwrap();
    let mut wait_frame : Option<extern "C" fn(session: Session, frame_wait_info: *const FrameWaitInfo, frame_state: *mut FrameState)->Result> = None;
    get_instance_proc_addr(xr, b"xrWaitFrame\0" as *const _, &mut wait_frame as *mut _ as *mut _);
    let wait_frame = wait_frame.unwrap();
    let mut begin_frame : Option<extern "C" fn(session: Session, frame_begin_info: *const FrameBeginInfo)->Result> = None;
    get_instance_proc_addr(xr, b"xrBeginFrame\0" as *const _, &mut begin_frame as *mut _ as *mut _);
    let begin_frame = begin_frame.unwrap();
    let mut end_frame : Option<extern "C" fn(session: Session, frame_end_info: *const FrameEndInfo)->Result> = None;
    get_instance_proc_addr(xr, b"xrEndFrame\0" as *const _, &mut end_frame as *mut _ as *mut _);
    let end_frame = end_frame.unwrap();
    let mut acquire_swapchain_image : Option<extern "C" fn(swapchain: Swapchain, acquire_info: *const SwapchainImageAcquireInfo, index: *mut u32)->Result> = None;
    get_instance_proc_addr(xr, b"xrAcquireSwapchainImage\0" as *const _, &mut acquire_swapchain_image as *mut _ as *mut _);
    let acquire_swapchain_image = acquire_swapchain_image.unwrap();
    let mut wait_swapchain_image : Option<extern "C" fn(swapchain: Swapchain, wait_info: *const SwapchainImageWaitInfo)->Result> = None;
    get_instance_proc_addr(xr, b"xrWaitSwapchainImage\0" as *const _, &mut wait_swapchain_image as *mut _ as *mut _);
    let wait_swapchain_image = wait_swapchain_image.unwrap();

    loop {       
        loop {
            let mut event = EventDataBuffer::default();
            assert_eq!(poll_event(xr, &mut event), Success);
            match event.ty {
                StructureType::InstanceLossPending => { return /*Ok(())*/; }
                StructureType::SessionStateChanged => {use SessionState::*; match unsafe{&*(&event as *const _ as *const SessionStateChanged)}.state {
                    Idle|Synchronized|Visible|Focused => {},
                    Ready => {
                        let mut begin_session : Option<extern "C" fn(session: Session, begin_info: *const SessionBeginInfo)->Result> = None;
                        get_instance_proc_addr(xr, b"xrBeginSession\0" as *const _, &mut begin_session as *mut _ as *mut _);
                        let begin_session = begin_session.unwrap();

                        assert_eq!(begin_session(session, &SessionBeginInfo{primary_view_configuration_type: view_configuration_type, ..default()}), Success);
                        println!("Ready");
                    }
                    Stopping => {
                        let mut end_session : Option<extern "C" fn(session: Session)->Result> = None;
                        get_instance_proc_addr(xr, b"xrEndSession\0" as *const _, &mut end_session as *mut _ as *mut _);
                        let end_session = end_session.unwrap();

                        assert_eq!(end_session(session), Success);

                        println!("Stopping");
                        return /*Ok(())*/;
                    }
                    Exiting|LossPending => { println!("Exiting|LossPending"); return /*Ok(())*/; }
                    e => panic!("{:?}", e as u32)
                }}
                StructureType::RemotingConnected => println!("RemotingConnected"),
                StructureType::RemotingDisconnected => println!("RemotingDisconnected"),
                StructureType::RemotingTimestampConversionReady => {},
                StructureType::EventDataBuffer => break,
                e => {panic!("{:?}", e as u32)}
            }
        }


        let mut frame_state = default();
        assert_eq!(wait_frame(session, &default(), &mut frame_state), Success);


        assert_eq!(begin_frame(session, &default()), Success);

        let environment_blend_mode = EnvironmentBlendMode::Additive;
        if frame_state.should_render == 0 {
            assert_eq!(end_frame(session, &FrameEndInfo{
                environment_blend_mode,
                display_time: frame_state.predicted_display_time,
                layer_count: 0,
                layers: null(),
                ..default()
            }), Success);
            continue;
        }

        let mut index = 0;
        assert_eq!(acquire_swapchain_image(swapchain, &default(), &mut index), Success);

        
        assert_eq!(wait_swapchain_image(swapchain, &SwapchainImageWaitInfo{timeout: i64::MAX, ..default()}), Success);

        let mut image = vec![0u16; 160*120];
        let (len, _sender) = camera.recv_from(bytemuck::cast_slice_mut(&mut image)).unwrap();
        assert!(len == image.len()*std::mem::size_of::<u16>());
        let min = *image.iter().min().unwrap();
        let max = *image.iter().max().unwrap();
        for value in image.iter_mut() { *value = ((*value - min) as u32 * ((1<<16)-1) / (max - min) as u32) as u16; } // Remap to full range. FIXME: does linear output get gamma compressed or wrongly interpreted as sRGB ?
        let size = wgpu::Extent3d{width: 160, height: 120, depth_or_array_layers: 1};
        let format = wgpu::TextureFormat::R16Unorm;
        let gpu_image = device.create_texture(&wgpu::TextureDescriptor{size, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
                format, usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, label: None, view_formats: &[format]});
        queue.write_texture(wgpu::ImageCopyTexture{texture: &gpu_image, mip_level: 0,origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All},
                    bytemuck::cast_slice(&image), wgpu::ImageDataLayout {offset: 0, bytes_per_row: std::num::NonZeroU32::new(2 * size.width), rows_per_image: std::num::NonZeroU32::new(size.height)},
                    size);
        let image_view = gpu_image.create_view(&default());
        let sampler = device.create_sampler(&default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{label: None, layout, entries: &[
                    wgpu::BindGroupEntry{binding: 0, resource: wgpu::BindingResource::TextureView(&image_view)},
                    wgpu::BindGroupEntry{binding: 1, resource: wgpu::BindingResource::Sampler(&sampler)}]});

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {let ref view = images[index as usize].create_view(&wgpu::TextureViewDescriptor{base_array_layer: 0, array_layer_count: 1/*(views.len() as u32)*/.try_into().ok(), ..default()});
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label: None,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment{view, resolve_target: None, ops: wgpu::Operations{load: wgpu::LoadOp::Clear(wgpu::Color::GREEN), store: true}})], depth_stencil_attachment: None});
        pass.set_pipeline(&render_pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.draw(0..3, 0..1);}
        queue.submit(Some(encoder.finish()));

        let mut release_swapchain_image : Option<extern "C" fn(swapchain: Swapchain, release_info: *const SwapchainImageReleaseInfo)->Result> = None;
        get_instance_proc_addr(xr, b"xrReleaseSwapchainImage\0" as *const _, &mut release_swapchain_image as *mut _ as *mut _);
        let release_swapchain_image = release_swapchain_image.unwrap();

        assert_eq!(release_swapchain_image(swapchain, &default()), Success);

        let mut locate_views : Option<extern "C" fn(session: Session, view_locate_info: *const ViewLocateInfo, view_state: *mut ViewState, capacity: u32, len: *mut u32, views: *mut View)->Result> = None;
        get_instance_proc_addr(xr, b"xrLocateViews\0" as *const _, &mut locate_views as *mut _ as *mut _);
        let locate_views = locate_views.unwrap();

        let views = array(|capacity, len, buffer| {
            let mut view_state = default();
            locate_views(session, &ViewLocateInfo{view_configuration_type, display_time: frame_state.predicted_display_time, space, ..default()}, &mut view_state, capacity, len, buffer)
        });

        assert_eq!(end_frame(session, &FrameEndInfo{
            environment_blend_mode,
            display_time: frame_state.predicted_display_time,
            layer_count: 1,
            layers: &[
                &CompositionLayerProjection{space, layer_flags: 0, view_count: 2, views: &[0,1].map(|i|
                    CompositionLayerProjectionView{
                        pose: views[i].pose,
                        fov: views[i].fov,
                        sub_image: SwapchainSubImage{
                            swapchain,
                            image_rect: Rect2D{offset: Offset2D{x: 0, y: 0}, extent: Extent2D{width: width as i32, height: height as i32}},
                            image_array_index: 0,//i as u32,
                        },
                        ..default()
                    }) as *const CompositionLayerProjectionView,
                    ..default()
                } as *const CompositionLayerProjection
            ] as *const *const CompositionLayerProjection,
            ..default()
        }), Success);
    }
}
