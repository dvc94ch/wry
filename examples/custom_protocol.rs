// Copyright 2020-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

fn main() -> wry::Result<()> {
  use std::fs::{canonicalize, read};

  use wry::{
    application::{
      event::{Event, StartCause, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::WindowBuilder,
    },
    http::{header::CONTENT_TYPE, Response},
    webview::WebViewBuilder,
  };

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title("Hello World")
    .build(&event_loop)
    .unwrap();

  let webview = WebViewBuilder::new(window)
    .unwrap()
    .with_custom_protocol("wry".into(), move |request| {
      let path = &request.uri().path();
      // Read the file content from file path
      let content = read(canonicalize(PathBuf::from("examples").join(
        if path == &"/" {
          "index.html"
        } else {
          // remove leading slash
          &path[1..]
        },
      ))?)?;

      // Return asset contents and mime types based on file extentions
      // If you don't want to do this manually, there are some crates for you.
      // Such as `infer` and `mime_guess`.
      let (data, meta) = if path.ends_with(".html") || path == &"/" {
        (content, "text/html")
      } else if path.ends_with(".js") {
        (content, "text/javascript")
      } else if path.ends_with(".png") {
        (content, "image/png")
      } else {
        unimplemented!();
      };

      Response::builder()
        .header(CONTENT_TYPE, meta)
        .body(data)
        .map_err(Into::into)
    })
    // tell the webview to load the custom protocol
    .with_url("wry://localhost")?
    .with_devtools(true)
    .build()?;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => println!("Wry application started!"),
      Event::WindowEvent {
        event: WindowEvent::Moved { .. },
        ..
      } => {
        let _ = webview.evaluate_script("console.log('hello');");
      }
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      _ => (),
    }
  });
}
