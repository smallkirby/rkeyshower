use gio::prelude::*;
use gtk::prelude::*;

fn on_activate(application: &gtk::Application) {
  let window = gtk::ApplicationWindow::new(application);
  window.set_keep_above(true);
  window.set_focus_on_map(false);
  window.set_accept_focus(false);

  // set size

  let screen = window.get_screen().unwrap();
  let monitor_ix = screen.get_primary_monitor();
  let workarea = screen.get_monitor_workarea(monitor_ix);
  let width = workarea.width;
  let height = workarea.height / 10;
  window.set_default_size(width, height);
  window.set_decorated(false);

  // set position
  let old_position = window.get_position();
  let topleft = (old_position.0, workarea.height / 10 * 9);
  window.set_gravity(gdk::Gravity::Center);
  window.set_screen(&screen);
  window.move_(topleft.0, topleft.1);

  // label
  let label = gtk::Label::new(Some("WAIWAI"));
  label.set_opacity(50.0);
  label.set_justify(gtk::Justification::Center);
  label.override_background_color(gtk::StateFlags::NORMAL, Some(&gdk::RGBA::black()));
  label.override_color(gtk::StateFlags::NORMAL, Some(&gdk::RGBA::white()));

  // font
  let mut font = pango::FontDescription::new();
  font.set_weight(pango::Weight::Heavy);
  font.set_size((50 * (workarea.height / 10) / 100) * 1000);
  label.override_font(&font);

  label.show();
  window.add(&label);

  // show
  window.show_all();
}

fn main() {
  let app =
    gtk::Application::new(Some("com.github.smallkirby.rscreenkey"), Default::default()).unwrap();
  app.connect_activate(|app| on_activate(app));
  app.run(&std::env::args().collect::<Vec<_>>());
}
