#[macro_export]
macro_rules! log {
	($($arg:tt)*) => {{
		print!("{}{}[Figtok]{}{}: ", termion::color::Fg(termion::color::Green), termion::style::Bold, termion::style::Reset, termion::color::Fg(termion::color::White));
		println!($($arg)*);
	}};
}