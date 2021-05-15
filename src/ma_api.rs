use crate::common::*;
use crate::ctp::*;
pub use std::ffi::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::rc::Rc;

pub struct MDApi {
    api_ptr: *mut c_void,
    holder: Option<TraitsHolder>,
}

impl MDApi {
    pub fn new(flow_path: CString, use_udp: bool, use_multicast: bool) -> Self {
        let flow_path_ptr = flow_path.into_raw();
        let api_ptr = unsafe { MdCreateApi(flow_path_ptr, use_udp, use_multicast) };
        let _ = unsafe { CString::from_raw(flow_path_ptr) };
        let result = MDApi {
            api_ptr,
            holder: None,
        };
        unsafe {
            MdRegisterCallback(
                api_ptr,
                Some(cb_front_event),
                Some(cb_rtn_rsp_event),
                Some(cb_rtn_event),
                api_ptr,
            );
        }
        result
    }

    pub fn register_spi(&mut self, spi: Box<dyn CtpSpiTrait>) {
        let holder = TraitsHolder { spi };
        let _ = self.holder.take();
        self.holder = Some(holder);
        if let Some(p) = &self.holder {
            let raw_ptr = p as *const TraitsHolder as *mut c_void;
            unsafe { MdSetObject(self.api_ptr, raw_ptr) };
        }
    }

    pub fn init(&mut self) {
        assert!(self.holder.is_some());
        unsafe {
            MdInit(self.api_ptr);
        }
    }

    pub fn get_trading_day<'a>(&mut self) -> &'a CStr {
        let trading_day_cstr = unsafe { MdGetTradingDay(self.api_ptr) };
        unsafe { CStr::from_ptr(trading_day_cstr) }
    }

    pub fn register_front(&mut self, front_socket_address: CString) {
        let front_socket_address_ptr = front_socket_address.into_raw();
        unsafe { MdRegisterFront(self.api_ptr, front_socket_address_ptr) };
        let front_socket_address = unsafe { CString::from_raw(front_socket_address_ptr) };
        drop(front_socket_address);
    }

    pub fn register_name_server(&mut self, name_server: CString) {
        let name_server_ptr = name_server.into_raw();
        unsafe { MdRegisterNameServer(self.api_ptr, name_server_ptr) };
        let name_server = unsafe { CString::from_raw(name_server_ptr) };
        drop(name_server);
    }

    pub fn register_fens_user_info(&mut self, fens_user_info: &CThostFtdcFensUserInfoField) {
        unsafe { MdRegisterFensUserInfo(self.api_ptr, fens_user_info) };
    }

    pub fn subscribe_market_data(&mut self, instrument_ids: &[CString]) -> ApiResult {
        let v = cstring_slice_to_char_star_vec(instrument_ids);
        from_api_return_to_api_result(unsafe {
            MdSubscribeMarketData(self.api_ptr, v.as_ptr(), v.len() as c_int)
        })
    }

    pub fn unsubscribe_market_data(&mut self, instrument_ids: &[CString]) -> ApiResult {
        let v = cstring_slice_to_char_star_vec(instrument_ids);
        from_api_return_to_api_result(unsafe {
            MdUnSubscribeMarketData(self.api_ptr, v.as_ptr(), v.len() as c_int)
        })
    }

    pub fn subscribe_for_quote_rsp(&mut self, instrument_ids: &[CString]) -> ApiResult {
        let v = cstring_slice_to_char_star_vec(instrument_ids);
        from_api_return_to_api_result(unsafe {
            MdSubscribeForQuoteRsp(self.api_ptr, v.as_ptr(), v.len() as c_int)
        })
    }

    pub fn unsubscribe_for_quote_rsp(&mut self, instrument_ids: &[CString]) -> ApiResult {
        let v = cstring_slice_to_char_star_vec(instrument_ids);
        from_api_return_to_api_result(unsafe {
            MdUnSubscribeForQuoteRsp(self.api_ptr, v.as_ptr(), v.len() as c_int)
        })
    }

    pub fn req_user_login(
        &mut self,
        req_user_login: &CThostFtdcReqUserLoginField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            MdReqUserLogin(self.api_ptr, req_user_login, request_id)
        })
    }

    pub fn req_user_logout(
        &mut self,
        req_user_logout: &CThostFtdcUserLogoutField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            MdReqUserLogout(self.api_ptr, req_user_logout, request_id)
        })
    }
}

impl Drop for MDApi {
    fn drop(&mut self) {
        unsafe { MdDestroyApi(self.api_ptr) };
        println!("MdApi dropped");
    }
}
