pub fn load() {
    let glib_resource_bytes = gtk::glib::Bytes::from_static(include_bytes!("../out/rdg.gresource"));
    let resources =
        gtk::gio::Resource::from_data(&glib_resource_bytes).expect("Failed to load resources");
    gtk::gio::resources_register(&resources);
}
