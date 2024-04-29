# Todo list in Rust

Attribution for app icon: https://www.flaticon.com/free-icon/to-do-list_3176366  

Link to app icon download: <a href="https://www.flaticon.com/free-icons/to-do-list" title="to do list icons">To do list icons created by Freepik - Flaticon</a>  

With references to the following:  

https://www.youtube.com/watch?v=NtUkr_z7l84  

https://www.youtube.com/watch?v=SvFPdgGwzTQ  

https://github.com/emilk/egui?tab=readme-ov-file#quick-start  

https://docs.rs/egui/latest/egui/  

https://github.com/emilk/egui/tree/master/crates/eframe  

https://docs.rs/eframe/latest/eframe/  

https://www.egui.rs/#demo  

https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/widget_gallery.rs  

https://github.com/appcove/egui.info/tree/master/examples/egui-215-todo-list  

https://rodneylab.com/trying-egui/  

https://github.com/emilk/egui/discussions/2026  

https://github.com/emilk/egui/discussions/3971  

https://github.com/emilk/egui/discussions/2133  

# Windows setup instructions  

1. clone the repo  

2. add build target via rustup  
`rustup target add x86_64-pc-windows-gnu`  

3. build release  
`cargo build --release --target x86_64-pc-windows-gnu`

or download the release binary from Releases  

# Linux setup instructions  

Same steps for 1. and 2. from Windows setup

3. build release  
`cargo build --release`  

or download the release binary from Releases  

# Enabling persistence  

Uncomment the eframe dependency within cargo.toml with persistence feature and comment out the efrane dependency that
has no features  


