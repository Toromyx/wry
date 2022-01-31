// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tao::event::{DeviceEvent, ElementState};

fn main() -> wry::Result<()> {
  use wry::{
    application::{
      event::{Event, StartCause, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::WindowBuilder,
    },
    webview::WebViewBuilder,
  };

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title("Hello World")
    .build(&event_loop)?;
  let webview = WebViewBuilder::new(window)?
    .with_url("https://html5test.com")?
    .build()?;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => {
        println!("Press any key inside the window to throw a coin. The result will be output here.")
      }
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      Event::DeviceEvent {
        event: DeviceEvent::Key(raw),
        ..
      } => {
        if raw.state == ElementState::Released {
          let _ = webview.evaluate_script(
            "
            window.__WRY_HEADS__ = Math.random() < 0.5;
            window.alert(window.__WRY_HEADS__ ? 'Heads!' : 'Tails!');
            window.__WRY_HEADS__;
            ",
            |result| {
              let heads: bool = serde_json::from_str(&result).unwrap();
              println!(
                "{}",
                match heads {
                  true => "Heads!",
                  false => "Tails!",
                }
              )
            },
          );
        }
      }
      _ => {}
    }
  });
}
