extern crate libc;
extern crate x11;

use std::ptr::{
    null,
    null_mut,
};
use std::mem::{
    zeroed,
    size_of_val,
};
// use std::ffi::CString;

use x11::xlib;
use x11::keysym;

fn main() {
    unsafe {
        let display = xlib::XOpenDisplay(null());
        if display == null_mut() {
            panic!("Canno't open X11-Display.")
        }

        // Default Resources
        let screen = xlib::XDefaultScreen(display);
        let root = xlib::XRootWindow(display, screen);
        let visual = xlib::XDefaultVisual(display, screen);
        let depth = xlib::XDefaultDepth(display, screen);
        let _colormap = xlib::XDefaultColormap(display, screen);

        let black_pixel = xlib::XBlackPixel(display, screen);

        let width = xlib::XDisplayWidth(display, screen) as u32;
        let height = xlib::XDisplayHeight(display, screen) as u32;

        // let mut color : xlib::XColor = std::mem::zeroed();
        // let mut dummy : xlib::XColor = std::mem::zeroed();
        // let colorname = CString::new("black").unwrap();
        // let color = xlib::XAllocNamedColor(display, colormap, colorname.as_ptr(), &mut color, &mut dummy) as u64;

        let mut attributes: xlib::XSetWindowAttributes = zeroed();
        attributes.background_pixel = black_pixel;
        attributes.override_redirect = xlib::True;
        attributes.event_mask = xlib::ExposureMask | xlib::KeyPressMask | xlib::VisibilityChangeMask;

        let window = xlib::XCreateWindow(display, root, 0, 0, width,
                                          height, 0, depth,
                                          xlib::CopyFromParent as u32,
                                          visual,
                                          xlib::CWOverrideRedirect |
                                          xlib::CWBackPixel |
                                          xlib::CWEventMask,
                                          &mut attributes);


        println!("Window: {}", window);
        xlib::XMapRaised(display, window);

        // Grab Keyboard
        for _ in 0..1000 {
            let grab = xlib::XGrabKeyboard(display, root, xlib::True, xlib::GrabModeAsync, xlib::GrabModeAsync, xlib::CurrentTime);
            if grab == xlib::GrabSuccess {
                println!("Grabbed keyboard");
                break;
            }
        }

        let mut password = "".to_string();

        // Main loop
        let mut event: xlib::XEvent = zeroed();
        loop {
            xlib::XNextEvent(display, &mut event);
            match event.get_type() {
                xlib::KeyPress => {
                    let mut input_char : libc::c_char = 0;
                    let mut ksym: xlib::KeySym = 0;
                    let mut key_event = xlib::XKeyEvent::from(event);
                    xlib::XLookupString(&mut key_event, &mut input_char, std::mem::size_of_val(&input_char) as i32, &mut ksym, null_mut());

                    const enter: u64 = keysym::XK_Return as u64;
                    match ksym {
                        enter => break,
                        _ => {
                            if libc::isprint(input_char as i32) != 0 {
                                let character = (input_char as u8) as char;
                                password = password + &character.to_string();
                            }
                        },
                    }
                }
                _ => {}
            }
        }

        println!("{}", password);
        xlib::XDestroyWindow(display, window);
        xlib::XCloseDisplay(display);
    }
}
