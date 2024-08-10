extern crate gio ; 
extern crate gtk ;

use gtk::{
    ToolButtonExt , 
    WidgetExt ,
}; 



use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags} ;

use KageGroove::app::app::{self, App} ;


fn main() {
    let application = gtk::Application::new(
        "com.github.gtk-rs.examples.toolbar",
        ApplicationFlags::empty(),
    ).expect("Initialization failed...");
    
    application.connect_startup(|application| { 
        App::new(application.clone()) ;
    });

    application.run(&[]) ;

    
}
