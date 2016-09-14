use std::mem::zeroed;
use std::ffi::CString;
use std::iter::repeat;
use x11::xlib;
use x11::xft;


/// Window struct
pub struct Lockscreen<'a> {
    username: &'a str,
    display: *mut xlib::Display,
    window: xlib::Window,
    font: *mut xft::XftFont,
    draw: *mut xft::XftDraw,
    font_color: xft::XftColor,
    bg_color: xft::XftColor,
}

impl<'a> Lockscreen<'a> {
    pub fn new(display: *mut xlib::Display, username: &'a str) -> Lockscreen {
        unsafe {
            let screen = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen);
            let visual = xlib::XDefaultVisual(display, screen);
            let depth = xlib::XDefaultDepth(display, screen);
            let colormap = xlib::XDefaultColormap(display, screen);

            let black_pixel = xlib::XBlackPixel(display, screen);
            let width = xlib::XDisplayWidth(display, screen) as u32;
            let height = xlib::XDisplayHeight(display, screen) as u32;

            let mut attributes: xlib::XSetWindowAttributes = zeroed();
            attributes.background_pixel = black_pixel;
            attributes.override_redirect = xlib::True;

            let window = xlib::XCreateWindow(display, root, 0, 0,
                                             width, height, 0, depth,
                                             xlib::CopyFromParent as u32,
                                             visual,
                                             xlib::CWOverrideRedirect |
                                             xlib::CWBackPixel,
                                             &mut attributes);


            let draw = xft::XftDrawCreate(display,
                                          window,
                                          visual,
                                          colormap);

            let font_name = CString::new("Inconsolata").unwrap();
            let font = xft::XftFontOpenName(display,
                                            screen,
                                            font_name.as_ptr());
            let mut font_color:  xft::XftColor = zeroed();
            let color_name = CString::new("White").unwrap();
            xft::XftColorAllocName(display,
                                   visual,
                                   colormap,
                                   color_name.as_ptr(),
                                   &mut font_color);

            let mut bg_color:  xft::XftColor = zeroed();
            let color_name = CString::new("Black").unwrap();
            xft::XftColorAllocName(display,
                                   visual,
                                   colormap,
                                   color_name.as_ptr(),
                                   &mut bg_color);

            Lockscreen {
                window: window,
                display: display,
                username: username,
                font: font,
                draw: draw,
                font_color: font_color,
                bg_color: bg_color,
            }
        }
    }

    pub fn show(&self) -> () {
        unsafe {
            xlib::XMapRaised(self.display, self.window);
        };
    }

    pub fn set_password_len(&self, len: usize) -> () {
        let str: String = repeat::<char>('*').take(len).collect();
        let len = str.len();
        let text = CString::new(str).unwrap();

        unsafe {
            xft::XftDrawRect(self.draw,
                             &self.bg_color,
                             100, 20, 2000, 2000);
            xft::XftDrawStringUtf8(self.draw,
                                   &self.font_color,
                                   self.font,
                                   100, 40,
                                   text.as_ptr() as *const u8,
                                   len as i32);
        };
    }

    pub fn write_screen(&self) -> () {
        unsafe {
            let str = "User: ".to_string() + self.username;
            let len = str.len();
            let text = CString::new(str).unwrap();

            xft::XftDrawStringUtf8(self.draw,
                                   &self.font_color,
                                   self.font,
                                   20, 20,
                                   text.as_ptr() as *const u8,
                                   len as i32);

            let str = "Password: ".to_string();
            let len = str.len();
            let text = CString::new(str).unwrap();

            xft::XftDrawStringUtf8(self.draw,
                                   &self.font_color,
                                   self.font,
                                   20, 40,
                                   text.as_ptr() as *const u8,
                                   len as i32);
            // TODO: Must be destroyed.
        }
    }
}

impl<'a> Drop for Lockscreen<'a> {
    fn drop(&mut self) {
        unsafe { xlib::XDestroyWindow(self.display, self.window) };
    }
}
