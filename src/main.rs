use gtk::prelude::*;
use webkit2gtk::WebView;
use webkit2gtk::WebViewExt;

fn main() {
    // Inicializa GTK
    if gtk::init().is_err() {
        eprintln!("Error al inicializar GTK.");
        return;
    }

    // Crea una ventana principal
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Chat DeepSeek");
    window.set_default_size(1024, 768); // Tamaño inicial de la ventana

    // Cierra la aplicación cuando se cierra la ventana
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Crea un WebView (navegador integrado)
    let webview = WebView::new();
    webview.load_uri("https://chat.deepseek.com");

    // Añade el WebView a la ventana
    window.add(&webview);

    // Muestra la ventana y todos sus componentes
    window.show_all();

    // Inicia el bucle principal de GTK
    gtk::main();
}
