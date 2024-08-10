
pub mod toolbar { 
    use gtk::{
        ContainerExt , 
        SeparatorToolItem , 
        Toolbar , 
        ToolButton
    } ;

    const PLAY_STOCK: &str = "gtk-media-play" ;

    pub struct MusicToolbar {
       pub open_button: ToolButton,
       pub next_button: ToolButton,
       pub play_button: ToolButton,
        pub  previous_button: ToolButton,
       pub quit_button: ToolButton,
       pub remove_button: ToolButton,
        pub  stop_button : ToolButton , 
       pub toolbar: Toolbar,
    }

    impl MusicToolbar { 
        pub fn new() -> Self { 
            let toolbar = Toolbar::new() ;

            let open_button = ToolButton::new_from_stock("gtk-open") ;
            let previous_button = ToolButton::new_from_stock("gtk-media-previous") ;
            let play_button = ToolButton::new_from_stock(PLAY_STOCK) ;
            let next_button = ToolButton::new_from_stock("gtk-media-next") ;
            let stop_button = ToolButton::new_from_stock("gtk-media-stop") ;
            let quit_button = ToolButton::new_from_stock("gtk-quit") ;
            let remove_button = ToolButton::new_from_stock("gtk-remove") ;

            toolbar.add(&open_button) ;
            toolbar.add(&SeparatorToolItem::new()) ;
            toolbar.add(&previous_button) ;
            toolbar.add(&SeparatorToolItem::new()) ;
            toolbar.add(&play_button) ;
            toolbar.add(&SeparatorToolItem::new()) ;
            toolbar.add(&next_button) ;
            toolbar.add(&SeparatorToolItem::new()) ;
            toolbar.add(&stop_button) ;
            toolbar.add(&SeparatorToolItem::new()) ;
            toolbar.add(&remove_button) ;
            toolbar.add(&SeparatorToolItem::new()) ;
            toolbar.add(&quit_button) ;

            Self { 
                open_button , 
                next_button , 
                play_button , 
                previous_button , 
                quit_button , 
                remove_button , 
                stop_button , 
                toolbar
            }

        }
        
        pub fn toolbar (&self) -> &Toolbar { 
            &self.toolbar
        }
    }
}