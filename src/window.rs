use std::mem::zeroed;
use x11::xlib;


pub fn lockscreen(display: *mut xlib::Display) -> xlib::Window {
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

