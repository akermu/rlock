extern crate libc;
extern crate x11;
extern crate xft_sys;

use libc::c_char;
use std::env;
use std::ptr::{null, null_mut};
use std::mem::zeroed;
use x11::xlib;
use x11::keysym;

mod auth;
mod window;


fn main() {
    let euid = unsafe { libc::geteuid() };
    if euid != 0 {
        panic!("{}", "Must be run as setuid root binary!");
    }

    let args = env::args().nth(1);
    let user = match args {
        None => env::var("USER").unwrap(),
        Some(user) => user,
    };

    let hashed_password = auth::get_hashed_password(&user);

    // Drop privileges
    unsafe {
        libc::setgroups(0, null());
        libc::setgid(libc::getgid());
        libc::setuid(libc::getuid());
    }

    let display = unsafe { xlib::XOpenDisplay(null()) };
    if display == null_mut() {
        panic!("Can't open X11-Display.")
    }

    let ls = window::Lockscreen::new(display, &user);
    ls.show();

    // Grab Keyboard
    grab_keyboard(display);

    let mut password = "".to_string();

    // Main loop
    let mut event: xlib::XEvent = unsafe { zeroed() };
    ls.write_screen();
    loop {
        unsafe {
            let ret = xlib::XNextEvent(display, &mut event);
            if ret != 0 {
                break;
            }
        };
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
                        let auth = auth::validate(&password, &hashed_password);
                        if auth {
                            break;
                        } else {
                            password = "".to_string();
                            ls.set_password_len(0);
                        }
                    },
                    keysym::XK_BackSpace => {
                        if !password.is_empty() {
                            let new_len = password.len() - 1;
                            password.truncate(new_len);
                            ls.set_password_len(new_len);
                        }
                    },
                    _ => {
                        if isprint(input_char) {
                            let character = (input_char as u8) as char;
                            password = password + &character.to_string();
                            ls.set_password_len(password.len());
                        }
                    },
                }
            }
            _ => {}
        };
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

fn isprint(c: c_char) -> bool { unsafe { libc::isprint(c as i32) != 0 }}
