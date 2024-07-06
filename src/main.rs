use std::path::PathBuf;

fn main() {
    let path_to_image = PathBuf::from("image-1.avif");
    gtk::gdk_pixbuf::Pixbuf::from_file(&path_to_image).unwrap();
    gtk::gdk::Texture::from_filename(&path_to_image).unwrap();
}
