pub use crate::ctp::{EnumOnErrRtnEvent, EnumOnFrontEvent, EnumOnRspEvent, EnumOnRtnEvent};
pub use std::ffi::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint};
use std::rc::Rc;

/// 交易接口和行情接口都实现所有的trait
pub trait CtpSpiTrait {
    fn on_err_rtn_event(
        &mut self,
        evt: EnumOnErrRtnEvent,
        param: *mut c_void,
        psp_info: *mut c_void,
    ) {
        println!("==> on_err_rtn_event, {:?}", evt);
    }

    fn on_front_event(&mut self, evt: EnumOnFrontEvent, reason: i32) {
        println!("==> on_front_event, {:?}, reason {}", evt, reason);
    }

    fn on_rtn_event(&mut self, evt: EnumOnRtnEvent, param: *mut c_void) {
        println!("==> on_rtn_event, {:?}", evt);
    }

    fn on_rtn_rsp_event(
        &mut self,
        evt: EnumOnRspEvent,
        param: *mut c_void,
        rsp_info: *mut c_void,
        request_id: i32,
        is_last: bool,
    ) {
        println!(
            "==> on_rtn_rsp_event, {:?}, req_id {}, last? {}",
            evt, request_id, is_last
        );
    }
}

pub fn cstring_slice_to_char_star_vec(cstring_vec: &[CString]) -> Vec<*const c_char> {
    cstring_vec.iter().map(|cstring| cstring.as_ptr()).collect()
}

#[repr(C)]
pub struct TraitsHolder {
    pub(crate) spi: Box<dyn CtpSpiTrait>,
}

//# region global callback function
pub(crate) extern "C" fn cb_on_err_rtn_event(
    object: *mut c_void,
    evt: EnumOnErrRtnEvent,
    param: *mut c_void,
    rsp_info: *mut c_void,
) {
    let r = object as *mut TraitsHolder;
    unsafe {
        (*r).spi.on_err_rtn_event(evt, param, rsp_info);
    }
}

pub(crate) extern "C" fn cb_front_event(object: *mut c_void, evt: EnumOnFrontEvent, reason: c_int) {
    let r = object as *mut TraitsHolder;
    unsafe {
        (*r).spi.on_front_event(evt, reason as i32);
    }
}

pub(crate) extern "C" fn cb_rtn_rsp_event(
    object: *mut c_void,
    evt: EnumOnRspEvent,
    param: *mut c_void,
    rsp_info: *mut c_void,
    request_id: c_int,
    is_last: bool,
) {
    let r = object as *mut TraitsHolder;
    unsafe {
        (*r).spi
            .on_rtn_rsp_event(evt, param, rsp_info, request_id, is_last);
    }
}

pub(crate) extern "C" fn cb_rtn_event(
    object: *mut c_void,
    evt: EnumOnRtnEvent,
    param: *mut c_void,
) {
    let r = object as *mut TraitsHolder;
    unsafe {
        (*r).spi.on_rtn_event(evt, param);
    }
}
//#end region
