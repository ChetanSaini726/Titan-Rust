extern crate sdl2;
extern crate vulkano;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::VkInstance;
use std::ffi::CString;
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::swapchain::Surface;
use vulkano::{Handle, Version, VulkanObject};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let display_res = 10;
    let width = video_subsystem.display_mode(0, display_res).unwrap().w;
    let height = video_subsystem.display_mode(0, display_res).unwrap().h;
    let hz = video_subsystem
        .display_mode(0, display_res)
        .unwrap()
        .refresh_rate;
    println!("{}x{}x{}", width, height, hz);

    let window = video_subsystem
        .window("Titan Engine", width as u32, height as u32)
        .vulkan()
        .build()
        .unwrap();

    let instance_extensions_strings: Vec<CString> = window
        .vulkan_instance_extensions()
        .unwrap()
        .iter()
        .map(|&v| CString::new(v).unwrap())
        .collect();
    let instance_extension =
        InstanceExtensions::from(instance_extensions_strings.iter().map(AsRef::as_ref));
    let instance = Instance::new(None, Version::V1_2, &instance_extension, None).unwrap();
    let surface_handle = window
        .vulkan_create_surface(instance.internal_object().as_raw() as VkInstance)
        .unwrap();
    let _surface = unsafe {
        Surface::from_raw_surface(instance, Handle::from_raw(surface_handle), window.context())
    };

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
