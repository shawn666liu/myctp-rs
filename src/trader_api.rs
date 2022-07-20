use crate::common::*;
use crate::ctp::*;
pub use std::ffi::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::rc::Rc;

pub struct TraderApi {
    api_ptr: *mut c_void,
    pub holder: Box<TraitsHolder>,
}

unsafe impl Send for TraderApi {}

impl Drop for TraderApi {
    fn drop(&mut self) {
        unsafe { TdDestroyApi(self.api_ptr) };
        println!("TraderApi::drop()");
    }
}

impl TraderApi {
    pub fn new(flow_path: &str, spi: Box<dyn CtpSpiTrait>) -> Self {
        let flow_path1 = std::ffi::CString::new(flow_path).unwrap();
        let flow_path_ptr = flow_path1.into_raw();
        let api_ptr = unsafe { TdCreateApi(flow_path_ptr) };
        let _ = unsafe { CString::from_raw(flow_path_ptr) };
        let result = TraderApi {
            api_ptr,
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

    pub fn get_api_version<'a>() -> &'a CStr {
        unsafe {
            let trading_day_cstr = TdGetApiVersion();
            return CStr::from_ptr(trading_day_cstr);
        }
    }

    pub fn init(&self) {
        // assert!(self.holder.is_some());
        unsafe { TdInit(self.api_ptr) };
    }

    // fn join(&mut self) -> ApiResult {
    //     from_api_return_to_api_result(unsafe { TdJoin(self.api_ptr) })
    // }

    pub fn get_trading_day<'a>(&self) -> &'a CStr {
        let trading_day_cstr = unsafe { TdGetTradingDay(self.api_ptr) };
        unsafe { CStr::from_ptr(trading_day_cstr) }
    }

    pub fn register_front(&self, front_address: &str) {
        let front_addr = std::ffi::CString::new(front_address).expect("CString::new failed");
        let front_socket_address_ptr = front_addr.into_raw();
        unsafe { TdRegisterFront(self.api_ptr, front_socket_address_ptr) };
        let front_addr = unsafe { CString::from_raw(front_socket_address_ptr) };
        drop(front_addr);
    }

    pub fn register_name_server(&self, name_server: &str) {
        let name_srv = std::ffi::CString::new(name_server).expect("CString::new failed");
        let name_server_ptr = name_srv.into_raw();
        unsafe { TdRegisterNameServer(self.api_ptr, name_server_ptr) };
        let name_srv = unsafe { CString::from_raw(name_server_ptr) };
        drop(name_srv);
    }

    pub fn register_fens_user_info(&self, fens_user_info: &CThostFtdcFensUserInfoField) {
        unsafe { TdRegisterFensUserInfo(self.api_ptr, fens_user_info) };
    }

    pub fn subscribe_private_topic(&self, resume_type: ResumeType) {
        unsafe { TdSubscribePrivateTopic(self.api_ptr, resume_type.into()) };
    }

    pub fn subscribe_public_topic(&self, resume_type: ResumeType) {
        unsafe { TdSubscribePublicTopic(self.api_ptr, resume_type.into()) };
    }

    pub fn req_authenticate(
        &self,
        req_authenticate: &CThostFtdcReqAuthenticateField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqAuthenticate(self.api_ptr, req_authenticate, request_id)
        })
    }

    pub fn req_user_login(
        &self,
        req_user_login: &CThostFtdcReqUserLoginField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqUserLogin(self.api_ptr, req_user_login, request_id)
        })
    }

    pub fn req_user_logout(
        &self,
        req_user_logout: &CThostFtdcUserLogoutField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqUserLogout(self.api_ptr, req_user_logout, request_id)
        })
    }

    pub fn req_user_password_update(
        &self,
        req_user_password_update: &CThostFtdcUserPasswordUpdateField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqUserPasswordUpdate(self.api_ptr, req_user_password_update, request_id)
        })
    }

    pub fn req_order_insert(
        &self,
        input_order: &CThostFtdcInputOrderField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqOrderInsert(self.api_ptr, input_order, request_id)
        })
    }

    pub fn req_order_action(
        &self,
        input_order_action: &CThostFtdcInputOrderActionField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqOrderAction(self.api_ptr, input_order_action, request_id)
        })
    }

    pub fn req_settlement_info_confirm(
        &self,
        settlement_info_confirm: &CThostFtdcSettlementInfoConfirmField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqSettlementInfoConfirm(self.api_ptr, settlement_info_confirm, request_id)
        })
    }

    pub fn req_qry_order(
        &self,
        qry_order: &CThostFtdcQryOrderField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe { TdReqQryOrder(self.api_ptr, qry_order, request_id) })
    }

    pub fn req_qry_trade(
        &self,
        qry_trade: &CThostFtdcQryTradeField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe { TdReqQryTrade(self.api_ptr, qry_trade, request_id) })
    }

    pub fn req_qry_investor_position(
        &self,
        qry_investor_position: &CThostFtdcQryInvestorPositionField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInvestorPosition(self.api_ptr, qry_investor_position, request_id)
        })
    }

    pub fn req_qry_trading_account(
        &self,
        qry_trading_account: &CThostFtdcQryTradingAccountField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryTradingAccount(self.api_ptr, qry_trading_account, request_id)
        })
    }

    pub fn req_qry_investor(
        &self,
        qry_investor: &CThostFtdcQryInvestorField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInvestor(self.api_ptr, qry_investor, request_id)
        })
    }

    pub fn req_qry_trading_code(
        &self,
        qry_trading_code: &CThostFtdcQryTradingCodeField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryTradingCode(self.api_ptr, qry_trading_code, request_id)
        })
    }

    pub fn req_qry_instrument_margin_rate(
        &self,
        qry_instrument_margin_rate: &CThostFtdcQryInstrumentMarginRateField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInstrumentMarginRate(self.api_ptr, qry_instrument_margin_rate, request_id)
        })
    }

    pub fn req_qry_instrument_commission_rate(
        &self,
        qry_instrument_commission_rate: &CThostFtdcQryInstrumentCommissionRateField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInstrumentCommissionRate(
                self.api_ptr,
                qry_instrument_commission_rate,
                request_id,
            )
        })
    }

    pub fn req_qry_exchange(
        &self,
        qry_exchange: &CThostFtdcQryExchangeField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryExchange(self.api_ptr, qry_exchange, request_id)
        })
    }

    pub fn req_qry_product(
        &self,
        qry_product: &CThostFtdcQryProductField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryProduct(self.api_ptr, qry_product, request_id)
        })
    }

    pub fn req_qry_instrument(
        &self,
        qry_instrument: &CThostFtdcQryInstrumentField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInstrument(self.api_ptr, qry_instrument, request_id)
        })
    }

    pub fn req_qry_classified_instrument(
        &self,
        qry_classified_instrument: &CThostFtdcQryClassifiedInstrumentField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryClassifiedInstrument(self.api_ptr, qry_classified_instrument, request_id)
        })
    }

    pub fn req_qry_settlement_info(
        &self,
        qry_settlement_info: &CThostFtdcQrySettlementInfoField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySettlementInfo(self.api_ptr, qry_settlement_info, request_id)
        })
    }

    pub fn req_qry_settlement_info_confirm(
        &self,
        qry_settlement_info_confirm: &CThostFtdcQrySettlementInfoConfirmField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySettlementInfoConfirm(self.api_ptr, qry_settlement_info_confirm, request_id)
        })
    }

    pub fn req_qry_exchange_margin_rate(
        &self,
        qry_exchange_margin_rate: &CThostFtdcQryExchangeMarginRateField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryExchangeMarginRate(self.api_ptr, qry_exchange_margin_rate, request_id)
        })
    }

    pub fn req_qry_exchange_margin_rate_adjust(
        &self,
        qry_exchange_margin_rate_adjust: &CThostFtdcQryExchangeMarginRateAdjustField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryExchangeMarginRateAdjust(
                self.api_ptr,
                qry_exchange_margin_rate_adjust,
                request_id,
            )
        })
    }

    pub fn req_qry_exchange_rate(
        &self,
        qry_exchange_rate: &CThostFtdcQryExchangeRateField,
        request_id: TThostFtdcRequestIDType,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryExchangeRate(self.api_ptr, qry_exchange_rate, request_id)
        })
    }
}
