pub use crate::ctp::CThostFtdcRspInfoField;
pub use crate::ctp::{from_rsp_info_to_rsp_result, RspResult};
pub use crate::ctp::{EnumOnErrRtnEvent, EnumOnFrontEvent, EnumOnRspEvent, EnumOnRtnEvent};
use std::any::Any;
pub use std::ffi::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint};
use std::rc::Rc;

/// 交易接口和行情接口都实现所有的trait
pub trait CtpSpiTrait: Send {
    // https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object
    fn as_any(&mut self) -> &mut dyn Any;

    fn on_err_rtn_event(
        &mut self,
        evt: EnumOnErrRtnEvent,
        param: *mut c_void,
        rsp_result: RspResult,
    ) {
        println!("==> on_err_rtn_event, {:?}, {:#?}", evt, rsp_result);
    }

    fn on_front_event(&mut self, evt: EnumOnFrontEvent, reason: i32) {
        println!("==> on_front_event, {:?}, reason {}", evt, reason);
    }

    fn on_rtn_event(&mut self, evt: EnumOnRtnEvent, param: *mut c_void) {
        println!("==> on_rtn_event, {:?}", evt);
    }

    fn on_rsp_event(
        &mut self,
        evt: EnumOnRspEvent,
        param: *mut c_void,
        rsp_result: RspResult,
        request_id: i32,
        is_last: bool,
    ) {
        // println!(
        //     "==> on_rsp_event, {:?}, {:#?} req_id {}, last? {}",
        //     evt, rsp_result, request_id, is_last
        // );
    }
}

pub fn cstring_slice_to_char_star_vec(cstring_vec: &[CString]) -> Vec<*const c_char> {
    cstring_vec.iter().map(|cstring| cstring.as_ptr()).collect()
}

pub fn str_slice_to_cstring_vec(str_vec: &[&str]) -> Vec<CString> {
    str_vec
        .iter()
        .map(|element| CString::new(*element).expect("CString::new failed"))
        .collect()
}

#[repr(C)]
pub struct TraitsHolder {
    pub spi: Box<dyn CtpSpiTrait>,
}

//# region global callback function
pub(crate) extern "C" fn cb_err_rtn_event(
    object: *mut c_void,
    evt: EnumOnErrRtnEvent,
    param: *mut c_void,
    rsp_info: *mut CThostFtdcRspInfoField,
) {
    let r = object as *mut TraitsHolder;
    unsafe {
        (*r).spi
            .on_err_rtn_event(evt, param, from_rsp_info_to_rsp_result(rsp_info));
    }
}

pub(crate) extern "C" fn cb_front_event(object: *mut c_void, evt: EnumOnFrontEvent, reason: c_int) {
    let r = object as *mut TraitsHolder;
    unsafe {
        (*r).spi.on_front_event(evt, reason as i32);
    }
}

pub(crate) extern "C" fn cb_rsp_event(
    object: *mut c_void,
    evt: EnumOnRspEvent,
    param: *mut c_void,
    rsp_info: *mut CThostFtdcRspInfoField,
    request_id: c_int,
    is_last: bool,
) {
    // println!(
    //     "==> 'c' on_rsp_event, {:?}, {:#?}, req_id {}, last? {}",
    //     evt, object, request_id, is_last
    // );

    let r = object as *mut TraitsHolder;
    unsafe {
        (*r).spi.on_rsp_event(
            evt,
            param,
            from_rsp_info_to_rsp_result(rsp_info),
            request_id,
            is_last,
        );
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
