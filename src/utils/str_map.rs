use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use std::sync::Once;

static mut STRMAP: Option<HashMap<StringType, CString>> = None;
static INIT_STRMAP: Once = Once::new();

#[derive(Hash, Eq, PartialEq)]
enum StringType {
    Static(&'static str),
    Owned(String),
}

fn init() {
    INIT_STRMAP.call_once(|| unsafe {
        STRMAP = Some(HashMap::new());
    });
}

pub fn get_static(s: &'static str) -> (*const c_char, usize) {
    init();

    let st = StringType::Static(s);

    if let Some(map) = unsafe { STRMAP.as_mut() } {
        match st {
            StringType::Static(s) => {
                if !map.contains_key(&st) {
                    let cs = CString::new(s).expect("CString::new failed");
                    map.insert(StringType::Static(s), cs);
                }
                let cs = map.get(&st).expect("CString not found");
                return (cs.as_ptr(), s.len());
            }
            _ => {}
        }
    }

    let res: (*const c_char, usize) = (ptr::null(), 0);
    return res;
}

pub fn get_owned(s: String) -> (*const c_char, usize) {
    init();

    let st = StringType::Owned(s);

    if let Some(map) = unsafe { STRMAP.as_mut() } {
        match st {
            StringType::Owned(ref s) => {
                if !map.contains_key(&st) {
                    let cs = CString::new(s.as_str()).expect("CString::new failed");
                    map.insert(StringType::Owned(s.clone()), cs);
                }
                let cs = map.get(&st).expect("CString not found");
                return (cs.as_ptr(), s.len());
            }
            _ => {}
        }
    }

    let res: (*const c_char, usize) = (ptr::null(), 0);
    return res;
}
