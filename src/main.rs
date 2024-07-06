use std::{
    io::{Cursor, Read},
    path::{Path, PathBuf},
    process::exit,
    fs::File,
};

use chrono::Utc;
use clap::Parser;
use gdk4::{glib::Bytes, Texture};
use image::ImageFormat;
use rand::{thread_rng, Rng};
use relm4::{
    gtk::{self, prelude::*, Orientation},
    RelmWidgetExt,
};
use relm4::{ComponentParts, ComponentSender, RelmApp, SimpleComponent};

#[derive(Parser)]
struct Cli {
    images: Vec<PathBuf>,
}

#[derive(Debug)]
struct AppModel {
    images: Vec<PathBuf>,
    left: usize,
    right: usize,
}

fn get_something(path: &Path) -> Texture {
    println!("Rendering {}", path.display());
    let start = Utc::now().time();
    let mut buffer = Vec::new();
    File::open(path).unwrap().read_to_end(&mut buffer).unwrap();
    let image = libavif_image::read(&buffer)
        .unwrap();
    buffer.clear();
    image.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png).unwrap();
    let texture = Texture::from_bytes(&Bytes::from(&buffer)).unwrap();
    println!(" in {}ms", (Utc::now().time() - start).num_milliseconds());
    texture
}

#[derive(Debug)]
enum AppInput {
    Left,
    Right,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = Vec<PathBuf>;
    type Input = AppInput;
    type Output = ();

    view! {
        #[root]
        gtk::ApplicationWindow {
            gtk::Box {
                set_orientation: Orientation::Horizontal,
                set_hexpand: true,
                set_homogeneous: true,
                gtk::Button {
                    set_margin_all: 0,
                    connect_clicked => AppInput::Left,
                    gtk::ScrolledWindow {
                        gtk::Image {
                            #[watch]
                            set_from_paintable: Some(&get_something(&model.images[model.left])),
                        }
                    }
                },
                gtk::Button {
                    set_margin_all: 0,
                    connect_clicked => AppInput::Right,
                    gtk::ScrolledWindow {
                        gtk::Image {
                            #[watch]
                            set_from_paintable: Some(&get_something(&model.images[model.right])),
                        }
                    }
                }
            }
        }
    }

    fn init(
        images: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut rng = thread_rng();
        let left = rng.gen_range(0..images.len());
        let mut right = rng.gen_range(0..images.len());
        while right == left {
            right = rng.gen_range(0..images.len());
        }
        let model = AppModel {
            images,
            left,
            right,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::Left => println!("left"),
            AppInput::Right => println!("right"),
        }
        let mut rng = thread_rng();
        self.left = rng.gen_range(0..self.images.len());
        self.right = rng.gen_range(0..self.images.len());
        while self.right == self.left {
            self.right = rng.gen_range(0..self.images.len());
        }
    }
}

fn main() {
    let Cli { images } = Cli::parse();
    if images.len() < 2 {
        eprintln!("You have to provide at least two images.");
        exit(1);
    }
    let app = RelmApp::new("jthulhu.ranki");
    app.with_args(Vec::new()).run::<AppModel>(images);
    // let path_to_image = PathBuf::from("image-1.webp");
    // gtk::gdk_pixbuf::Pixbuf::from_file(&path_to_image).unwrap();
    // gtk::gdk::Texture::from_filename(&path_to_image).unwrap();
}
