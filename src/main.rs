use termion;

use std::io::{stdin, stdout, Write};
use termion::clear;
use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

fn main() {
    let stdin = stdin();

    // stdout変数がDropするときにrawモードから元の状態に戻る　
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    // 画面全体のクリア
    write!(stdout, "{}", clear::All);
    // カーソルを左上に設定(1-indexed)
    write!(stdout, "Hello World!");
    stdout.flush().unwrap();


    for event in stdin.events() {
        if event.unwrap() == Event::Key(Key::Ctrl('c')) {
            return;
        }
    }
}
