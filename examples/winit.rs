#![allow(unused)]
use muda::{
    accelerator::{Accelerator, Code, Modifiers},
    menu_event_receiver,
    predefined::{self, AboutMetadata},
    CheckMenuItem, Menu, Submenu, TextMenuItem,
};
#[cfg(target_os = "macos")]
use winit::platform::macos::EventLoopBuilderExtMacOS;
#[cfg(target_os = "windows")]
use winit::platform::windows::{EventLoopBuilderExtWindows, WindowExtWindows};
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};

fn main() {
    let mut event_loop_builder = EventLoopBuilder::new();

    let mut menu_bar = Menu::new();

    #[cfg(target_os = "windows")]
    {
        let menu_bar_c = menu_bar.clone();
        event_loop_builder.with_msg_hook(move |msg| {
            use windows_sys::Win32::UI::WindowsAndMessaging::{TranslateAcceleratorW, MSG};
            unsafe {
                let msg = msg as *mut MSG;
                let translated = TranslateAcceleratorW((*msg).hwnd, menu_bar_c.haccel(), msg);
                translated == 1
            }
        });
    }
    #[cfg(target_os = "macos")]
    event_loop_builder.with_default_menu(false);

    #[allow(unused_mut)]
    let mut event_loop = event_loop_builder.build();

    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let _window2 = WindowBuilder::new().build(&event_loop).unwrap();

    let file_m = Submenu::new("File", true);
    let edit_m = Submenu::new("Edit", true);
    let window_m = Submenu::new("Window", true);

    menu_bar.append_items(&[&file_m, &edit_m, &window_m]);

    let custom_i_1 = TextMenuItem::new(
        "C&ustom 1",
        true,
        Some(Accelerator::new(Some(Modifiers::ALT), Code::KeyC)),
    );
    let custom_i_2 = TextMenuItem::new("Custom 2", false, None);
    let check_custom_i_1 = CheckMenuItem::new("Check Custom 1", true, true, None);
    let check_custom_i_2 = CheckMenuItem::new("Check Custom 2", false, true, None);
    let check_custom_i_3 = CheckMenuItem::new(
        "Check Custom 3",
        true,
        true,
        Some(Accelerator::new(Some(Modifiers::SHIFT), Code::KeyD)),
    );

    let copy_i = predefined::copy(None);
    let cut_i = predefined::cut(None);
    let paste_i = predefined::paste(None);

    file_m.append_items(&[
        &custom_i_1,
        &custom_i_2,
        &predefined::separator(),
        &check_custom_i_1,
    ]);

    window_m.append_items(&[
        &check_custom_i_2,
        &predefined::close_window(None),
        &predefined::separator(),
        &predefined::quit(None),
        &predefined::select_all(None),
        &predefined::about(
            None,
            Some(AboutMetadata {
                name: Some("tao".to_string()),
                copyright: Some("Copyright TAO".to_string()),
                ..Default::default()
            }),
        ),
        &predefined::minimize(None),
        &check_custom_i_3,
        &custom_i_1,
    ]);

    edit_m.append_items(&[&copy_i, &predefined::separator(), &cut_i]);

    edit_m.prepend(&paste_i);
    window_m.insert(&cut_i, 2);

    #[cfg(target_os = "windows")]
    {
        menu_bar.init_for_hwnd(window.hwnd() as _);
        menu_bar.init_for_hwnd(_window2.hwnd() as _);
    }

    #[cfg(target_os = "macos")]
    {
        menu_bar.init_for_nsapp();
    }

    let menu_channel = menu_event_receiver();

    let mut x = 0_f64;
    let mut y = 0_f64;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            #[cfg(target_os = "macos")]
            Event::NewEvents(tao::event::StartCause::Init) => {
                menu_bar.init_for_nsapp();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                window_id,
                ..
            } => {
                if window_id == window.id() {
                    x = position.x;
                    y = position.y;
                }
            }
            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        button: MouseButton::Right,
                        ..
                    },
                window_id,
                ..
            } => {
                if window_id == window.id() {
                    #[cfg(target_os = "windows")]
                    window_m.show_context_menu_for_hwnd(_window2.hwnd(), x, y);
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }

        if let Ok(event) = menu_channel.try_recv() {
            if event.id == custom_i_1.id() {
                file_m.insert(&TextMenuItem::new("asdasd", false, None), 2);
            }
            println!("{:?}", event);
        }
    })
}
