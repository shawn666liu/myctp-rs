use crate::common::*;
use crate::ctp::*;
pub use std::ffi::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::rc::Rc;

pub struct MDApi {
    api_ptr: *mut c_void,
    pub holder: Box<TraitsHolder>,
}

unsafe impl Send for MDApi {}

impl Drop for MDApi {
    fn drop(&mut self) {
        unsafe { MdDestroyApi(self.api_ptr) };
        println!("MdApi::drop()");
    }
}

impl MDApi {
    pub fn new(
        flow_path: &str,
        use_udp: bool,
        use_multicast: bool,
        spi: Box<dyn CtpSpiTrait>,
    ) -> Self {
        let flow_path1 = std::ffi::CString::new(flow_path).expect("CString::new failed");
        let flow_path_ptr = flow_path1.into_raw();
        let api_ptr = unsafe { MdCreateApi(flow_path_ptr, use_udp, use_multicast, true) };
        let _ = unsafe { CString::from_raw(flow_path_ptr) };
        let result = MDApi {
            api_ptr,
            holder: Box::new(TraitsHolder { spi }),
        };
        unsafe {
            // register callbacks and spi
            let pp = result.holder.as_ref();
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

    pub fn get_api_version<'a>() -> &'a CStr {
        unsafe {
            let trading_day_cstr = MdGetApiVersion();
            return CStr::from_ptr(trading_day_cstr);
        }
    }

    pub fn init(&self) {
        unsafe {
            MdInit(self.api_ptr);
        }
    }

    pub fn get_trading_day<'a>(&self) -> &'a CStr {
        let trading_day_cstr = unsafe { MdGetTradingDay(self.api_ptr) };
        unsafe { CStr::from_ptr(trading_day_cstr) }
    }

    pub fn register_front(&self, front_address: &str) {
        let front_addr = std::ffi::CString::new(front_address).expect("CString::new failed");
        let front_socket_address_ptr = front_addr.into_raw();
        unsafe { MdRegisterFront(self.api_ptr, front_socket_address_ptr) };
        let front_addr = unsafe { CString::from_raw(front_socket_address_ptr) };
        drop(front_addr);
    }

    pub fn register_name_server(&self, name_server: &str) {
        let name_srv = std::ffi::CString::new(name_server).expect("CString::new failed");
        let name_server_ptr = name_srv.into_raw();
        unsafe { MdRegisterNameServer(self.api_ptr, name_server_ptr) };
        let name_srv = unsafe { CString::from_raw(name_server_ptr) };
        drop(name_srv);
    }

    pub fn register_fens_user_info(&self, fens_user_info: &CThostFtdcFensUserInfoField) {
        unsafe { MdRegisterFensUserInfo(self.api_ptr, fens_user_info) };
    }

    pub fn subscribe_market_data(&self, instrument_ids: &[&str]) -> ApiResult {
        let ids = str_slice_to_cstring_vec(instrument_ids);
        let v = cstring_slice_to_char_star_vec(&ids);
        from_api_return_to_api_result(unsafe {
            MdSubscribeMarketData(self.api_ptr, v.as_ptr(), v.len() as c_int)
        })
    }

    pub fn unsubscribe_market_data(&self, instrument_ids: &[&str]) -> ApiResult {
        let ids = str_slice_to_cstring_vec(instrument_ids);
        let v = cstring_slice_to_char_star_vec(&ids);
        from_api_return_to_api_result(unsafe {
            MdUnSubscribeMarketData(self.api_ptr, v.as_ptr(), v.len() as c_int)
        })
    }

    pub fn subscribe_for_quote_rsp(&self, instrument_ids: &[&str]) -> ApiResult {
        let ids = str_slice_to_cstring_vec(instrument_ids);
        let v = cstring_slice_to_char_star_vec(&ids);
        from_api_return_to_api_result(unsafe {
            MdSubscribeForQuoteRsp(self.api_ptr, v.as_ptr(), v.len() as c_int)
        })
    }

    pub fn unsubscribe_for_quote_rsp(&self, instrument_ids: &[&str]) -> ApiResult {
        let ids = str_slice_to_cstring_vec(instrument_ids);
        let v = cstring_slice_to_char_star_vec(&ids);
        from_api_return_to_api_result(unsafe {
            MdUnSubscribeForQuoteRsp(self.api_ptr, v.as_ptr(), v.len() as c_int)
        })
    }

    pub fn req_user_login(
        &self,
        req_user_login: &CThostFtdcReqUserLoginField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            MdReqUserLogin(self.api_ptr, req_user_login, request_id)
        })
    }

    pub fn req_user_logout(
        &self,
        req_user_logout: &CThostFtdcUserLogoutField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            MdReqUserLogout(self.api_ptr, req_user_logout, request_id)
        })
    }
}
