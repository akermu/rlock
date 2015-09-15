use libc::{ c_char, c_long, c_ulong};
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr::null_mut;
use std::str;

#[repr(C)]
#[derive(Copy)]
struct Spwd {
    name: *mut c_char,
    password: *mut c_char,
    sp_lstchg: c_long,
    sp_min: c_long,
    sp_max: c_long,
    sp_warn: c_long,
    sp_inact: c_long,
    sp_expire: c_long,
    sp_flag: c_ulong,
}

impl Spwd {
    pub fn get_password(&self) -> &str {
        let slice = unsafe { CStr::from_ptr(self.password).to_bytes() };
        str::from_utf8(slice).unwrap()
    }
}

impl ::std::clone::Clone for Spwd {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Spwd {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

extern "C" {
    fn getspnam(name: *const c_char) -> *mut Spwd;
}


#[link(name = "crypt")]
extern "C" {
    fn crypt(key: *const c_char, salt: *const c_char) -> *mut c_char;
}

pub fn auth_user(user: &str, password: &str) -> bool {

    let entry = unsafe {
        let shadow = getspnam(CString::new(user).unwrap().as_ptr());
        if shadow != null_mut()  {
            (*shadow).get_password()
        } else {
            panic!("Can't get password from /etc/shadow for user {}.", user);
        }
    };

    let parts: Vec<&str> = entry.split('$').collect();
    let salt = format!("${}${}$", parts[1], parts[2]);

    let enc_pass = unsafe {
        let salt = CString::new(salt).unwrap();
        let password = CString::new(password).unwrap();
        let pass = crypt(password.as_ptr(), salt.as_ptr());
        //TODO: Zero out password
        str::from_utf8(CStr::from_ptr(pass).to_bytes()).unwrap()
    };
    
    enc_pass == entry
}
