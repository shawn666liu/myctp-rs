use crate::common::*;
use crate::ctp::*;
use std::ffi::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::rc::Rc;

use crate::md_api::MDApi;
impl MDApi {
    pub fn get_api_version() -> &'static CStr {
        unsafe {
            let tmp = MdGetApiVersion();
            return CStr::from_ptr(tmp);
        }
    }

    pub fn get_trading_day<'a>(&self) -> &'a CStr {
        unsafe {
            let tmp = MdGetTradingDay(self.quoter_ptr);
            return CStr::from_ptr(tmp);
        }
    }

    pub fn register_front(&self, front_address: &str) {
        let front_address_1 = std::ffi::CString::new(front_address).expect("CString::new failed");
        let front_address_1_ = front_address_1.into_raw();
        unsafe { MdRegisterFront(self.quoter_ptr, front_address_1_) };
        let front_address_1 = unsafe { CString::from_raw(front_address_1_) };
        drop(front_address_1);
    }

    pub fn register_name_server(&self, ns_address: &str) {
        let ns_address_1 = std::ffi::CString::new(ns_address).expect("CString::new failed");
        let ns_address_1_ = ns_address_1.into_raw();
        unsafe { MdRegisterNameServer(self.quoter_ptr, ns_address_1_) };
        let ns_address_1 = unsafe { CString::from_raw(ns_address_1_) };
        drop(ns_address_1);
    }

    pub fn register_fens_user_info(&self, fens_user_info: &CThostFtdcFensUserInfoField) {
        unsafe { MdRegisterFensUserInfo(self.quoter_ptr, fens_user_info) };
    }

    pub fn subscribe_market_data(&self, instrument_id: &[&str]) -> ApiResult {
        let instrument_id_1 = str_slice_to_cstring_vec(instrument_id);
        let instrument_id_1_v = cstring_slice_to_char_star_vec(&instrument_id_1);
        from_api_return_to_api_result(unsafe {
            MdSubscribeMarketData(
                self.quoter_ptr,
                instrument_id_1_v.as_ptr(),
                instrument_id_1_v.len() as c_int,
            )
        })
    }

    pub fn unsubscribe_market_data(&self, instrument_id: &[&str]) -> ApiResult {
        let instrument_id_1 = str_slice_to_cstring_vec(instrument_id);
        let instrument_id_1_v = cstring_slice_to_char_star_vec(&instrument_id_1);
        from_api_return_to_api_result(unsafe {
            MdUnSubscribeMarketData(
                self.quoter_ptr,
                instrument_id_1_v.as_ptr(),
                instrument_id_1_v.len() as c_int,
            )
        })
    }

    pub fn subscribe_for_quote_rsp(&self, instrument_id: &[&str]) -> ApiResult {
        let instrument_id_1 = str_slice_to_cstring_vec(instrument_id);
        let instrument_id_1_v = cstring_slice_to_char_star_vec(&instrument_id_1);
        from_api_return_to_api_result(unsafe {
            MdSubscribeForQuoteRsp(
                self.quoter_ptr,
                instrument_id_1_v.as_ptr(),
                instrument_id_1_v.len() as c_int,
            )
        })
    }

    pub fn unsubscribe_for_quote_rsp(&self, instrument_id: &[&str]) -> ApiResult {
        let instrument_id_1 = str_slice_to_cstring_vec(instrument_id);
        let instrument_id_1_v = cstring_slice_to_char_star_vec(&instrument_id_1);
        from_api_return_to_api_result(unsafe {
            MdUnSubscribeForQuoteRsp(
                self.quoter_ptr,
                instrument_id_1_v.as_ptr(),
                instrument_id_1_v.len() as c_int,
            )
        })
    }

    pub fn req_user_login(
        &self,
        req_user_login_field: &CThostFtdcReqUserLoginField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            MdReqUserLogin(self.quoter_ptr, req_user_login_field, request_id)
        })
    }

    pub fn req_user_logout(
        &self,
        user_logout: &CThostFtdcUserLogoutField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            MdReqUserLogout(self.quoter_ptr, user_logout, request_id)
        })
    }

    pub fn req_qry_multicast_instrument(
        &self,
        qry_multicast_instrument: &CThostFtdcQryMulticastInstrumentField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            MdReqQryMulticastInstrument(self.quoter_ptr, qry_multicast_instrument, request_id)
        })
    }
}
