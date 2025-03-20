use gtk::{gdk::Display, prelude::*};
use webkit2gtk::{WebView, WebViewExt};

fn main() {
    gtk::init().expect("Failed to initialize GTK");
    let display = Display::default().expect("Failed to get display");
    println!("Using backend: {}", display.backend().is_x11());

    // Создание главного окна
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("WebKit2GTK Test Browser");
    window.set_default_size(800, 600);

    // Создание WebView
    let web_view = WebView::new();

    // Загрузка HTML-контента
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


    web_view.load_html(html_content, None);

    // Добавление WebView в окно
    window.add(&web_view);

    // Обработка закрытия окна
    window.connect_destroy(|_| {
        gtk::main_quit();
    });

    // Показать все элементы
    window.show_all();

    // Запуск главного цикла GTK
    gtk::main();
}