# Rust XCB

Rust-XCB is a safe Rust interface to [XCB](http://xcb.freedesktop.org).
Rust-XCB uses under the hood the core XCB functions to connect and communicate
with the X server.

```toml
[dependencies]
xcb = "1.0.0-beta"
```

__Documentation__:
https://rust-x-bindings.github.io/rust-xcb/branches/v1.0-dev/xcb/

Rust-XCB is constituted of the following components:
 - the core library
 - the X protocol and its extensions

See [CONTRIBUTING.md](https://github.com/rust-x-bindings/rust-xcb/blob/v1.0-dev/CONTRIBUTING.md) for contributions.

## The core library

The main component is the `Connection` class which is used to connect and
communicate to the X server. The `Connection` class wraps calls to the C XCB
functions in a safe to use interface.

In the new API (`v1.0+`), Rust-XCB takes care of all the heavy lifting of event
and error resolution, including the handling of the different kinds of event
(regular events, "GeGeneric" events, the specifics about Xkb ...) and present
them under a unified and safe to use `enum` instead of requesting the user to
perform unsafe cast as in the C library.

The core library also provides many traits that are used in the protocol
implementation. e.g. the `Wired` trait has implementation for each type that
must be serialized for the requests.

## The protocol implementation

The core X protocol and all the extensions present in XCB are generated by the
build scripts, entirely written in Rust (no more python dependency).
The build script do not generate bindings to the C protocol extensions, it
generates an actual Rust implementation of the protocol:
 - Simple structures that have the same memory layout than C are translated in
   Rust directly (e.g. points etc.)
 - More complex structures wrap a slice of raw data and provide accessor methods
   (`Debug` will still print all the members)
 - The masks use the `bitflags` crate macro
 - The enums are enums!
 - The unions are also enums, but they carry data
 - The Xids (handles to windows, pixmaps etc.) are type-safe implementations of
   the `Xid` trait, not just integers like with `0.x` versions.
 - The requests are structures that serialize themselves when passed to the
   `Connection`.
    - Each request has two type of cookie, for checked and unchecked requests.
      This allows type safe reply fetching and error checking
 - The protocol and each extension provide an `Event` and an `Error` enum,
   which are unified by the core library.

## The new API

Here are some highlights of the new API:

### Modules

Previously, the core X protocol was directly accessible under the `xcb` crate
namespace. This is no longer the case, the protocol is under the `x` module,
and each extension gets its own module too like before.

Only the core library is directly accessible under `xcb`.
This helps for a clean separation of concerns and save a lot of confusion in
regards to the new event and error types.

### Window creation

```rust
// v0.x
let window = conn.generate_id(); // this is a u32

let values = [
    (xcb::CW_BACK_PIXEL, screen.white_pixel()),
    (
        xcb::CW_EVENT_MASK,
        xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS,
    ),
];

xcb::create_window(
    &conn,
    xcb::COPY_FROM_PARENT as u8,
    window,
    screen.root(),
    0,
    0,
    150,
    150,
    10,
    xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
    screen.root_visual(),
    &values,
);

// v1.0

let window: x::Window = conn.generate_id();

conn.send_request(&x::CreateWindow {
    depth: x::COPY_FROM_PARENT as u8,
    wid: window,
    parent: screen.root(),
    x: 0,
    y: 0,
    width: 150,
    height: 150,
    border_width: 10,
    class: x::WindowClass::InputOutput,
    visual: screen.root_visual(),
    value_list: &[
        x::Cw::BackPixel(screen.white_pixel()),
        x::Cw::EventMask((x::EventMask::EXPOSURE | x::EventMask::KEY_PRESS).bits()),
    ],
});
```

### Checked void request
```rust
// v0.x
// same cookie than for xcb::map_window
let cookie = xcb::map_window_checked(&conn, window);
// this error is a simple wrapper over xcb_generic_error_t
cookie.request_check()?;

// v1.0
// specific cookie type for the checked request
// code would not compile with `conn.send_request(..)`
let cookie = conn.send_request_checked(&x::MapWindow {window});
// reports a resolved error enum (e.g. x::Error::Drawable(..))
conn.check_request(cookie)?;

```

### Event and error handling
```rust
// 0.x
loop {
    let event = conn.wait_for_event();
    // Errors are only wrappers around generic C errors and must be casted
    // from the event if `response_type` is 0. That seem very safe Rust...
    // I am the core author and I don't exactly know how to do proper error
    // handling with the 0.x API...
    match event {
        None => {
            break;
        }
        Some(event) => {
            let r = event.response_type();
            if r == 0 {
                // This is an error.
                panic!("received error from the X server");
            } else if r == xcb::KEY_PRESS as u8 {
                let key_press: &xcb::KeyPressEvent = unsafe {
                    // so much for the safe interface
                    xcb::cast_event(&event)
                };

                // do stuff

            } else if r == xkb_first_event {
                // resolving extension events like we would in C
                // very poor support is provided (not better than C)
            }
        }
    }
}

// 1.0
fn main() -> xcb::Result<()> {
    // ...
    loop {
        match conn.wait_for_event()? {
            xcb::Event::X(x::Event::KeyPress(key_press)) => {
                // do stuff
            }
            xcb::Event::Xkb(xkb::Event::MapNotify(ev)) => {
                // do other stuff (pass data to xkbcommon for example)
            }
            _ => {}
        }
    }
}
```
