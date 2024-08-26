extern crate gio ; 
extern crate gtk ;

extern crate gdk_pixbuf ;
extern crate id3 ; 
extern crate crossbeam; 
extern crate pulse_simple ; 
extern crate simplemad ; 
use std::{env, time::Duration};


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

    application.connect_activate(|_| {});

    application.run(&env::args().collect::<Vec<_>>());

    
}
