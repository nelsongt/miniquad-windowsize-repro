//! Minimal repro for the miniquad Linux window bugs:
//!   1. X11: after `set_fullscreen(false)`, KWin restores the window
//!      **maximized** instead of at its previous size.
//!   2. Wayland: `set_window_size` is silently dropped by the backend.
//!
//! Build/run: `cargo run` (or `WAYLAND=1 cargo run` to force the
//! Wayland backend instead of the default X11Only/XWayland)
//! Keys:
//!   F        — toggle fullscreen
//!   R        — cycle window size 1024x768 -> 1280x720 -> 800x600
//!   Escape   — quit
//!
//! Expected (buggy) behavior on KWin, X11 session or XWayland:
//!   1. Window starts at 1024x768.
//!   2. Press R: window resizes fine.
//!   3. Press F: goes fullscreen.
//!   4. Press F: comes back **maximized**, not at its previous size.
//!      (Pressing R afterwards also has no visible effect, because
//!      KWin holds the window maximized.)
//!
//! Expected (buggy) behavior on native Wayland:
//!   1. Window starts at 1024x768.
//!   2. Press R: nothing happens (request dropped by the backend).
//!
//! Every `set_fullscreen` / `set_window_size` request and every
//! `resize_event` is logged to stderr, so you can compare what the
//! backend was asked for against what the window manager shows.

use miniquad::{conf, window, EventHandler, KeyCode, KeyMods};

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 768;
const SIZES: [(i32, i32); 3] = [(1024, 768), (1280, 720), (800, 600)];

struct Repro {
    fullscreen: bool,
    size_idx: usize,
}

impl EventHandler for Repro {
    fn update(&mut self) {}
    fn draw(&mut self) {}

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::F => {
                self.fullscreen = !self.fullscreen;
                window::set_fullscreen(self.fullscreen);
                eprintln!("[repro] set_fullscreen({})", self.fullscreen);
            }
            KeyCode::R => {
                if !self.fullscreen {
                    self.size_idx = (self.size_idx + 1) % SIZES.len();
                    let (w, h) = SIZES[self.size_idx];
                    window::set_window_size(w as u32, h as u32);
                    eprintln!("[repro] set_window_size({w}x{h})");
                }
            }
            KeyCode::Escape => window::quit(),
            _ => {}
        }
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        eprintln!(
            "[repro] resize_event: {}x{} (fullscreen={})",
            width, height, self.fullscreen
        );
    }
}

fn main() {
    // The default `linux_backend` is `X11Only`, so on a Wayland session
    // this runs as an X11 client via XWayland — same setup as the game.
    // `WAYLAND=1` forces the native Wayland backend for bug 2.
    let platform = if std::env::var("WAYLAND").is_ok() {
        conf::Platform {
            linux_backend: conf::LinuxBackend::WaylandOnly,
            ..Default::default()
        }
    } else {
        conf::Platform::default()
    };
    miniquad::start(
        conf::Conf {
            window_title: "miniquad X11 fullscreen repro".to_string(),
            window_width: WIDTH,
            window_height: HEIGHT,
            platform,
            ..Default::default()
        },
        || {
            Box::new(Repro {
                fullscreen: false,
                size_idx: 0,
            })
        },
    );
}
