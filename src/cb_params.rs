use crate::ctp::*;
use std::ffi::c_void;
// CTP回调参数，为避免跨线程传递反复拷贝，使用Box，仅拷贝一次

#[derive(Debug, Clone)]
pub enum OnErrRtnOptParam {
    OnErrRtnOrderInsert(Option<Box<CThostFtdcInputOrderField>>),
    OnErrRtnOrderAction(Option<Box<CThostFtdcOrderActionField>>),
    OnErrRtnExecOrderInsert(Option<Box<CThostFtdcInputExecOrderField>>),
    OnErrRtnExecOrderAction(Option<Box<CThostFtdcExecOrderActionField>>),
    OnErrRtnForQuoteInsert(Option<Box<CThostFtdcInputForQuoteField>>),
    OnErrRtnQuoteInsert(Option<Box<CThostFtdcInputQuoteField>>),
    OnErrRtnQuoteAction(Option<Box<CThostFtdcQuoteActionField>>),
    OnErrRtnBatchOrderAction(Option<Box<CThostFtdcBatchOrderActionField>>),
    OnErrRtnOptionSelfCloseInsert(Option<Box<CThostFtdcInputOptionSelfCloseField>>),
    OnErrRtnOptionSelfCloseAction(Option<Box<CThostFtdcOptionSelfCloseActionField>>),
    OnErrRtnCombActionInsert(Option<Box<CThostFtdcInputCombActionField>>),
    OnErrRtnBankToFutureByFuture(Option<Box<CThostFtdcReqTransferField>>),
    OnErrRtnFutureToBankByFuture(Option<Box<CThostFtdcReqTransferField>>),
    OnErrRtnRepealBankToFutureByFutureManual(Option<Box<CThostFtdcReqRepealField>>),
    OnErrRtnRepealFutureToBankByFutureManual(Option<Box<CThostFtdcReqRepealField>>),
    OnErrRtnQueryBankBalanceByFuture(Option<Box<CThostFtdcReqQueryAccountField>>),
}

#[derive(Debug, Clone)]
pub enum OnFrontOptParam {
    OnFrontConnected,
    OnFrontDisconnected(i32),
}

#[derive(Debug, Clone)]
pub enum OnRspOptParam {
    OnRspUserLogin(Option<Box<CThostFtdcRspUserLoginField>>),
    OnRspUserLogout(Option<Box<CThostFtdcUserLogoutField>>),
    OnRspQryMulticastInstrument(Option<Box<CThostFtdcMulticastInstrumentField>>),
    OnRspError(Option<Box<CThostFtdcRspInfoField>>),
    OnRspSubMarketData(Option<Box<CThostFtdcSpecificInstrumentField>>),
    OnRspUnSubMarketData(Option<Box<CThostFtdcSpecificInstrumentField>>),
    OnRspSubForQuoteRsp(Option<Box<CThostFtdcSpecificInstrumentField>>),
    OnRspUnSubForQuoteRsp(Option<Box<CThostFtdcSpecificInstrumentField>>),
    OnRspAuthenticate(Option<Box<CThostFtdcRspAuthenticateField>>),
    OnRspUserPasswordUpdate(Option<Box<CThostFtdcUserPasswordUpdateField>>),
    OnRspTradingAccountPasswordUpdate(Option<Box<CThostFtdcTradingAccountPasswordUpdateField>>),
    OnRspUserAuthMethod(Option<Box<CThostFtdcRspUserAuthMethodField>>),
    OnRspGenUserCaptcha(Option<Box<CThostFtdcRspGenUserCaptchaField>>),
    OnRspGenUserText(Option<Box<CThostFtdcRspGenUserTextField>>),
    OnRspOrderInsert(Option<Box<CThostFtdcInputOrderField>>),
    OnRspParkedOrderInsert(Option<Box<CThostFtdcParkedOrderField>>),
    OnRspParkedOrderAction(Option<Box<CThostFtdcParkedOrderActionField>>),
    OnRspOrderAction(Option<Box<CThostFtdcInputOrderActionField>>),
    OnRspQryMaxOrderVolume(Option<Box<CThostFtdcQryMaxOrderVolumeField>>),
    OnRspSettlementInfoConfirm(Option<Box<CThostFtdcSettlementInfoConfirmField>>),
    OnRspRemoveParkedOrder(Option<Box<CThostFtdcRemoveParkedOrderField>>),
    OnRspRemoveParkedOrderAction(Option<Box<CThostFtdcRemoveParkedOrderActionField>>),
    OnRspExecOrderInsert(Option<Box<CThostFtdcInputExecOrderField>>),
    OnRspExecOrderAction(Option<Box<CThostFtdcInputExecOrderActionField>>),
    OnRspForQuoteInsert(Option<Box<CThostFtdcInputForQuoteField>>),
    OnRspQuoteInsert(Option<Box<CThostFtdcInputQuoteField>>),
    OnRspQuoteAction(Option<Box<CThostFtdcInputQuoteActionField>>),
    OnRspBatchOrderAction(Option<Box<CThostFtdcInputBatchOrderActionField>>),
    OnRspOptionSelfCloseInsert(Option<Box<CThostFtdcInputOptionSelfCloseField>>),
    OnRspOptionSelfCloseAction(Option<Box<CThostFtdcInputOptionSelfCloseActionField>>),
    OnRspCombActionInsert(Option<Box<CThostFtdcInputCombActionField>>),
    OnRspQryOrder(Option<Box<CThostFtdcOrderField>>),
    OnRspQryTrade(Option<Box<CThostFtdcTradeField>>),
    OnRspQryInvestorPosition(Option<Box<CThostFtdcInvestorPositionField>>),
    OnRspQryTradingAccount(Option<Box<CThostFtdcTradingAccountField>>),
    OnRspQryInvestor(Option<Box<CThostFtdcInvestorField>>),
    OnRspQryTradingCode(Option<Box<CThostFtdcTradingCodeField>>),
    OnRspQryInstrumentMarginRate(Option<Box<CThostFtdcInstrumentMarginRateField>>),
    OnRspQryInstrumentCommissionRate(Option<Box<CThostFtdcInstrumentCommissionRateField>>),
    OnRspQryExchange(Option<Box<CThostFtdcExchangeField>>),
    OnRspQryProduct(Option<Box<CThostFtdcProductField>>),
    OnRspQryInstrument(Option<Box<CThostFtdcInstrumentField>>),
    OnRspQryDepthMarketData(Option<Box<CThostFtdcDepthMarketDataField>>),
    OnRspQrySettlementInfo(Option<Box<CThostFtdcSettlementInfoField>>),
    OnRspQryTransferBank(Option<Box<CThostFtdcTransferBankField>>),
    OnRspQryInvestorPositionDetail(Option<Box<CThostFtdcInvestorPositionDetailField>>),
    OnRspQryNotice(Option<Box<CThostFtdcNoticeField>>),
    OnRspQrySettlementInfoConfirm(Option<Box<CThostFtdcSettlementInfoConfirmField>>),
    OnRspQryInvestorPositionCombineDetail(
        Option<Box<CThostFtdcInvestorPositionCombineDetailField>>,
    ),
    OnRspQryCFMMCTradingAccountKey(Option<Box<CThostFtdcCFMMCTradingAccountKeyField>>),
    OnRspQryEWarrantOffset(Option<Box<CThostFtdcEWarrantOffsetField>>),
    OnRspQryInvestorProductGroupMargin(Option<Box<CThostFtdcInvestorProductGroupMarginField>>),
    OnRspQryExchangeMarginRate(Option<Box<CThostFtdcExchangeMarginRateField>>),
    OnRspQryExchangeMarginRateAdjust(Option<Box<CThostFtdcExchangeMarginRateAdjustField>>),
    OnRspQryExchangeRate(Option<Box<CThostFtdcExchangeRateField>>),
    OnRspQrySecAgentACIDMap(Option<Box<CThostFtdcSecAgentACIDMapField>>),
    OnRspQryProductExchRate(Option<Box<CThostFtdcProductExchRateField>>),
    OnRspQryProductGroup(Option<Box<CThostFtdcProductGroupField>>),
    OnRspQryMMInstrumentCommissionRate(Option<Box<CThostFtdcMMInstrumentCommissionRateField>>),
    OnRspQryMMOptionInstrCommRate(Option<Box<CThostFtdcMMOptionInstrCommRateField>>),
    OnRspQryInstrumentOrderCommRate(Option<Box<CThostFtdcInstrumentOrderCommRateField>>),
    OnRspQrySecAgentTradingAccount(Option<Box<CThostFtdcTradingAccountField>>),
    OnRspQrySecAgentCheckMode(Option<Box<CThostFtdcSecAgentCheckModeField>>),
    OnRspQrySecAgentTradeInfo(Option<Box<CThostFtdcSecAgentTradeInfoField>>),
    OnRspQryOptionInstrTradeCost(Option<Box<CThostFtdcOptionInstrTradeCostField>>),
    OnRspQryOptionInstrCommRate(Option<Box<CThostFtdcOptionInstrCommRateField>>),
    OnRspQryExecOrder(Option<Box<CThostFtdcExecOrderField>>),
    OnRspQryForQuote(Option<Box<CThostFtdcForQuoteField>>),
    OnRspQryQuote(Option<Box<CThostFtdcQuoteField>>),
    OnRspQryOptionSelfClose(Option<Box<CThostFtdcOptionSelfCloseField>>),
    OnRspQryInvestUnit(Option<Box<CThostFtdcInvestUnitField>>),
    OnRspQryCombInstrumentGuard(Option<Box<CThostFtdcCombInstrumentGuardField>>),
    OnRspQryCombAction(Option<Box<CThostFtdcCombActionField>>),
    OnRspQryTransferSerial(Option<Box<CThostFtdcTransferSerialField>>),
    OnRspQryAccountregister(Option<Box<CThostFtdcAccountregisterField>>),
    OnRspQryContractBank(Option<Box<CThostFtdcContractBankField>>),
    OnRspQryParkedOrder(Option<Box<CThostFtdcParkedOrderField>>),
    OnRspQryParkedOrderAction(Option<Box<CThostFtdcParkedOrderActionField>>),
    OnRspQryTradingNotice(Option<Box<CThostFtdcTradingNoticeField>>),
    OnRspQryBrokerTradingParams(Option<Box<CThostFtdcBrokerTradingParamsField>>),
    OnRspQryBrokerTradingAlgos(Option<Box<CThostFtdcBrokerTradingAlgosField>>),
    OnRspQueryCFMMCTradingAccountToken(Option<Box<CThostFtdcQueryCFMMCTradingAccountTokenField>>),
    OnRspFromBankToFutureByFuture(Option<Box<CThostFtdcReqTransferField>>),
    OnRspFromFutureToBankByFuture(Option<Box<CThostFtdcReqTransferField>>),
    OnRspQueryBankAccountMoneyByFuture(Option<Box<CThostFtdcReqQueryAccountField>>),
    OnRspQryClassifiedInstrument(Option<Box<CThostFtdcInstrumentField>>),
    OnRspQryCombPromotionParam(Option<Box<CThostFtdcCombPromotionParamField>>),
    OnRspQryTraderOffer(Option<Box<CThostFtdcTraderOfferField>>),
    OnRspQryRiskSettleInvstPosition(Option<Box<CThostFtdcRiskSettleInvstPositionField>>),
    OnRspQryRiskSettleProductStatus(Option<Box<CThostFtdcRiskSettleProductStatusField>>),
}

#[derive(Debug, Clone)]
pub enum OnRtnOptParam {
    OnRtnDepthMarketData(Option<Box<CThostFtdcDepthMarketDataField>>),
    OnRtnForQuoteRsp(Option<Box<CThostFtdcForQuoteRspField>>),
    OnRtnOrder(Option<Box<CThostFtdcOrderField>>),
    OnRtnTrade(Option<Box<CThostFtdcTradeField>>),
    OnRtnInstrumentStatus(Option<Box<CThostFtdcInstrumentStatusField>>),
    OnRtnBulletin(Option<Box<CThostFtdcBulletinField>>),
    OnRtnTradingNotice(Option<Box<CThostFtdcTradingNoticeInfoField>>),
    OnRtnErrorConditionalOrder(Option<Box<CThostFtdcErrorConditionalOrderField>>),
    OnRtnExecOrder(Option<Box<CThostFtdcExecOrderField>>),
    OnRtnQuote(Option<Box<CThostFtdcQuoteField>>),
    OnRtnCFMMCTradingAccountToken(Option<Box<CThostFtdcCFMMCTradingAccountTokenField>>),
    OnRtnOptionSelfClose(Option<Box<CThostFtdcOptionSelfCloseField>>),
    OnRtnCombAction(Option<Box<CThostFtdcCombActionField>>),
    OnRtnFromBankToFutureByBank(Option<Box<CThostFtdcRspTransferField>>),
    OnRtnFromFutureToBankByBank(Option<Box<CThostFtdcRspTransferField>>),
    OnRtnRepealFromBankToFutureByBank(Option<Box<CThostFtdcRspRepealField>>),
    OnRtnRepealFromFutureToBankByBank(Option<Box<CThostFtdcRspRepealField>>),
    OnRtnFromBankToFutureByFuture(Option<Box<CThostFtdcRspTransferField>>),
    OnRtnFromFutureToBankByFuture(Option<Box<CThostFtdcRspTransferField>>),
    OnRtnRepealFromBankToFutureByFutureManual(Option<Box<CThostFtdcRspRepealField>>),
    OnRtnRepealFromFutureToBankByFutureManual(Option<Box<CThostFtdcRspRepealField>>),
    OnRtnQueryBankBalanceByFuture(Option<Box<CThostFtdcNotifyQueryAccountField>>),
    OnRtnRepealFromBankToFutureByFuture(Option<Box<CThostFtdcRspRepealField>>),
    OnRtnRepealFromFutureToBankByFuture(Option<Box<CThostFtdcRspRepealField>>),
    OnRtnOpenAccountByBank(Option<Box<CThostFtdcOpenAccountField>>),
    OnRtnCancelAccountByBank(Option<Box<CThostFtdcCancelAccountField>>),
    OnRtnChangeAccountByBank(Option<Box<CThostFtdcChangeAccountField>>),
}

pub fn cvoid_to_errrtn_param(evt: EnumOnErrRtnEvent, param: *const c_void) -> OnErrRtnOptParam {
    match evt {
        EnumOnErrRtnEvent::OnErrRtnOrderInsert => {
            let fld = param as *const CThostFtdcInputOrderField;
            return OnErrRtnOptParam::OnErrRtnOrderInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnOrderAction => {
            let fld = param as *const CThostFtdcOrderActionField;
            return OnErrRtnOptParam::OnErrRtnOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnExecOrderInsert => {
            let fld = param as *const CThostFtdcInputExecOrderField;
            return OnErrRtnOptParam::OnErrRtnExecOrderInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnExecOrderAction => {
            let fld = param as *const CThostFtdcExecOrderActionField;
            return OnErrRtnOptParam::OnErrRtnExecOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnForQuoteInsert => {
            let fld = param as *const CThostFtdcInputForQuoteField;
            return OnErrRtnOptParam::OnErrRtnForQuoteInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnQuoteInsert => {
            let fld = param as *const CThostFtdcInputQuoteField;
            return OnErrRtnOptParam::OnErrRtnQuoteInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnQuoteAction => {
            let fld = param as *const CThostFtdcQuoteActionField;
            return OnErrRtnOptParam::OnErrRtnQuoteAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnBatchOrderAction => {
            let fld = param as *const CThostFtdcBatchOrderActionField;
            return OnErrRtnOptParam::OnErrRtnBatchOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnOptionSelfCloseInsert => {
            let fld = param as *const CThostFtdcInputOptionSelfCloseField;
            return OnErrRtnOptParam::OnErrRtnOptionSelfCloseInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnOptionSelfCloseAction => {
            let fld = param as *const CThostFtdcOptionSelfCloseActionField;
            return OnErrRtnOptParam::OnErrRtnOptionSelfCloseAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnCombActionInsert => {
            let fld = param as *const CThostFtdcInputCombActionField;
            return OnErrRtnOptParam::OnErrRtnCombActionInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnBankToFutureByFuture => {
            let fld = param as *const CThostFtdcReqTransferField;
            return OnErrRtnOptParam::OnErrRtnBankToFutureByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnFutureToBankByFuture => {
            let fld = param as *const CThostFtdcReqTransferField;
            return OnErrRtnOptParam::OnErrRtnFutureToBankByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnRepealBankToFutureByFutureManual => {
            let fld = param as *const CThostFtdcReqRepealField;
            return OnErrRtnOptParam::OnErrRtnRepealBankToFutureByFutureManual(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnRepealFutureToBankByFutureManual => {
            let fld = param as *const CThostFtdcReqRepealField;
            return OnErrRtnOptParam::OnErrRtnRepealFutureToBankByFutureManual(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnQueryBankBalanceByFuture => {
            let fld = param as *const CThostFtdcReqQueryAccountField;
            return OnErrRtnOptParam::OnErrRtnQueryBankBalanceByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
    }
}

pub fn cvoid_to_rsp_param(evt: EnumOnRspEvent, param: *const c_void) -> OnRspOptParam {
    match evt {
        EnumOnRspEvent::OnRspUserLogin => {
            let fld = param as *const CThostFtdcRspUserLoginField;
            return OnRspOptParam::OnRspUserLogin(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspUserLogout => {
            let fld = param as *const CThostFtdcUserLogoutField;
            return OnRspOptParam::OnRspUserLogout(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryMulticastInstrument => {
            let fld = param as *const CThostFtdcMulticastInstrumentField;
            return OnRspOptParam::OnRspQryMulticastInstrument(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspError => {
            let fld = param as *const CThostFtdcRspInfoField;
            return OnRspOptParam::OnRspError(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspSubMarketData => {
            let fld = param as *const CThostFtdcSpecificInstrumentField;
            return OnRspOptParam::OnRspSubMarketData(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspUnSubMarketData => {
            let fld = param as *const CThostFtdcSpecificInstrumentField;
            return OnRspOptParam::OnRspUnSubMarketData(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspSubForQuoteRsp => {
            let fld = param as *const CThostFtdcSpecificInstrumentField;
            return OnRspOptParam::OnRspSubForQuoteRsp(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspUnSubForQuoteRsp => {
            let fld = param as *const CThostFtdcSpecificInstrumentField;
            return OnRspOptParam::OnRspUnSubForQuoteRsp(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspAuthenticate => {
            let fld = param as *const CThostFtdcRspAuthenticateField;
            return OnRspOptParam::OnRspAuthenticate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspUserPasswordUpdate => {
            let fld = param as *const CThostFtdcUserPasswordUpdateField;
            return OnRspOptParam::OnRspUserPasswordUpdate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspTradingAccountPasswordUpdate => {
            let fld = param as *const CThostFtdcTradingAccountPasswordUpdateField;
            return OnRspOptParam::OnRspTradingAccountPasswordUpdate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspUserAuthMethod => {
            let fld = param as *const CThostFtdcRspUserAuthMethodField;
            return OnRspOptParam::OnRspUserAuthMethod(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspGenUserCaptcha => {
            let fld = param as *const CThostFtdcRspGenUserCaptchaField;
            return OnRspOptParam::OnRspGenUserCaptcha(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspGenUserText => {
            let fld = param as *const CThostFtdcRspGenUserTextField;
            return OnRspOptParam::OnRspGenUserText(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspOrderInsert => {
            let fld = param as *const CThostFtdcInputOrderField;
            return OnRspOptParam::OnRspOrderInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspParkedOrderInsert => {
            let fld = param as *const CThostFtdcParkedOrderField;
            return OnRspOptParam::OnRspParkedOrderInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspParkedOrderAction => {
            let fld = param as *const CThostFtdcParkedOrderActionField;
            return OnRspOptParam::OnRspParkedOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspOrderAction => {
            let fld = param as *const CThostFtdcInputOrderActionField;
            return OnRspOptParam::OnRspOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryMaxOrderVolume => {
            let fld = param as *const CThostFtdcQryMaxOrderVolumeField;
            return OnRspOptParam::OnRspQryMaxOrderVolume(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspSettlementInfoConfirm => {
            let fld = param as *const CThostFtdcSettlementInfoConfirmField;
            return OnRspOptParam::OnRspSettlementInfoConfirm(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspRemoveParkedOrder => {
            let fld = param as *const CThostFtdcRemoveParkedOrderField;
            return OnRspOptParam::OnRspRemoveParkedOrder(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspRemoveParkedOrderAction => {
            let fld = param as *const CThostFtdcRemoveParkedOrderActionField;
            return OnRspOptParam::OnRspRemoveParkedOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspExecOrderInsert => {
            let fld = param as *const CThostFtdcInputExecOrderField;
            return OnRspOptParam::OnRspExecOrderInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspExecOrderAction => {
            let fld = param as *const CThostFtdcInputExecOrderActionField;
            return OnRspOptParam::OnRspExecOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspForQuoteInsert => {
            let fld = param as *const CThostFtdcInputForQuoteField;
            return OnRspOptParam::OnRspForQuoteInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQuoteInsert => {
            let fld = param as *const CThostFtdcInputQuoteField;
            return OnRspOptParam::OnRspQuoteInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQuoteAction => {
            let fld = param as *const CThostFtdcInputQuoteActionField;
            return OnRspOptParam::OnRspQuoteAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspBatchOrderAction => {
            let fld = param as *const CThostFtdcInputBatchOrderActionField;
            return OnRspOptParam::OnRspBatchOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspOptionSelfCloseInsert => {
            let fld = param as *const CThostFtdcInputOptionSelfCloseField;
            return OnRspOptParam::OnRspOptionSelfCloseInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspOptionSelfCloseAction => {
            let fld = param as *const CThostFtdcInputOptionSelfCloseActionField;
            return OnRspOptParam::OnRspOptionSelfCloseAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspCombActionInsert => {
            let fld = param as *const CThostFtdcInputCombActionField;
            return OnRspOptParam::OnRspCombActionInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryOrder => {
            let fld = param as *const CThostFtdcOrderField;
            return OnRspOptParam::OnRspQryOrder(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTrade => {
            let fld = param as *const CThostFtdcTradeField;
            return OnRspOptParam::OnRspQryTrade(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPosition => {
            let fld = param as *const CThostFtdcInvestorPositionField;
            return OnRspOptParam::OnRspQryInvestorPosition(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTradingAccount => {
            let fld = param as *const CThostFtdcTradingAccountField;
            return OnRspOptParam::OnRspQryTradingAccount(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestor => {
            let fld = param as *const CThostFtdcInvestorField;
            return OnRspOptParam::OnRspQryInvestor(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTradingCode => {
            let fld = param as *const CThostFtdcTradingCodeField;
            return OnRspOptParam::OnRspQryTradingCode(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInstrumentMarginRate => {
            let fld = param as *const CThostFtdcInstrumentMarginRateField;
            return OnRspOptParam::OnRspQryInstrumentMarginRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInstrumentCommissionRate => {
            let fld = param as *const CThostFtdcInstrumentCommissionRateField;
            return OnRspOptParam::OnRspQryInstrumentCommissionRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryExchange => {
            let fld = param as *const CThostFtdcExchangeField;
            return OnRspOptParam::OnRspQryExchange(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryProduct => {
            let fld = param as *const CThostFtdcProductField;
            return OnRspOptParam::OnRspQryProduct(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInstrument => {
            let fld = param as *const CThostFtdcInstrumentField;
            return OnRspOptParam::OnRspQryInstrument(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryDepthMarketData => {
            let fld = param as *const CThostFtdcDepthMarketDataField;
            return OnRspOptParam::OnRspQryDepthMarketData(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySettlementInfo => {
            let fld = param as *const CThostFtdcSettlementInfoField;
            return OnRspOptParam::OnRspQrySettlementInfo(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTransferBank => {
            let fld = param as *const CThostFtdcTransferBankField;
            return OnRspOptParam::OnRspQryTransferBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPositionDetail => {
            let fld = param as *const CThostFtdcInvestorPositionDetailField;
            return OnRspOptParam::OnRspQryInvestorPositionDetail(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryNotice => {
            let fld = param as *const CThostFtdcNoticeField;
            return OnRspOptParam::OnRspQryNotice(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySettlementInfoConfirm => {
            let fld = param as *const CThostFtdcSettlementInfoConfirmField;
            return OnRspOptParam::OnRspQrySettlementInfoConfirm(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPositionCombineDetail => {
            let fld = param as *const CThostFtdcInvestorPositionCombineDetailField;
            return OnRspOptParam::OnRspQryInvestorPositionCombineDetail(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryCFMMCTradingAccountKey => {
            let fld = param as *const CThostFtdcCFMMCTradingAccountKeyField;
            return OnRspOptParam::OnRspQryCFMMCTradingAccountKey(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryEWarrantOffset => {
            let fld = param as *const CThostFtdcEWarrantOffsetField;
            return OnRspOptParam::OnRspQryEWarrantOffset(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorProductGroupMargin => {
            let fld = param as *const CThostFtdcInvestorProductGroupMarginField;
            return OnRspOptParam::OnRspQryInvestorProductGroupMargin(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryExchangeMarginRate => {
            let fld = param as *const CThostFtdcExchangeMarginRateField;
            return OnRspOptParam::OnRspQryExchangeMarginRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryExchangeMarginRateAdjust => {
            let fld = param as *const CThostFtdcExchangeMarginRateAdjustField;
            return OnRspOptParam::OnRspQryExchangeMarginRateAdjust(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryExchangeRate => {
            let fld = param as *const CThostFtdcExchangeRateField;
            return OnRspOptParam::OnRspQryExchangeRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySecAgentACIDMap => {
            let fld = param as *const CThostFtdcSecAgentACIDMapField;
            return OnRspOptParam::OnRspQrySecAgentACIDMap(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryProductExchRate => {
            let fld = param as *const CThostFtdcProductExchRateField;
            return OnRspOptParam::OnRspQryProductExchRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryProductGroup => {
            let fld = param as *const CThostFtdcProductGroupField;
            return OnRspOptParam::OnRspQryProductGroup(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryMMInstrumentCommissionRate => {
            let fld = param as *const CThostFtdcMMInstrumentCommissionRateField;
            return OnRspOptParam::OnRspQryMMInstrumentCommissionRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryMMOptionInstrCommRate => {
            let fld = param as *const CThostFtdcMMOptionInstrCommRateField;
            return OnRspOptParam::OnRspQryMMOptionInstrCommRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInstrumentOrderCommRate => {
            let fld = param as *const CThostFtdcInstrumentOrderCommRateField;
            return OnRspOptParam::OnRspQryInstrumentOrderCommRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySecAgentTradingAccount => {
            let fld = param as *const CThostFtdcTradingAccountField;
            return OnRspOptParam::OnRspQrySecAgentTradingAccount(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySecAgentCheckMode => {
            let fld = param as *const CThostFtdcSecAgentCheckModeField;
            return OnRspOptParam::OnRspQrySecAgentCheckMode(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySecAgentTradeInfo => {
            let fld = param as *const CThostFtdcSecAgentTradeInfoField;
            return OnRspOptParam::OnRspQrySecAgentTradeInfo(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryOptionInstrTradeCost => {
            let fld = param as *const CThostFtdcOptionInstrTradeCostField;
            return OnRspOptParam::OnRspQryOptionInstrTradeCost(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryOptionInstrCommRate => {
            let fld = param as *const CThostFtdcOptionInstrCommRateField;
            return OnRspOptParam::OnRspQryOptionInstrCommRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryExecOrder => {
            let fld = param as *const CThostFtdcExecOrderField;
            return OnRspOptParam::OnRspQryExecOrder(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryForQuote => {
            let fld = param as *const CThostFtdcForQuoteField;
            return OnRspOptParam::OnRspQryForQuote(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryQuote => {
            let fld = param as *const CThostFtdcQuoteField;
            return OnRspOptParam::OnRspQryQuote(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryOptionSelfClose => {
            let fld = param as *const CThostFtdcOptionSelfCloseField;
            return OnRspOptParam::OnRspQryOptionSelfClose(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestUnit => {
            let fld = param as *const CThostFtdcInvestUnitField;
            return OnRspOptParam::OnRspQryInvestUnit(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryCombInstrumentGuard => {
            let fld = param as *const CThostFtdcCombInstrumentGuardField;
            return OnRspOptParam::OnRspQryCombInstrumentGuard(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryCombAction => {
            let fld = param as *const CThostFtdcCombActionField;
            return OnRspOptParam::OnRspQryCombAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTransferSerial => {
            let fld = param as *const CThostFtdcTransferSerialField;
            return OnRspOptParam::OnRspQryTransferSerial(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryAccountregister => {
            let fld = param as *const CThostFtdcAccountregisterField;
            return OnRspOptParam::OnRspQryAccountregister(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryContractBank => {
            let fld = param as *const CThostFtdcContractBankField;
            return OnRspOptParam::OnRspQryContractBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryParkedOrder => {
            let fld = param as *const CThostFtdcParkedOrderField;
            return OnRspOptParam::OnRspQryParkedOrder(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryParkedOrderAction => {
            let fld = param as *const CThostFtdcParkedOrderActionField;
            return OnRspOptParam::OnRspQryParkedOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTradingNotice => {
            let fld = param as *const CThostFtdcTradingNoticeField;
            return OnRspOptParam::OnRspQryTradingNotice(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryBrokerTradingParams => {
            let fld = param as *const CThostFtdcBrokerTradingParamsField;
            return OnRspOptParam::OnRspQryBrokerTradingParams(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryBrokerTradingAlgos => {
            let fld = param as *const CThostFtdcBrokerTradingAlgosField;
            return OnRspOptParam::OnRspQryBrokerTradingAlgos(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQueryCFMMCTradingAccountToken => {
            let fld = param as *const CThostFtdcQueryCFMMCTradingAccountTokenField;
            return OnRspOptParam::OnRspQueryCFMMCTradingAccountToken(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspFromBankToFutureByFuture => {
            let fld = param as *const CThostFtdcReqTransferField;
            return OnRspOptParam::OnRspFromBankToFutureByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspFromFutureToBankByFuture => {
            let fld = param as *const CThostFtdcReqTransferField;
            return OnRspOptParam::OnRspFromFutureToBankByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQueryBankAccountMoneyByFuture => {
            let fld = param as *const CThostFtdcReqQueryAccountField;
            return OnRspOptParam::OnRspQueryBankAccountMoneyByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryClassifiedInstrument => {
            let fld = param as *const CThostFtdcInstrumentField;
            return OnRspOptParam::OnRspQryClassifiedInstrument(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryCombPromotionParam => {
            let fld = param as *const CThostFtdcCombPromotionParamField;
            return OnRspOptParam::OnRspQryCombPromotionParam(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTraderOffer => {
            let fld = param as *const CThostFtdcTraderOfferField;
            return OnRspOptParam::OnRspQryTraderOffer(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRiskSettleInvstPosition => {
            let fld = param as *const CThostFtdcRiskSettleInvstPositionField;
            return OnRspOptParam::OnRspQryRiskSettleInvstPosition(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRiskSettleProductStatus => {
            let fld = param as *const CThostFtdcRiskSettleProductStatusField;
            return OnRspOptParam::OnRspQryRiskSettleProductStatus(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
    }
}

pub fn cvoid_to_rtn_param(evt: EnumOnRtnEvent, param: *const c_void) -> OnRtnOptParam {
    match evt {
        EnumOnRtnEvent::OnRtnDepthMarketData => {
            let fld = param as *const CThostFtdcDepthMarketDataField;
            return OnRtnOptParam::OnRtnDepthMarketData(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnForQuoteRsp => {
            let fld = param as *const CThostFtdcForQuoteRspField;
            return OnRtnOptParam::OnRtnForQuoteRsp(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnOrder => {
            let fld = param as *const CThostFtdcOrderField;
            return OnRtnOptParam::OnRtnOrder(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnTrade => {
            let fld = param as *const CThostFtdcTradeField;
            return OnRtnOptParam::OnRtnTrade(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnInstrumentStatus => {
            let fld = param as *const CThostFtdcInstrumentStatusField;
            return OnRtnOptParam::OnRtnInstrumentStatus(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnBulletin => {
            let fld = param as *const CThostFtdcBulletinField;
            return OnRtnOptParam::OnRtnBulletin(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnTradingNotice => {
            let fld = param as *const CThostFtdcTradingNoticeInfoField;
            return OnRtnOptParam::OnRtnTradingNotice(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnErrorConditionalOrder => {
            let fld = param as *const CThostFtdcErrorConditionalOrderField;
            return OnRtnOptParam::OnRtnErrorConditionalOrder(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnExecOrder => {
            let fld = param as *const CThostFtdcExecOrderField;
            return OnRtnOptParam::OnRtnExecOrder(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnQuote => {
            let fld = param as *const CThostFtdcQuoteField;
            return OnRtnOptParam::OnRtnQuote(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnCFMMCTradingAccountToken => {
            let fld = param as *const CThostFtdcCFMMCTradingAccountTokenField;
            return OnRtnOptParam::OnRtnCFMMCTradingAccountToken(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnOptionSelfClose => {
            let fld = param as *const CThostFtdcOptionSelfCloseField;
            return OnRtnOptParam::OnRtnOptionSelfClose(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnCombAction => {
            let fld = param as *const CThostFtdcCombActionField;
            return OnRtnOptParam::OnRtnCombAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnFromBankToFutureByBank => {
            let fld = param as *const CThostFtdcRspTransferField;
            return OnRtnOptParam::OnRtnFromBankToFutureByBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnFromFutureToBankByBank => {
            let fld = param as *const CThostFtdcRspTransferField;
            return OnRtnOptParam::OnRtnFromFutureToBankByBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromBankToFutureByBank => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptParam::OnRtnRepealFromBankToFutureByBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromFutureToBankByBank => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptParam::OnRtnRepealFromFutureToBankByBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnFromBankToFutureByFuture => {
            let fld = param as *const CThostFtdcRspTransferField;
            return OnRtnOptParam::OnRtnFromBankToFutureByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnFromFutureToBankByFuture => {
            let fld = param as *const CThostFtdcRspTransferField;
            return OnRtnOptParam::OnRtnFromFutureToBankByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromBankToFutureByFutureManual => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptParam::OnRtnRepealFromBankToFutureByFutureManual(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromFutureToBankByFutureManual => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptParam::OnRtnRepealFromFutureToBankByFutureManual(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnQueryBankBalanceByFuture => {
            let fld = param as *const CThostFtdcNotifyQueryAccountField;
            return OnRtnOptParam::OnRtnQueryBankBalanceByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromBankToFutureByFuture => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptParam::OnRtnRepealFromBankToFutureByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromFutureToBankByFuture => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptParam::OnRtnRepealFromFutureToBankByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnOpenAccountByBank => {
            let fld = param as *const CThostFtdcOpenAccountField;
            return OnRtnOptParam::OnRtnOpenAccountByBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnCancelAccountByBank => {
            let fld = param as *const CThostFtdcCancelAccountField;
            return OnRtnOptParam::OnRtnCancelAccountByBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnChangeAccountByBank => {
            let fld = param as *const CThostFtdcChangeAccountField;
            return OnRtnOptParam::OnRtnChangeAccountByBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
    }
}
