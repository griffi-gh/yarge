#[cfg(all(windows, feature = "windows-icon"))]
extern crate winres;

fn main() {
  #[cfg(all(windows, feature = "windows-icon"))] {
    println!("cargo:rerun-if-changed=yarge.ico");
    let mut res = winres::WindowsResource::new();
    res.set_icon("yarge.ico");
    res.compile().unwrap();
  }
}