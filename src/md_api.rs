use crate::common::*;
use crate::ctp::*;
pub use std::ffi::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::rc::Rc;

pub struct MDApi {
    pub(crate) quoter_ptr: *mut c_void,
    pub holder: Box<TraitsHolder>,
}

unsafe impl Send for MDApi {}

impl Drop for MDApi {
    fn drop(&mut self) {
        unsafe { MdDestroyApi(self.quoter_ptr) };
        println!("MdApi::drop()");
    }
}

impl MDApi {
    pub fn new(
        flow_path: &str,
        use_udp: bool,
        use_multicast: bool,
        product_mode: bool,
        spi: Box<dyn CtpSpiTrait>,
    ) -> Self {
        let flow_path1 = std::ffi::CString::new(flow_path).expect("CString::new failed");
        let flow_path_ptr = flow_path1.into_raw();
        let api_ptr = unsafe { MdCreateApi(flow_path_ptr, use_udp, use_multicast, product_mode) };
        let _ = unsafe { CString::from_raw(flow_path_ptr) };
        let result = MDApi {
            quoter_ptr: api_ptr,
            holder: Box::new(TraitsHolder { spi }),
        };
        unsafe {
            // register callbacks and spi
            let pp: &TraitsHolder = result.holder.as_ref();
            let spi_raw_ptr = pp as *const TraitsHolder as *mut c_void;
            MdRegisterCallback(
                api_ptr,
                Some(cb_front_event),
                Some(cb_rsp_event),
                Some(cb_rtn_event),
                spi_raw_ptr,
            );
        }
        result
    }

    pub fn init(&self) {
        unsafe {
            MdInit(self.quoter_ptr);
        }
    }
}
