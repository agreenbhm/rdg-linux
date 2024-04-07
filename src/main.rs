mod connections;
mod error;
mod macros;
mod profiles;
mod rdg;
mod resources;
mod settings;

use gtk::gio::prelude::*;

fn main() -> error::RdgResult<()> {
    // Load resources
    resources::load();

    let application = gtk::Application::new(Some("net.olback.rdg"), Default::default());

    application.connect_activate(move |app| {
        let _ = rdg::Rdg::build(app).expect("failed to build app");
    });

    application.run();

    Ok(())
}
