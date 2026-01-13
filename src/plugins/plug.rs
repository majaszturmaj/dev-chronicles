


use std::env;
use std::fs;
use std::ffi::CString;
use std::os::raw::c_char;

/// Funkcja eksportowana (widoczna dla innych języków)
#[no_mangle]
pub extern "C" fn plugin_hello() -> *const c_char {
    let msg = CString::new("Hello from Rust plugin").unwrap();
    msg.into_raw()
}

#[no_mangle]
pub extern "C" fn read_temp_file(filename: *const c_char) -> *const c_char {
    // Zamiana C string -> Rust
    let c_str = unsafe { CStr::from_ptr(filename) };
    let filename = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return CString::new("Invalid filename").unwrap().into_raw(),
    };

    let mut path = env::temp_dir();
    path.push(filename);

    match fs::read_to_string(path) {
        Ok(content) => CString::new(content).unwrap().into_raw(),
        Err(e) => CString::new(e.to_string()).unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(ptr));
    }
}

fn read_temp_file(filename: &str) -> Result<String, std::io::Error> {
    let mut path: PathBuf = env::temp_dir();
    path.push(filename);

    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

fn main() {
    match read_temp_file("browser_history.tmp") {
        Ok(data) => println!("{}", data),
        Err(e) => eprintln!("Błąd: {}", e),
    }
}

`# Chrome
C:\Users\<Username>\AppData\Local\Google\Chrome\User Data\Default\History
 
# Firefox
C:\Users\<Username>\AppData\Roaming\Mozilla\Firefox\Profiles\<ProfileName>\places.sqlite
 
# Edge
C:\Users\<Username>\AppData\Local\Microsoft\Edge\User Data\Default\History
`