use crate::common::*;
use crate::ctp::*;
use std::ffi::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::rc::Rc;

use crate::trader_api::TraderApi;
impl TraderApi {
    pub fn get_api_version() -> &'static CStr {
        unsafe {
            let tmp = TdGetApiVersion();
            return CStr::from_ptr(tmp);
        }
    }

    pub fn get_trading_day<'a>(&self) -> &'a CStr {
        unsafe {
            let tmp = TdGetTradingDay(self.trader_ptr);
            return CStr::from_ptr(tmp);
        }
    }

    pub fn get_front_info(&self, front_info: &mut CThostFtdcFrontInfoField) {
        unsafe {
            TdGetFrontInfo(self.trader_ptr, front_info);
        }
    }

    pub fn register_front(&self, front_address: &str) {
        let front_address_1 = std::ffi::CString::new(front_address).expect("CString::new failed");
        let front_address_1_ = front_address_1.into_raw();
        unsafe { TdRegisterFront(self.trader_ptr, front_address_1_) };
        let front_address_1 = unsafe { CString::from_raw(front_address_1_) };
        drop(front_address_1);
    }

    pub fn register_name_server(&self, ns_address: &str) {
        let ns_address_1 = std::ffi::CString::new(ns_address).expect("CString::new failed");
        let ns_address_1_ = ns_address_1.into_raw();
        unsafe { TdRegisterNameServer(self.trader_ptr, ns_address_1_) };
        let ns_address_1 = unsafe { CString::from_raw(ns_address_1_) };
        drop(ns_address_1);
    }

    pub fn register_fens_user_info(&self, fens_user_info: &CThostFtdcFensUserInfoField) {
        unsafe { TdRegisterFensUserInfo(self.trader_ptr, fens_user_info) };
    }

    pub fn subscribe_private_topic(&self, resume_type: THOST_TE_RESUME_TYPE) {
        unsafe { TdSubscribePrivateTopic(self.trader_ptr, resume_type) }
    }

    pub fn subscribe_public_topic(&self, resume_type: THOST_TE_RESUME_TYPE) {
        unsafe { TdSubscribePublicTopic(self.trader_ptr, resume_type) }
    }

    pub fn req_authenticate(
        &self,
        req_authenticate_field: &CThostFtdcReqAuthenticateField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqAuthenticate(self.trader_ptr, req_authenticate_field, request_id)
        })
    }

    pub fn register_user_system_info(
        &self,
        user_system_info: &CThostFtdcUserSystemInfoField,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdRegisterUserSystemInfo(self.trader_ptr, user_system_info)
        })
    }

    pub fn submit_user_system_info(
        &self,
        user_system_info: &CThostFtdcUserSystemInfoField,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdSubmitUserSystemInfo(self.trader_ptr, user_system_info)
        })
    }

    pub fn register_wechat_user_system_info(
        &self,
        user_system_info: &CThostFtdcWechatUserSystemInfoField,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdRegisterWechatUserSystemInfo(self.trader_ptr, user_system_info)
        })
    }

    pub fn submit_wechat_user_system_info(
        &self,
        user_system_info: &CThostFtdcWechatUserSystemInfoField,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdSubmitWechatUserSystemInfo(self.trader_ptr, user_system_info)
        })
    }

    pub fn req_user_login(
        &self,
        req_user_login_field: &CThostFtdcReqUserLoginField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqUserLogin(self.trader_ptr, req_user_login_field, request_id)
        })
    }

    pub fn req_user_logout(
        &self,
        user_logout: &CThostFtdcUserLogoutField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqUserLogout(self.trader_ptr, user_logout, request_id)
        })
    }

    pub fn req_user_password_update(
        &self,
        user_password_update: &CThostFtdcUserPasswordUpdateField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqUserPasswordUpdate(self.trader_ptr, user_password_update, request_id)
        })
    }

    pub fn req_trading_account_password_update(
        &self,
        trading_account_password_update: &CThostFtdcTradingAccountPasswordUpdateField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqTradingAccountPasswordUpdate(
                self.trader_ptr,
                trading_account_password_update,
                request_id,
            )
        })
    }

    pub fn req_user_auth_method(
        &self,
        req_user_auth_method: &CThostFtdcReqUserAuthMethodField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqUserAuthMethod(self.trader_ptr, req_user_auth_method, request_id)
        })
    }

    pub fn req_gen_user_captcha(
        &self,
        req_gen_user_captcha: &CThostFtdcReqGenUserCaptchaField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqGenUserCaptcha(self.trader_ptr, req_gen_user_captcha, request_id)
        })
    }

    pub fn req_gen_user_text(
        &self,
        req_gen_user_text: &CThostFtdcReqGenUserTextField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqGenUserText(self.trader_ptr, req_gen_user_text, request_id)
        })
    }

    pub fn req_user_login_with_captcha(
        &self,
        req_user_login_with_captcha: &CThostFtdcReqUserLoginWithCaptchaField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqUserLoginWithCaptcha(self.trader_ptr, req_user_login_with_captcha, request_id)
        })
    }

    pub fn req_user_login_with_text(
        &self,
        req_user_login_with_text: &CThostFtdcReqUserLoginWithTextField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqUserLoginWithText(self.trader_ptr, req_user_login_with_text, request_id)
        })
    }

    pub fn req_user_login_with_otp(
        &self,
        req_user_login_with_otp: &CThostFtdcReqUserLoginWithOTPField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqUserLoginWithOTP(self.trader_ptr, req_user_login_with_otp, request_id)
        })
    }

    pub fn req_order_insert(
        &self,
        input_order: &CThostFtdcInputOrderField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqOrderInsert(self.trader_ptr, input_order, request_id)
        })
    }

    pub fn req_parked_order_insert(
        &self,
        parked_order: &CThostFtdcParkedOrderField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqParkedOrderInsert(self.trader_ptr, parked_order, request_id)
        })
    }

    pub fn req_parked_order_action(
        &self,
        parked_order_action: &CThostFtdcParkedOrderActionField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqParkedOrderAction(self.trader_ptr, parked_order_action, request_id)
        })
    }

    pub fn req_order_action(
        &self,
        input_order_action: &CThostFtdcInputOrderActionField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqOrderAction(self.trader_ptr, input_order_action, request_id)
        })
    }

    pub fn req_qry_max_order_volume(
        &self,
        qry_max_order_volume: &CThostFtdcQryMaxOrderVolumeField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryMaxOrderVolume(self.trader_ptr, qry_max_order_volume, request_id)
        })
    }

    pub fn req_settlement_info_confirm(
        &self,
        settlement_info_confirm: &CThostFtdcSettlementInfoConfirmField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqSettlementInfoConfirm(self.trader_ptr, settlement_info_confirm, request_id)
        })
    }

    pub fn req_remove_parked_order(
        &self,
        remove_parked_order: &CThostFtdcRemoveParkedOrderField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqRemoveParkedOrder(self.trader_ptr, remove_parked_order, request_id)
        })
    }

    pub fn req_remove_parked_order_action(
        &self,
        remove_parked_order_action: &CThostFtdcRemoveParkedOrderActionField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqRemoveParkedOrderAction(self.trader_ptr, remove_parked_order_action, request_id)
        })
    }

    pub fn req_exec_order_insert(
        &self,
        input_exec_order: &CThostFtdcInputExecOrderField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqExecOrderInsert(self.trader_ptr, input_exec_order, request_id)
        })
    }

    pub fn req_exec_order_action(
        &self,
        input_exec_order_action: &CThostFtdcInputExecOrderActionField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqExecOrderAction(self.trader_ptr, input_exec_order_action, request_id)
        })
    }

    pub fn req_for_quote_insert(
        &self,
        input_for_quote: &CThostFtdcInputForQuoteField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqForQuoteInsert(self.trader_ptr, input_for_quote, request_id)
        })
    }

    pub fn req_quote_insert(
        &self,
        input_quote: &CThostFtdcInputQuoteField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQuoteInsert(self.trader_ptr, input_quote, request_id)
        })
    }

    pub fn req_quote_action(
        &self,
        input_quote_action: &CThostFtdcInputQuoteActionField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQuoteAction(self.trader_ptr, input_quote_action, request_id)
        })
    }

    pub fn req_batch_order_action(
        &self,
        input_batch_order_action: &CThostFtdcInputBatchOrderActionField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqBatchOrderAction(self.trader_ptr, input_batch_order_action, request_id)
        })
    }

    pub fn req_option_self_close_insert(
        &self,
        input_option_self_close: &CThostFtdcInputOptionSelfCloseField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqOptionSelfCloseInsert(self.trader_ptr, input_option_self_close, request_id)
        })
    }

    pub fn req_option_self_close_action(
        &self,
        input_option_self_close_action: &CThostFtdcInputOptionSelfCloseActionField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqOptionSelfCloseAction(self.trader_ptr, input_option_self_close_action, request_id)
        })
    }

    pub fn req_comb_action_insert(
        &self,
        input_comb_action: &CThostFtdcInputCombActionField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqCombActionInsert(self.trader_ptr, input_comb_action, request_id)
        })
    }

    pub fn req_qry_order(&self, qry_order: &CThostFtdcQryOrderField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryOrder(self.trader_ptr, qry_order, request_id)
        })
    }

    pub fn req_qry_trade(&self, qry_trade: &CThostFtdcQryTradeField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryTrade(self.trader_ptr, qry_trade, request_id)
        })
    }

    pub fn req_qry_investor_position(
        &self,
        qry_investor_position: &CThostFtdcQryInvestorPositionField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInvestorPosition(self.trader_ptr, qry_investor_position, request_id)
        })
    }

    pub fn req_qry_trading_account(
        &self,
        qry_trading_account: &CThostFtdcQryTradingAccountField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryTradingAccount(self.trader_ptr, qry_trading_account, request_id)
        })
    }

    pub fn req_qry_investor(
        &self,
        qry_investor: &CThostFtdcQryInvestorField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInvestor(self.trader_ptr, qry_investor, request_id)
        })
    }

    pub fn req_qry_trading_code(
        &self,
        qry_trading_code: &CThostFtdcQryTradingCodeField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryTradingCode(self.trader_ptr, qry_trading_code, request_id)
        })
    }

    pub fn req_qry_instrument_margin_rate(
        &self,
        qry_instrument_margin_rate: &CThostFtdcQryInstrumentMarginRateField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInstrumentMarginRate(self.trader_ptr, qry_instrument_margin_rate, request_id)
        })
    }

    pub fn req_qry_instrument_commission_rate(
        &self,
        qry_instrument_commission_rate: &CThostFtdcQryInstrumentCommissionRateField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInstrumentCommissionRate(
                self.trader_ptr,
                qry_instrument_commission_rate,
                request_id,
            )
        })
    }

    pub fn req_qry_user_session(
        &self,
        qry_user_session: &CThostFtdcQryUserSessionField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryUserSession(self.trader_ptr, qry_user_session, request_id)
        })
    }

    pub fn req_qry_exchange(
        &self,
        qry_exchange: &CThostFtdcQryExchangeField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryExchange(self.trader_ptr, qry_exchange, request_id)
        })
    }

    pub fn req_qry_product(
        &self,
        qry_product: &CThostFtdcQryProductField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryProduct(self.trader_ptr, qry_product, request_id)
        })
    }

    pub fn req_qry_instrument(
        &self,
        qry_instrument: &CThostFtdcQryInstrumentField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInstrument(self.trader_ptr, qry_instrument, request_id)
        })
    }

    pub fn req_qry_depth_market_data(
        &self,
        qry_depth_market_data: &CThostFtdcQryDepthMarketDataField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryDepthMarketData(self.trader_ptr, qry_depth_market_data, request_id)
        })
    }

    pub fn req_qry_trader_offer(
        &self,
        qry_trader_offer: &CThostFtdcQryTraderOfferField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryTraderOffer(self.trader_ptr, qry_trader_offer, request_id)
        })
    }

    pub fn req_qry_settlement_info(
        &self,
        qry_settlement_info: &CThostFtdcQrySettlementInfoField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySettlementInfo(self.trader_ptr, qry_settlement_info, request_id)
        })
    }

    pub fn req_qry_transfer_bank(
        &self,
        qry_transfer_bank: &CThostFtdcQryTransferBankField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryTransferBank(self.trader_ptr, qry_transfer_bank, request_id)
        })
    }

    pub fn req_qry_investor_position_detail(
        &self,
        qry_investor_position_detail: &CThostFtdcQryInvestorPositionDetailField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInvestorPositionDetail(
                self.trader_ptr,
                qry_investor_position_detail,
                request_id,
            )
        })
    }

    pub fn req_qry_notice(
        &self,
        qry_notice: &CThostFtdcQryNoticeField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryNotice(self.trader_ptr, qry_notice, request_id)
        })
    }

    pub fn req_qry_settlement_info_confirm(
        &self,
        qry_settlement_info_confirm: &CThostFtdcQrySettlementInfoConfirmField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySettlementInfoConfirm(self.trader_ptr, qry_settlement_info_confirm, request_id)
        })
    }

    pub fn req_qry_investor_position_combine_detail(
        &self,
        qry_investor_position_combine_detail: &CThostFtdcQryInvestorPositionCombineDetailField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInvestorPositionCombineDetail(
                self.trader_ptr,
                qry_investor_position_combine_detail,
                request_id,
            )
        })
    }

    pub fn req_qry_cfmmctrading_account_key(
        &self,
        qry_cfmmctrading_account_key: &CThostFtdcQryCFMMCTradingAccountKeyField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryCFMMCTradingAccountKey(
                self.trader_ptr,
                qry_cfmmctrading_account_key,
                request_id,
            )
        })
    }

    pub fn req_qry_ewarrant_offset(
        &self,
        qry_ewarrant_offset: &CThostFtdcQryEWarrantOffsetField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryEWarrantOffset(self.trader_ptr, qry_ewarrant_offset, request_id)
        })
    }

    pub fn req_qry_investor_product_group_margin(
        &self,
        qry_investor_product_group_margin: &CThostFtdcQryInvestorProductGroupMarginField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInvestorProductGroupMargin(
                self.trader_ptr,
                qry_investor_product_group_margin,
                request_id,
            )
        })
    }

    pub fn req_qry_exchange_margin_rate(
        &self,
        qry_exchange_margin_rate: &CThostFtdcQryExchangeMarginRateField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryExchangeMarginRate(self.trader_ptr, qry_exchange_margin_rate, request_id)
        })
    }

    pub fn req_qry_exchange_margin_rate_adjust(
        &self,
        qry_exchange_margin_rate_adjust: &CThostFtdcQryExchangeMarginRateAdjustField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryExchangeMarginRateAdjust(
                self.trader_ptr,
                qry_exchange_margin_rate_adjust,
                request_id,
            )
        })
    }

    pub fn req_qry_exchange_rate(
        &self,
        qry_exchange_rate: &CThostFtdcQryExchangeRateField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryExchangeRate(self.trader_ptr, qry_exchange_rate, request_id)
        })
    }

    pub fn req_qry_sec_agent_acidmap(
        &self,
        qry_sec_agent_acidmap: &CThostFtdcQrySecAgentACIDMapField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySecAgentACIDMap(self.trader_ptr, qry_sec_agent_acidmap, request_id)
        })
    }

    pub fn req_qry_product_exch_rate(
        &self,
        qry_product_exch_rate: &CThostFtdcQryProductExchRateField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryProductExchRate(self.trader_ptr, qry_product_exch_rate, request_id)
        })
    }

    pub fn req_qry_product_group(
        &self,
        qry_product_group: &CThostFtdcQryProductGroupField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryProductGroup(self.trader_ptr, qry_product_group, request_id)
        })
    }

    pub fn req_qry_mminstrument_commission_rate(
        &self,
        qry_mminstrument_commission_rate: &CThostFtdcQryMMInstrumentCommissionRateField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryMMInstrumentCommissionRate(
                self.trader_ptr,
                qry_mminstrument_commission_rate,
                request_id,
            )
        })
    }

    pub fn req_qry_mmoption_instr_comm_rate(
        &self,
        qry_mmoption_instr_comm_rate: &CThostFtdcQryMMOptionInstrCommRateField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryMMOptionInstrCommRate(self.trader_ptr, qry_mmoption_instr_comm_rate, request_id)
        })
    }

    pub fn req_qry_instrument_order_comm_rate(
        &self,
        qry_instrument_order_comm_rate: &CThostFtdcQryInstrumentOrderCommRateField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInstrumentOrderCommRate(
                self.trader_ptr,
                qry_instrument_order_comm_rate,
                request_id,
            )
        })
    }

    pub fn req_qry_sec_agent_trading_account(
        &self,
        qry_trading_account: &CThostFtdcQryTradingAccountField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySecAgentTradingAccount(self.trader_ptr, qry_trading_account, request_id)
        })
    }

    pub fn req_qry_sec_agent_check_mode(
        &self,
        qry_sec_agent_check_mode: &CThostFtdcQrySecAgentCheckModeField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySecAgentCheckMode(self.trader_ptr, qry_sec_agent_check_mode, request_id)
        })
    }

    pub fn req_qry_sec_agent_trade_info(
        &self,
        qry_sec_agent_trade_info: &CThostFtdcQrySecAgentTradeInfoField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySecAgentTradeInfo(self.trader_ptr, qry_sec_agent_trade_info, request_id)
        })
    }

    pub fn req_qry_option_instr_trade_cost(
        &self,
        qry_option_instr_trade_cost: &CThostFtdcQryOptionInstrTradeCostField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryOptionInstrTradeCost(self.trader_ptr, qry_option_instr_trade_cost, request_id)
        })
    }

    pub fn req_qry_option_instr_comm_rate(
        &self,
        qry_option_instr_comm_rate: &CThostFtdcQryOptionInstrCommRateField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryOptionInstrCommRate(self.trader_ptr, qry_option_instr_comm_rate, request_id)
        })
    }

    pub fn req_qry_exec_order(
        &self,
        qry_exec_order: &CThostFtdcQryExecOrderField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryExecOrder(self.trader_ptr, qry_exec_order, request_id)
        })
    }

    pub fn req_qry_for_quote(
        &self,
        qry_for_quote: &CThostFtdcQryForQuoteField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryForQuote(self.trader_ptr, qry_for_quote, request_id)
        })
    }

    pub fn req_qry_quote(&self, qry_quote: &CThostFtdcQryQuoteField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryQuote(self.trader_ptr, qry_quote, request_id)
        })
    }

    pub fn req_qry_option_self_close(
        &self,
        qry_option_self_close: &CThostFtdcQryOptionSelfCloseField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryOptionSelfClose(self.trader_ptr, qry_option_self_close, request_id)
        })
    }

    pub fn req_qry_invest_unit(
        &self,
        qry_invest_unit: &CThostFtdcQryInvestUnitField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInvestUnit(self.trader_ptr, qry_invest_unit, request_id)
        })
    }

    pub fn req_qry_comb_instrument_guard(
        &self,
        qry_comb_instrument_guard: &CThostFtdcQryCombInstrumentGuardField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryCombInstrumentGuard(self.trader_ptr, qry_comb_instrument_guard, request_id)
        })
    }

    pub fn req_qry_comb_action(
        &self,
        qry_comb_action: &CThostFtdcQryCombActionField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryCombAction(self.trader_ptr, qry_comb_action, request_id)
        })
    }

    pub fn req_qry_transfer_serial(
        &self,
        qry_transfer_serial: &CThostFtdcQryTransferSerialField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryTransferSerial(self.trader_ptr, qry_transfer_serial, request_id)
        })
    }

    pub fn req_qry_accountregister(
        &self,
        qry_accountregister: &CThostFtdcQryAccountregisterField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryAccountregister(self.trader_ptr, qry_accountregister, request_id)
        })
    }

    pub fn req_qry_contract_bank(
        &self,
        qry_contract_bank: &CThostFtdcQryContractBankField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryContractBank(self.trader_ptr, qry_contract_bank, request_id)
        })
    }

    pub fn req_qry_parked_order(
        &self,
        qry_parked_order: &CThostFtdcQryParkedOrderField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryParkedOrder(self.trader_ptr, qry_parked_order, request_id)
        })
    }

    pub fn req_qry_parked_order_action(
        &self,
        qry_parked_order_action: &CThostFtdcQryParkedOrderActionField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryParkedOrderAction(self.trader_ptr, qry_parked_order_action, request_id)
        })
    }

    pub fn req_qry_trading_notice(
        &self,
        qry_trading_notice: &CThostFtdcQryTradingNoticeField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryTradingNotice(self.trader_ptr, qry_trading_notice, request_id)
        })
    }

    pub fn req_qry_broker_trading_params(
        &self,
        qry_broker_trading_params: &CThostFtdcQryBrokerTradingParamsField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryBrokerTradingParams(self.trader_ptr, qry_broker_trading_params, request_id)
        })
    }

    pub fn req_qry_broker_trading_algos(
        &self,
        qry_broker_trading_algos: &CThostFtdcQryBrokerTradingAlgosField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryBrokerTradingAlgos(self.trader_ptr, qry_broker_trading_algos, request_id)
        })
    }

    pub fn req_query_cfmmctrading_account_token(
        &self,
        query_cfmmctrading_account_token: &CThostFtdcQueryCFMMCTradingAccountTokenField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQueryCFMMCTradingAccountToken(
                self.trader_ptr,
                query_cfmmctrading_account_token,
                request_id,
            )
        })
    }

    pub fn req_from_bank_to_future_by_future(
        &self,
        req_transfer: &CThostFtdcReqTransferField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqFromBankToFutureByFuture(self.trader_ptr, req_transfer, request_id)
        })
    }

    pub fn req_from_future_to_bank_by_future(
        &self,
        req_transfer: &CThostFtdcReqTransferField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqFromFutureToBankByFuture(self.trader_ptr, req_transfer, request_id)
        })
    }

    pub fn req_query_bank_account_money_by_future(
        &self,
        req_query_account: &CThostFtdcReqQueryAccountField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQueryBankAccountMoneyByFuture(self.trader_ptr, req_query_account, request_id)
        })
    }

    pub fn req_qry_classified_instrument(
        &self,
        qry_classified_instrument: &CThostFtdcQryClassifiedInstrumentField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryClassifiedInstrument(self.trader_ptr, qry_classified_instrument, request_id)
        })
    }

    pub fn req_qry_comb_promotion_param(
        &self,
        qry_comb_promotion_param: &CThostFtdcQryCombPromotionParamField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryCombPromotionParam(self.trader_ptr, qry_comb_promotion_param, request_id)
        })
    }

    pub fn req_qry_risk_settle_invst_position(
        &self,
        qry_risk_settle_invst_position: &CThostFtdcQryRiskSettleInvstPositionField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryRiskSettleInvstPosition(
                self.trader_ptr,
                qry_risk_settle_invst_position,
                request_id,
            )
        })
    }

    pub fn req_qry_risk_settle_product_status(
        &self,
        qry_risk_settle_product_status: &CThostFtdcQryRiskSettleProductStatusField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryRiskSettleProductStatus(
                self.trader_ptr,
                qry_risk_settle_product_status,
                request_id,
            )
        })
    }

    pub fn req_qry_spbmfuture_parameter(
        &self,
        qry_spbmfuture_parameter: &CThostFtdcQrySPBMFutureParameterField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySPBMFutureParameter(self.trader_ptr, qry_spbmfuture_parameter, request_id)
        })
    }

    pub fn req_qry_spbmoption_parameter(
        &self,
        qry_spbmoption_parameter: &CThostFtdcQrySPBMOptionParameterField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySPBMOptionParameter(self.trader_ptr, qry_spbmoption_parameter, request_id)
        })
    }

    pub fn req_qry_spbmintra_parameter(
        &self,
        qry_spbmintra_parameter: &CThostFtdcQrySPBMIntraParameterField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySPBMIntraParameter(self.trader_ptr, qry_spbmintra_parameter, request_id)
        })
    }

    pub fn req_qry_spbminter_parameter(
        &self,
        qry_spbminter_parameter: &CThostFtdcQrySPBMInterParameterField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySPBMInterParameter(self.trader_ptr, qry_spbminter_parameter, request_id)
        })
    }

    pub fn req_qry_spbmportf_definition(
        &self,
        qry_spbmportf_definition: &CThostFtdcQrySPBMPortfDefinitionField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySPBMPortfDefinition(self.trader_ptr, qry_spbmportf_definition, request_id)
        })
    }

    pub fn req_qry_spbminvestor_portf_def(
        &self,
        qry_spbminvestor_portf_def: &CThostFtdcQrySPBMInvestorPortfDefField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySPBMInvestorPortfDef(self.trader_ptr, qry_spbminvestor_portf_def, request_id)
        })
    }

    pub fn req_qry_investor_portf_margin_ratio(
        &self,
        qry_investor_portf_margin_ratio: &CThostFtdcQryInvestorPortfMarginRatioField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInvestorPortfMarginRatio(
                self.trader_ptr,
                qry_investor_portf_margin_ratio,
                request_id,
            )
        })
    }

    pub fn req_qry_investor_prod_spbmdetail(
        &self,
        qry_investor_prod_spbmdetail: &CThostFtdcQryInvestorProdSPBMDetailField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInvestorProdSPBMDetail(
                self.trader_ptr,
                qry_investor_prod_spbmdetail,
                request_id,
            )
        })
    }

    pub fn req_qry_investor_commodity_spmmmargin(
        &self,
        qry_investor_commodity_spmmmargin: &CThostFtdcQryInvestorCommoditySPMMMarginField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInvestorCommoditySPMMMargin(
                self.trader_ptr,
                qry_investor_commodity_spmmmargin,
                request_id,
            )
        })
    }

    pub fn req_qry_investor_commodity_group_spmmmargin(
        &self,
        qry_investor_commodity_group_spmmmargin:&CThostFtdcQryInvestorCommodityGroupSPMMMarginField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInvestorCommodityGroupSPMMMargin(
                self.trader_ptr,
                qry_investor_commodity_group_spmmmargin,
                request_id,
            )
        })
    }

    pub fn req_qry_spmminst_param(
        &self,
        qry_spmminst_param: &CThostFtdcQrySPMMInstParamField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySPMMInstParam(self.trader_ptr, qry_spmminst_param, request_id)
        })
    }

    pub fn req_qry_spmmproduct_param(
        &self,
        qry_spmmproduct_param: &CThostFtdcQrySPMMProductParamField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySPMMProductParam(self.trader_ptr, qry_spmmproduct_param, request_id)
        })
    }

    pub fn req_qry_spbmadd_on_inter_parameter(
        &self,
        qry_spbmadd_on_inter_parameter: &CThostFtdcQrySPBMAddOnInterParameterField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQrySPBMAddOnInterParameter(
                self.trader_ptr,
                qry_spbmadd_on_inter_parameter,
                request_id,
            )
        })
    }

    pub fn req_qry_rcamscomb_product_info(
        &self,
        qry_rcamscomb_product_info: &CThostFtdcQryRCAMSCombProductInfoField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryRCAMSCombProductInfo(self.trader_ptr, qry_rcamscomb_product_info, request_id)
        })
    }

    pub fn req_qry_rcamsinstr_parameter(
        &self,
        qry_rcamsinstr_parameter: &CThostFtdcQryRCAMSInstrParameterField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryRCAMSInstrParameter(self.trader_ptr, qry_rcamsinstr_parameter, request_id)
        })
    }

    pub fn req_qry_rcamsintra_parameter(
        &self,
        qry_rcamsintra_parameter: &CThostFtdcQryRCAMSIntraParameterField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryRCAMSIntraParameter(self.trader_ptr, qry_rcamsintra_parameter, request_id)
        })
    }

    pub fn req_qry_rcamsinter_parameter(
        &self,
        qry_rcamsinter_parameter: &CThostFtdcQryRCAMSInterParameterField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryRCAMSInterParameter(self.trader_ptr, qry_rcamsinter_parameter, request_id)
        })
    }

    pub fn req_qry_rcamsshort_opt_adjust_param(
        &self,
        qry_rcamsshort_opt_adjust_param: &CThostFtdcQryRCAMSShortOptAdjustParamField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryRCAMSShortOptAdjustParam(
                self.trader_ptr,
                qry_rcamsshort_opt_adjust_param,
                request_id,
            )
        })
    }

    pub fn req_qry_rcamsinvestor_comb_position(
        &self,
        qry_rcamsinvestor_comb_position: &CThostFtdcQryRCAMSInvestorCombPositionField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryRCAMSInvestorCombPosition(
                self.trader_ptr,
                qry_rcamsinvestor_comb_position,
                request_id,
            )
        })
    }

    pub fn req_qry_investor_prod_rcamsmargin(
        &self,
        qry_investor_prod_rcamsmargin: &CThostFtdcQryInvestorProdRCAMSMarginField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInvestorProdRCAMSMargin(
                self.trader_ptr,
                qry_investor_prod_rcamsmargin,
                request_id,
            )
        })
    }

    pub fn req_qry_ruleinstr_parameter(
        &self,
        qry_ruleinstr_parameter: &CThostFtdcQryRULEInstrParameterField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryRULEInstrParameter(self.trader_ptr, qry_ruleinstr_parameter, request_id)
        })
    }

    pub fn req_qry_ruleintra_parameter(
        &self,
        qry_ruleintra_parameter: &CThostFtdcQryRULEIntraParameterField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryRULEIntraParameter(self.trader_ptr, qry_ruleintra_parameter, request_id)
        })
    }

    pub fn req_qry_ruleinter_parameter(
        &self,
        qry_ruleinter_parameter: &CThostFtdcQryRULEInterParameterField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryRULEInterParameter(self.trader_ptr, qry_ruleinter_parameter, request_id)
        })
    }

    pub fn req_qry_investor_prod_rulemargin(
        &self,
        qry_investor_prod_rulemargin: &CThostFtdcQryInvestorProdRULEMarginField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInvestorProdRULEMargin(
                self.trader_ptr,
                qry_investor_prod_rulemargin,
                request_id,
            )
        })
    }

    pub fn req_qry_investor_portf_setting(
        &self,
        qry_investor_portf_setting: &CThostFtdcQryInvestorPortfSettingField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInvestorPortfSetting(self.trader_ptr, qry_investor_portf_setting, request_id)
        })
    }

    pub fn req_qry_investor_info_comm_rec(
        &self,
        qry_investor_info_comm_rec: &CThostFtdcQryInvestorInfoCommRecField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryInvestorInfoCommRec(self.trader_ptr, qry_investor_info_comm_rec, request_id)
        })
    }

    pub fn req_qry_comb_leg(
        &self,
        qry_comb_leg: &CThostFtdcQryCombLegField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryCombLeg(self.trader_ptr, qry_comb_leg, request_id)
        })
    }

    pub fn req_offset_setting(
        &self,
        input_offset_setting: &CThostFtdcInputOffsetSettingField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqOffsetSetting(self.trader_ptr, input_offset_setting, request_id)
        })
    }

    pub fn req_cancel_offset_setting(
        &self,
        input_offset_setting: &CThostFtdcInputOffsetSettingField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqCancelOffsetSetting(self.trader_ptr, input_offset_setting, request_id)
        })
    }

    pub fn req_qry_offset_setting(
        &self,
        qry_offset_setting: &CThostFtdcQryOffsetSettingField,
        request_id: i32,
    ) -> ApiResult {
        from_api_return_to_api_result(unsafe {
            TdReqQryOffsetSetting(self.trader_ptr, qry_offset_setting, request_id)
        })
    }
}
