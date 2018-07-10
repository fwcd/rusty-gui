extern crate sdl2;
#[macro_use]
extern crate log;
extern crate simple_logger;

mod gui;
mod utils;
mod view;
mod model;

use gui::core::gui_application::GUIApplication;
use gui::widgets::widget_gui_app::WidgetGUIApp;
use gui::widgets::layouts::box_layout::BoxLayout;
use gui::widgets::button::Button;
use gui::widgets::label::Label;
use gui::widgets::container::Container;
use gui::core::mainloop::run_gui_app;
use utils::shared::share;

fn main() {
	simple_logger::init_with_level(log::Level::Trace).expect("Could not initialize logger");
	
	let title = "VinylFlow";
	let width = 640;
	let height = 480;
	let layout = BoxLayout::horizontal();
	
	info!("Initializing application...");
	
	let mut app = WidgetGUIApp::new(title, width, height, Box::new(layout));
	{
		let root = app.root();
		root.add(share(Button::labelled("Test", 64)));
		root.add(share(Button::labelled("Test2", 64)));
		root.add(share(Label::of("Demo", 15)));
		let mut container = Container::vbox();
		container.add(share(Button::labelled("One", 12)));
		container.add(share(Button::labelled("Two", 12)));
		container.add(share(Button::labelled("Three", 12)));
		root.add(share(container));
	}
	run_gui_app(&mut app);
}
