use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

fn on_activate(application: &gtk::Application) {
  let window = gtk::ApplicationWindow::new(application);
  let button = gtk::Button::with_label("Hello World");
  button.connect_clicked(clone!(@weak window => move |_| window.close()));
  window.add(&button);
  window.show_all();
}

fn main() {
  let app = gtk::Application::new(Some("com.github.smallkirby.rkeyshower"), Default::default()).unwrap();
  app.connect_activate(|app| on_activate(app));
  app.run(&std::env::args().collect::<Vec<_>>());
}