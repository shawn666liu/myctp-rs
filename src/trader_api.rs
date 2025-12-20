use crate::common::*;
use crate::ctp::*;
pub use std::ffi::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::rc::Rc;

pub struct TraderApi {
    pub(crate) trader_ptr: *mut c_void,
    pub holder: Box<TraitsHolder>,
}

unsafe impl Send for TraderApi {}

impl Drop for TraderApi {
    fn drop(&mut self) {
        unsafe { TdDestroyApi(self.trader_ptr) };
        println!("TraderApi::drop()");
    }
}

impl TraderApi {
    pub fn new(flow_path: &str, product_mode: bool, spi: Box<dyn CtpSpiTrait>) -> Self {
        let flow_path1 = std::ffi::CString::new(flow_path).unwrap();
        let flow_path_ptr = flow_path1.into_raw();
        let api_ptr = unsafe { TdCreateApi(flow_path_ptr, product_mode) };
        let _ = unsafe { CString::from_raw(flow_path_ptr) };
        let result = TraderApi {
            trader_ptr: api_ptr,
            holder: Box::new(TraitsHolder { spi }),
        };
        unsafe {
            // register callbacks and spi
            let pp = result.holder.as_ref();
            let spi_raw_ptr = pp as *const TraitsHolder as *mut c_void;
            TdRegisterCallback(
                api_ptr,
                Some(cb_err_rtn_event),
                Some(cb_front_event),
                Some(cb_rsp_event),
                Some(cb_rtn_event),
                spi_raw_ptr,
            );
        }
        result
    }

    pub fn init(&self) {
        unsafe { TdInit(self.trader_ptr) };
    }
}
