#![feature(type_alias_impl_trait)]
#![feature(type_ascription)]

use adw::prelude::*;

use adw::{ActionRow, Application, ApplicationWindow, HeaderBar};
use adw::builders;
use adw::builders::ApplicationWindowBuilder;
use adw::gtk::{Box, ListBox, /*Native,*/ Orientation, SelectionMode};

pub use native_ui_rs::abstracts;
//use native_ui_rs::abstracts::ApplicationBuilder;

enum NativeWidgetBuilder {
    Window(builders::ApplicationWindowBuilder),
}

impl Into<builders::ApplicationWindowBuilder> for NativeWidgetBuilder {
    fn into(self) -> ApplicationWindowBuilder {
        match self {
            NativeWidgetBuilder::Window(window) => window,
            _ => panic!("Wrong NativeWidgetBuilder type"),
        }
    }
}

//enum NativeWidget {
//    Window(ApplicationWindow)
//}

trait BuildWidget {
    type Widget: IsA<adw::gtk::Widget>;
    fn build(self) -> Self::Widget;
}

trait NativeWidgetBuild {
    // creates NativeWidgetBuilder from abstracts::Widget
    // fills in an many fields as possible
    // returns NativeWidgetBuilder
    fn native_builder(&self) -> NativeWidgetBuilder;
}

impl NativeWidgetBuild for abstracts::Window {
    fn native_builder(&self) -> NativeWidgetBuilder {
        let mut builder = ApplicationWindow::builder()
                .title(self.title.as_str())
                .default_width(self.width as i32)
                .default_height(self.height as i32);
        builder = match self.content.clone() {
            Some(widget) => builder.content(
                &widget.native_builder().build()
            ),
            None => builder,
        };
        NativeWidgetBuilder::Window(builder)
    }
}

impl NativeWidgetBuild for abstracts::Widget {
    #[allow(unconditional_recursion)]
    fn native_builder(&self) -> NativeWidgetBuilder {
        match self.clone() {
            abstracts::Widget::Window(window) => {
                window.content.unwrap().native_builder()
            },
            #[allow(unreachable_patterns)]
            _ => panic!("Widget doesn't implement builder to native yet {:#?}", self)
        }
    }
}

impl BuildWidget for NativeWidgetBuilder {
    type Widget = impl IsA<adw::gtk::Widget>;
    fn build(self) -> Self::Widget {
        match self {
            NativeWidgetBuilder::Window(window) => window.build(),
            #[allow(unreachable_patterns)]
            _ => panic!("Native Widget build not implemented yet")
        }
    }
}

pub struct NativeApplication {
    abst: abstracts::Application
}

impl NativeApplication {
    pub fn new(abst: abstracts::Application) -> NativeApplication {
        NativeApplication {
            abst
        }
    }

    pub fn run(&mut self) {
        let application: Application = Application::builder()
            .application_id("com.example.FirstAdwaitaApp")
            .build();

        let abst = self.abst.clone();

        application.connect_activate(move |app| activate_application(app, &abst));

        application.run();
    }

}

fn activate_application(app: &Application, abst: &abstracts::Application) {
    // ActionRows are only available in Adwaita
    let row = ActionRow::builder()
        .activatable(true)
        .title("Click me")
        .build();
    row.connect_activated(|_| {
        eprintln!("Clicked!");
    });

    let list = ListBox::builder()
        .margin_top(32)
        .margin_end(32)
        .margin_bottom(32)
        .margin_start(32)
        .selection_mode(SelectionMode::None)
        // makes the list look nicer
        .css_classes(vec![String::from("boxed-list")])
        .build();
    list.append(&row);

    // Combine the content in a box
    let content = Box::new(Orientation::Vertical, 0);
    // Adwaitas' ApplicationWindow does not include a HeaderBar
    content.append(&HeaderBar::new());
    content.append(&list);

    let mut application_windows = Vec::new();
    for window in &abst.main_windows {
        let app_window = (window
            .native_builder()
            .into(): ApplicationWindowBuilder)
            .application(app)
            .content(&content)
            .build();
        app_window.show();
        application_windows.push(app_window);
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
