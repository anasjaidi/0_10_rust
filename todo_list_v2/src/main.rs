use std::io::stdout;
use termion::raw::IntoRawMode;

fn main() {
    let out = stdout();
    let mut stdout = out.lock().into_raw_mode().unwrap();
    println!(
        "{} {}RED",
        termion::clear::All,
        termion::color::Fg(termion::color::Green)
    )
}
