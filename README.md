# miniquad Linux window resizing repro

Minimal reproduction for two miniquad window bugs, reported against
`miniquad 0.4.11` (the version used by macroquad 0.4.16):

1. **X11** — after `set_fullscreen(false)`, KWin restores the window
   **maximized** instead of at its previous size.
2. **Wayland** — `set_window_size` is silently dropped by the backend.

Reproduced with KWin (KDE Plasma) on an X11 session and on a Wayland
session (XWayland client).

Issue: [link to issue (tbd)]

## Running

```
cargo run             # X11 backend (miniquad default; via XWayland on a Wayland session)
WAYLAND=1 cargo run   # native Wayland backend
```

| Key      | Action |
| -------- | ------ |
| `F`      | Toggle fullscreen |
| `R`      | Cycle window size: 1024x768 -> 1280x720 -> 800x600 (ignored while fullscreen) |
| `Escape` | Quit |

## Buggy behavior

**X11** (`cargo run`, KWin):

1. Window starts at 1024x768.
2. `R` resizes fine.
3. `F` enters fullscreen.
4. `F` returns to windowed — the window comes back **maximized**, not at
   its previous size. `R` has no visible effect afterwards because KWin
   holds the window maximized.

**Wayland** (`WAYLAND=1 cargo run`):

1. Window starts at 1024x768.
2. `R` does nothing — the request is dropped by the backend.

## Verification

Every `set_fullscreen` / `set_window_size` request and every
`resize_event` reported by the backend is logged to stderr with a
`[repro]` prefix, so you can compare what was asked for against what
the window manager actually did.

## Testing the fix

To test against a patched miniquad, point the dependency at your fork
checkout in `Cargo.toml` (replaces the crates.io version):

```toml
[patch.crates-io]
miniquad = { path = "/path/to/miniquad" }
```

Expected results with the fix:

- **X11**: `F` -> `F` returns the window at its previous size, not maximized.
- **Wayland**: `R` resizes the window. Note the size is a compositor
  hint under xdg-shell — honored for floating windows, ignored while
  the window is tiled or fullscreen.

## Notes

- Standalone, no game assets, builds as-is.
- Backend selection uses the `linux_backend` platform conf option:
  `X11Only` by default (miniquad's default), `WaylandOnly` when
  `WAYLAND=1` is set.
