use std::cell::Cell;
use std::rc::Rc;

use glib::clone;
use gtk::{prelude::*, Label};
use gtk::{self, glib, Application, ApplicationWindow, Button, Orientation};

const APP_ID: &str = "org.gtk_rs.GObjectMemoryManagement3";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}
fn build_ui(app: &Application) {
    // Create two buttons
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Reference-counted object with inner mutability
    let number = Rc::new(Cell::new(0));

    let label_number = Label::builder()
    .label("0")
    .margin_top(12)
    .margin_bottom(12)
    .margin_start(12)
    .margin_end(12)
    .build();

    // Connect callbacks
    // When a button is clicked, `number` will be changed
    button_increase.connect_clicked(clone!(@weak number, @strong label_number => move |_| {
        number.set(number.get() + 1);
        let x = number.get();
        println!("Number: {x}");
        label_number.set_label(&number.get().to_string());
    }));

    button_decrease.connect_clicked(clone!(@weak label_number => move |_| {
        number.set(number.get() - 1);
        let x = number.get();
        println!("Number: {x}");
        label_number.set_label(&number.get().to_string());
    }));

    // Add buttons to `gtk_box`
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    gtk_box.append(&label_number);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}