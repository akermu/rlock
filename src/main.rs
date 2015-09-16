extern crate libc;
extern crate x11;

use std::env;
use std::ptr::{null, null_mut};
use std::mem::{zeroed, size_of_val};
use x11::xlib::{
    Display,
    Window
};
use x11::xlib;
use x11::keysym;
use libc::c_char;

mod auth;


fn main() {
    let euid = unsafe { libc::funcs::posix88::unistd::geteuid() };
    if euid != 0 {
        panic!("{}", "Must be run as root!");
    }
    
    let args = env::args().nth(1);
    let user = match args {
        None => panic!("Usage: rlock <username>"),
        Some(user) => user,
    };
    // let user = args.nth(1).unwrap();

    let display = unsafe { xlib::XOpenDisplay(null()) };
    if display == null_mut() {
        panic!("Can't open X11-Display.")
    }

    let window = lockscreen(display);

    // Grab Keyboard
    grab_keyboard(display);

    let mut password = "".to_string();

    // Main loop
    let mut event: xlib::XEvent = unsafe { zeroed() };
    loop {
        unsafe { xlib::XNextEvent(display, &mut event); }
        match event.get_type() {
            xlib::KeyPress => {
                let mut input_char : libc::c_char = 0;
                let mut ksym: xlib::KeySym = 0;
                let mut key_event = xlib::XKeyEvent::from(event);
                unsafe {
                    xlib::XLookupString(&mut key_event,
                                        &mut input_char,
                                        std::mem::size_of_val(&input_char) as i32,
                                        &mut ksym, null_mut())
                };

                match ksym as u32 {
                    keysym::XK_Return  => {
                        let auth = auth::auth_user(&user, &password);
                        if auth {
                            break;
                        } else {
                            password = "".to_string();
                        }
                    },
                    keysym::XK_BackSpace => {
                        if !password.is_empty() {
                            let new_len = password.len() - 1;
                            password.truncate(new_len);
                        }
                    },
                    _ => {
                        if isprint(input_char) {
                            let character = (input_char as u8) as char;
                            password = password + &character.to_string();
                        }
                    },
                }
            }
            _ => {}
        }
    }

    unsafe {
        xlib::XDestroyWindow(display, window);
        xlib::XCloseDisplay(display);
    }
}

fn grab_keyboard(display: *mut xlib::Display) -> bool {
    unsafe {
        let screen = xlib::XDefaultScreen(display);
        let root = xlib::XRootWindow(display, screen);

        for _ in 0..1000 {
            let grab = xlib::XGrabKeyboard(display,
                                           root,
                                           xlib::True,
                                           xlib::GrabModeAsync,
                                           xlib::GrabModeAsync,
                                           xlib::CurrentTime);
            return grab == xlib::GrabSuccess;
        }
    }
    false
}

fn lockscreen(display: *mut xlib::Display) -> xlib::Window {
    unsafe {
        let screen = xlib::XDefaultScreen(display);
        let root = xlib::XRootWindow(display, screen);
        let visual = xlib::XDefaultVisual(display, screen);
        let depth = xlib::XDefaultDepth(display, screen);

        let black_pixel = xlib::XBlackPixel(display, screen);

        let width = xlib::XDisplayWidth(display, screen) as u32;
        let height = xlib::XDisplayHeight(display, screen) as u32;

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

        xlib::XMapRaised(display, window);
        window
    }
}

fn isprint(c: c_char) -> bool { unsafe { libc::isprint(c as i32) != 0 }}
