// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
  };
use wry::WebViewBuilder;
  
fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let html_content = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="utf-8">
        <style>
            body {
                background: white !important;
                color: black !important;
                font-family: Arial, sans-serif;
                padding: 20px;
                margin: 0;
            }
            h1 { color: #1a73e8 !important; }
            .status { 
                padding: 10px;
                background: #f1f3f4;
                border-radius: 5px;
                margin-top: 20px;
            }
        </style>
    </head>
    <body>
        <h1>Visible Content Test</h1>
        <p>This text should be clearly visible</p>
        <div class="status">
            System Information:
            <ul>
                <li>Platform: <span id="platform"></span></li>
                <li>User Agent: <span id="ua"></span></li>
            </ul>
        </div>
        <script>
            document.getElementById('platform').textContent = navigator.platform;
            document.getElementById('ua').textContent = navigator.userAgent;
        </script>
    </body>
    </html>
    "#;

    let builder = WebViewBuilder::new()
        //.with_url("http://tauri.app")
        .with_html(html_content)
        .with_drag_drop_handler(|e| {
        match e {
            wry::DragDropEvent::Enter { paths, position } => {
            println!("DragEnter: {position:?} {paths:?} ")
            }
            wry::DragDropEvent::Over { position } => println!("DragOver: {position:?} "),
            wry::DragDropEvent::Drop { paths, position } => {
            println!("DragDrop: {position:?} {paths:?} ")
            }
            wry::DragDropEvent::Leave => println!("DragLeave"),
            _ => {}
        }

        true
        });

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let _webview = builder.build(&window)?;
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let _webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox)?
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
        } = event
        {
        *control_flow = ControlFlow::Exit;
        }
    });
}