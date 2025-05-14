use fltk::{app, button::Button, draw, enums::Font, frame::Frame, prelude::*, window::Window};

pub fn popup(msg: String) {
	let app = app::App::default();

    draw::set_font(Font::HelveticaBold, 24);
    let frame_size = draw::measure(&msg, true);
    let win_w = frame_size.0 + 10;
    let win_h = frame_size.1 + 100;

	let mut win = Window::default()
		.with_size(win_w, win_h)
		.center_screen()
		.with_label("Cool Birthday");

	let mut frame = Frame::default()
		.with_size(15, 50)
		.center_of(&win)
		.with_label(msg.as_str());
	frame.set_label_size(24);
	frame.set_label_font(Font::HelveticaBold);

	let mut btn = Button::default()
		.with_size(30, 30)
		.below_of(&frame, 0)
		.with_label("Ok");

	btn.set_callback(move |_| {
		app::quit();
	});

	win.end();
	win.show();
	app.run().unwrap();
}