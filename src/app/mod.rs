pub mod app {
    use crate::toolbar::toolbar::MusicToolbar;
    use gio::{prelude::*, ApplicationFlags};
    use gtk::{prelude::*, Application, ApplicationWindow};

    use std::env;

    use gtk::{ContainerExt, SeparatorToolItem, ToolButton, Toolbar};

    use gtk::{
        Adjustment , 
        Image , 
        ImageExt , 
        Scale , 
        ScaleExt ,
    } ; 

    use gtk::Orientation::{Horizontal, Vertical};

    const PLAY_STOCK: &str = "gtk-media-play";
    const PAUSE_STOCK: &str = "gtk-media-pause";
    pub struct App {
        toolbar: MusicToolbar,
        window: ApplicationWindow,
    }

    impl App {
       pub fn new(application: Application) -> Self {
            let window = ApplicationWindow::new(&application);
            let toolbar = MusicToolbar::new();

            window.set_title("Music Player");
            let vbox = gtk::Box::new(Vertical, 0);
            window.add(&vbox);
            
            vbox.add(toolbar.toolbar()); 

            let cover = Image::new();
            cover.set_from_file("cover.jpg");
            vbox.add(&cover);

            let adjustment = Adjustment::new(0.0 , 0.0 , 10.0 , 0.0, 0.0, 0.0) ;
            let scale = Scale::new(Horizontal, &adjustment) ;
            
            vbox.add(&scale) ;

            window.show_all();

            let app = App { toolbar, window };

            app.connect_events();
            app.connect_toolbar_events();

            app
        }

        pub fn connect_toolbar_events(&self) { 
            let window = self.window.clone() ;
            self.toolbar.quit_button.connect_clicked(move |_| {
                window.destroy();
            });

            let play_button = self.toolbar.play_button.clone() ;

            self.toolbar.play_button.connect_clicked(move |_|  {
                if play_button.get_stock_id() == Some(PLAY_STOCK.to_string()) { 
                    play_button.set_stock_id(PAUSE_STOCK) ;
                }else  {
                    play_button.set_stock_id(PLAY_STOCK) ;
                }
            }); 

        
        }

        
        fn connect_events(&self) {
            
        }
    }
}
