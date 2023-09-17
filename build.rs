fn main() {
    glib_build_tools::compile_resources(
        &["content"],
        "content/app.gresource.xml",
        "gtk-rss-reader.gresource",
    );
}
