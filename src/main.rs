extern crate gio ; 
extern crate gtk ;

use gtk::{
    ToolButtonExt , 
    WidgetExt ,
}; 

use gio::ApplicationFlags ;

use KageGroove::app::app::App ;


fn main() {
    let application = gtk::Application::new(
        "com.github.gtk-rs.examples.toolbar",
        ApplicationFlags::empty(),
    ).expect("Initialization failed...");
    
    App::new(application) ;
}
