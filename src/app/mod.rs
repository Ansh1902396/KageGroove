pub mod app {
    use crate::toolbar::toolbar::MusicToolbar;
    use gio::{prelude::*, ApplicationFlags};
    use gtk::{prelude::*, Application, ApplicationWindow};

    use std::env;

    use gtk::{ContainerExt, SeparatorToolItem, ToolButton, Toolbar};

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

            window.add(toolbar.toolbar());

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
            unimplemented!()
        }
    }
}
