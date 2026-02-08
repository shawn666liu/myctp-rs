use crate::ctp::*;
use std::ffi::c_void;
use std::sync::Arc;

// CTP回调参数，为避免跨线程传递反复拷贝，使用Arc，仅拷贝一次

#[derive(Debug, Clone)]
pub enum OnErrRtnOptArc {
    OnErrRtnOrderInsert(Option<Arc<CThostFtdcInputOrderField>>),
    OnErrRtnOrderAction(Option<Arc<CThostFtdcOrderActionField>>),
    OnErrRtnExecOrderInsert(Option<Arc<CThostFtdcInputExecOrderField>>),
    OnErrRtnExecOrderAction(Option<Arc<CThostFtdcExecOrderActionField>>),
    OnErrRtnForQuoteInsert(Option<Arc<CThostFtdcInputForQuoteField>>),
    OnErrRtnQuoteInsert(Option<Arc<CThostFtdcInputQuoteField>>),
    OnErrRtnQuoteAction(Option<Arc<CThostFtdcQuoteActionField>>),
    OnErrRtnBatchOrderAction(Option<Arc<CThostFtdcBatchOrderActionField>>),
    OnErrRtnOptionSelfCloseInsert(Option<Arc<CThostFtdcInputOptionSelfCloseField>>),
    OnErrRtnOptionSelfCloseAction(Option<Arc<CThostFtdcOptionSelfCloseActionField>>),
    OnErrRtnCombActionInsert(Option<Arc<CThostFtdcInputCombActionField>>),
    OnErrRtnBankToFutureByFuture(Option<Arc<CThostFtdcReqTransferField>>),
    OnErrRtnFutureToBankByFuture(Option<Arc<CThostFtdcReqTransferField>>),
    OnErrRtnRepealBankToFutureByFutureManual(Option<Arc<CThostFtdcReqRepealField>>),
    OnErrRtnRepealFutureToBankByFutureManual(Option<Arc<CThostFtdcReqRepealField>>),
    OnErrRtnQueryBankBalanceByFuture(Option<Arc<CThostFtdcReqQueryAccountField>>),
    OnErrRtnOffsetSetting(Option<Arc<CThostFtdcInputOffsetSettingField>>),
    OnErrRtnCancelOffsetSetting(Option<Arc<CThostFtdcCancelOffsetSettingField>>),
}

#[derive(Debug, Clone)]
pub enum OnRspOptArc {
    OnRspUserLogin(Option<Arc<CThostFtdcRspUserLoginField>>),
    OnRspUserLogout(Option<Arc<CThostFtdcUserLogoutField>>),
    OnRspQryMulticastInstrument(Option<Arc<CThostFtdcMulticastInstrumentField>>),
    OnRspError(Option<Arc<CThostFtdcRspInfoField>>),
    OnRspSubMarketData(Option<Arc<CThostFtdcSpecificInstrumentField>>),
    OnRspUnSubMarketData(Option<Arc<CThostFtdcSpecificInstrumentField>>),
    OnRspSubForQuoteRsp(Option<Arc<CThostFtdcSpecificInstrumentField>>),
    OnRspUnSubForQuoteRsp(Option<Arc<CThostFtdcSpecificInstrumentField>>),
    OnRspAuthenticate(Option<Arc<CThostFtdcRspAuthenticateField>>),
    OnRspUserPasswordUpdate(Option<Arc<CThostFtdcUserPasswordUpdateField>>),
    OnRspTradingAccountPasswordUpdate(Option<Arc<CThostFtdcTradingAccountPasswordUpdateField>>),
    OnRspUserAuthMethod(Option<Arc<CThostFtdcRspUserAuthMethodField>>),
    OnRspGenUserCaptcha(Option<Arc<CThostFtdcRspGenUserCaptchaField>>),
    OnRspGenUserText(Option<Arc<CThostFtdcRspGenUserTextField>>),
    OnRspOrderInsert(Option<Arc<CThostFtdcInputOrderField>>),
    OnRspParkedOrderInsert(Option<Arc<CThostFtdcParkedOrderField>>),
    OnRspParkedOrderAction(Option<Arc<CThostFtdcParkedOrderActionField>>),
    OnRspOrderAction(Option<Arc<CThostFtdcInputOrderActionField>>),
    OnRspQryMaxOrderVolume(Option<Arc<CThostFtdcQryMaxOrderVolumeField>>),
    OnRspSettlementInfoConfirm(Option<Arc<CThostFtdcSettlementInfoConfirmField>>),
    OnRspRemoveParkedOrder(Option<Arc<CThostFtdcRemoveParkedOrderField>>),
    OnRspRemoveParkedOrderAction(Option<Arc<CThostFtdcRemoveParkedOrderActionField>>),
    OnRspExecOrderInsert(Option<Arc<CThostFtdcInputExecOrderField>>),
    OnRspExecOrderAction(Option<Arc<CThostFtdcInputExecOrderActionField>>),
    OnRspForQuoteInsert(Option<Arc<CThostFtdcInputForQuoteField>>),
    OnRspQuoteInsert(Option<Arc<CThostFtdcInputQuoteField>>),
    OnRspQuoteAction(Option<Arc<CThostFtdcInputQuoteActionField>>),
    OnRspBatchOrderAction(Option<Arc<CThostFtdcInputBatchOrderActionField>>),
    OnRspOptionSelfCloseInsert(Option<Arc<CThostFtdcInputOptionSelfCloseField>>),
    OnRspOptionSelfCloseAction(Option<Arc<CThostFtdcInputOptionSelfCloseActionField>>),
    OnRspCombActionInsert(Option<Arc<CThostFtdcInputCombActionField>>),
    OnRspQryOrder(Option<Arc<CThostFtdcOrderField>>),
    OnRspQryTrade(Option<Arc<CThostFtdcTradeField>>),
    OnRspQryInvestorPosition(Option<Arc<CThostFtdcInvestorPositionField>>),
    OnRspQryTradingAccount(Option<Arc<CThostFtdcTradingAccountField>>),
    OnRspQryInvestor(Option<Arc<CThostFtdcInvestorField>>),
    OnRspQryTradingCode(Option<Arc<CThostFtdcTradingCodeField>>),
    OnRspQryInstrumentMarginRate(Option<Arc<CThostFtdcInstrumentMarginRateField>>),
    OnRspQryInstrumentCommissionRate(Option<Arc<CThostFtdcInstrumentCommissionRateField>>),
    OnRspQryUserSession(Option<Arc<CThostFtdcUserSessionField>>),
    OnRspQryExchange(Option<Arc<CThostFtdcExchangeField>>),
    OnRspQryProduct(Option<Arc<CThostFtdcProductField>>),
    OnRspQryInstrument(Option<Arc<CThostFtdcInstrumentField>>),
    OnRspQryDepthMarketData(Option<Arc<CThostFtdcDepthMarketDataField>>),
    OnRspQryTraderOffer(Option<Arc<CThostFtdcTraderOfferField>>),
    OnRspQrySettlementInfo(Option<Arc<CThostFtdcSettlementInfoField>>),
    OnRspQryTransferBank(Option<Arc<CThostFtdcTransferBankField>>),
    OnRspQryInvestorPositionDetail(Option<Arc<CThostFtdcInvestorPositionDetailField>>),
    OnRspQryNotice(Option<Arc<CThostFtdcNoticeField>>),
    OnRspQrySettlementInfoConfirm(Option<Arc<CThostFtdcSettlementInfoConfirmField>>),
    OnRspQryInvestorPositionCombineDetail(
        Option<Arc<CThostFtdcInvestorPositionCombineDetailField>>,
    ),
    OnRspQryCFMMCTradingAccountKey(Option<Arc<CThostFtdcCFMMCTradingAccountKeyField>>),
    OnRspQryEWarrantOffset(Option<Arc<CThostFtdcEWarrantOffsetField>>),
    OnRspQryInvestorProductGroupMargin(Option<Arc<CThostFtdcInvestorProductGroupMarginField>>),
    OnRspQryExchangeMarginRate(Option<Arc<CThostFtdcExchangeMarginRateField>>),
    OnRspQryExchangeMarginRateAdjust(Option<Arc<CThostFtdcExchangeMarginRateAdjustField>>),
    OnRspQryExchangeRate(Option<Arc<CThostFtdcExchangeRateField>>),
    OnRspQrySecAgentACIDMap(Option<Arc<CThostFtdcSecAgentACIDMapField>>),
    OnRspQryProductExchRate(Option<Arc<CThostFtdcProductExchRateField>>),
    OnRspQryProductGroup(Option<Arc<CThostFtdcProductGroupField>>),
    OnRspQryMMInstrumentCommissionRate(Option<Arc<CThostFtdcMMInstrumentCommissionRateField>>),
    OnRspQryMMOptionInstrCommRate(Option<Arc<CThostFtdcMMOptionInstrCommRateField>>),
    OnRspQryInstrumentOrderCommRate(Option<Arc<CThostFtdcInstrumentOrderCommRateField>>),
    OnRspQrySecAgentTradingAccount(Option<Arc<CThostFtdcTradingAccountField>>),
    OnRspQrySecAgentCheckMode(Option<Arc<CThostFtdcSecAgentCheckModeField>>),
    OnRspQrySecAgentTradeInfo(Option<Arc<CThostFtdcSecAgentTradeInfoField>>),
    OnRspQryOptionInstrTradeCost(Option<Arc<CThostFtdcOptionInstrTradeCostField>>),
    OnRspQryOptionInstrCommRate(Option<Arc<CThostFtdcOptionInstrCommRateField>>),
    OnRspQryExecOrder(Option<Arc<CThostFtdcExecOrderField>>),
    OnRspQryForQuote(Option<Arc<CThostFtdcForQuoteField>>),
    OnRspQryQuote(Option<Arc<CThostFtdcQuoteField>>),
    OnRspQryOptionSelfClose(Option<Arc<CThostFtdcOptionSelfCloseField>>),
    OnRspQryInvestUnit(Option<Arc<CThostFtdcInvestUnitField>>),
    OnRspQryCombInstrumentGuard(Option<Arc<CThostFtdcCombInstrumentGuardField>>),
    OnRspQryCombAction(Option<Arc<CThostFtdcCombActionField>>),
    OnRspQryTransferSerial(Option<Arc<CThostFtdcTransferSerialField>>),
    OnRspQryAccountregister(Option<Arc<CThostFtdcAccountregisterField>>),
    OnRspQryContractBank(Option<Arc<CThostFtdcContractBankField>>),
    OnRspQryParkedOrder(Option<Arc<CThostFtdcParkedOrderField>>),
    OnRspQryParkedOrderAction(Option<Arc<CThostFtdcParkedOrderActionField>>),
    OnRspQryTradingNotice(Option<Arc<CThostFtdcTradingNoticeField>>),
    OnRspQryBrokerTradingParams(Option<Arc<CThostFtdcBrokerTradingParamsField>>),
    OnRspQryBrokerTradingAlgos(Option<Arc<CThostFtdcBrokerTradingAlgosField>>),
    OnRspQueryCFMMCTradingAccountToken(Option<Arc<CThostFtdcQueryCFMMCTradingAccountTokenField>>),
    OnRspFromBankToFutureByFuture(Option<Arc<CThostFtdcReqTransferField>>),
    OnRspFromFutureToBankByFuture(Option<Arc<CThostFtdcReqTransferField>>),
    OnRspQueryBankAccountMoneyByFuture(Option<Arc<CThostFtdcReqQueryAccountField>>),
    OnRspQryClassifiedInstrument(Option<Arc<CThostFtdcInstrumentField>>),
    OnRspQryCombPromotionParam(Option<Arc<CThostFtdcCombPromotionParamField>>),
    OnRspQryRiskSettleInvstPosition(Option<Arc<CThostFtdcRiskSettleInvstPositionField>>),
    OnRspQryRiskSettleProductStatus(Option<Arc<CThostFtdcRiskSettleProductStatusField>>),
    OnRspQrySPBMFutureParameter(Option<Arc<CThostFtdcSPBMFutureParameterField>>),
    OnRspQrySPBMOptionParameter(Option<Arc<CThostFtdcSPBMOptionParameterField>>),
    OnRspQrySPBMIntraParameter(Option<Arc<CThostFtdcSPBMIntraParameterField>>),
    OnRspQrySPBMInterParameter(Option<Arc<CThostFtdcSPBMInterParameterField>>),
    OnRspQrySPBMPortfDefinition(Option<Arc<CThostFtdcSPBMPortfDefinitionField>>),
    OnRspQrySPBMInvestorPortfDef(Option<Arc<CThostFtdcSPBMInvestorPortfDefField>>),
    OnRspQryInvestorPortfMarginRatio(Option<Arc<CThostFtdcInvestorPortfMarginRatioField>>),
    OnRspQryInvestorProdSPBMDetail(Option<Arc<CThostFtdcInvestorProdSPBMDetailField>>),
    OnRspQryInvestorCommoditySPMMMargin(Option<Arc<CThostFtdcInvestorCommoditySPMMMarginField>>),
    OnRspQryInvestorCommodityGroupSPMMMargin(
        Option<Arc<CThostFtdcInvestorCommodityGroupSPMMMarginField>>,
    ),
    OnRspQrySPMMInstParam(Option<Arc<CThostFtdcSPMMInstParamField>>),
    OnRspQrySPMMProductParam(Option<Arc<CThostFtdcSPMMProductParamField>>),
    OnRspQrySPBMAddOnInterParameter(Option<Arc<CThostFtdcSPBMAddOnInterParameterField>>),
    OnRspQryRCAMSCombProductInfo(Option<Arc<CThostFtdcRCAMSCombProductInfoField>>),
    OnRspQryRCAMSInstrParameter(Option<Arc<CThostFtdcRCAMSInstrParameterField>>),
    OnRspQryRCAMSIntraParameter(Option<Arc<CThostFtdcRCAMSIntraParameterField>>),
    OnRspQryRCAMSInterParameter(Option<Arc<CThostFtdcRCAMSInterParameterField>>),
    OnRspQryRCAMSShortOptAdjustParam(Option<Arc<CThostFtdcRCAMSShortOptAdjustParamField>>),
    OnRspQryRCAMSInvestorCombPosition(Option<Arc<CThostFtdcRCAMSInvestorCombPositionField>>),
    OnRspQryInvestorProdRCAMSMargin(Option<Arc<CThostFtdcInvestorProdRCAMSMarginField>>),
    OnRspQryRULEInstrParameter(Option<Arc<CThostFtdcRULEInstrParameterField>>),
    OnRspQryRULEIntraParameter(Option<Arc<CThostFtdcRULEIntraParameterField>>),
    OnRspQryRULEInterParameter(Option<Arc<CThostFtdcRULEInterParameterField>>),
    OnRspQryInvestorProdRULEMargin(Option<Arc<CThostFtdcInvestorProdRULEMarginField>>),
    OnRspQryInvestorPortfSetting(Option<Arc<CThostFtdcInvestorPortfSettingField>>),
    OnRspQryInvestorInfoCommRec(Option<Arc<CThostFtdcInvestorInfoCommRecField>>),
    OnRspQryCombLeg(Option<Arc<CThostFtdcCombLegField>>),
    OnRspOffsetSetting(Option<Arc<CThostFtdcInputOffsetSettingField>>),
    OnRspCancelOffsetSetting(Option<Arc<CThostFtdcInputOffsetSettingField>>),
    OnRspQryOffsetSetting(Option<Arc<CThostFtdcOffsetSettingField>>),
}

#[derive(Debug, Clone)]
pub enum OnRtnOptArc {
    OnRtnDepthMarketData(Option<Arc<CThostFtdcDepthMarketDataField>>),
    OnRtnForQuoteRsp(Option<Arc<CThostFtdcForQuoteRspField>>),
    OnRtnOrder(Option<Arc<CThostFtdcOrderField>>),
    OnRtnTrade(Option<Arc<CThostFtdcTradeField>>),
    OnRtnInstrumentStatus(Option<Arc<CThostFtdcInstrumentStatusField>>),
    OnRtnBulletin(Option<Arc<CThostFtdcBulletinField>>),
    OnRtnTradingNotice(Option<Arc<CThostFtdcTradingNoticeInfoField>>),
    OnRtnErrorConditionalOrder(Option<Arc<CThostFtdcErrorConditionalOrderField>>),
    OnRtnExecOrder(Option<Arc<CThostFtdcExecOrderField>>),
    OnRtnQuote(Option<Arc<CThostFtdcQuoteField>>),
    OnRtnCFMMCTradingAccountToken(Option<Arc<CThostFtdcCFMMCTradingAccountTokenField>>),
    OnRtnOptionSelfClose(Option<Arc<CThostFtdcOptionSelfCloseField>>),
    OnRtnCombAction(Option<Arc<CThostFtdcCombActionField>>),
    OnRtnFromBankToFutureByBank(Option<Arc<CThostFtdcRspTransferField>>),
    OnRtnFromFutureToBankByBank(Option<Arc<CThostFtdcRspTransferField>>),
    OnRtnRepealFromBankToFutureByBank(Option<Arc<CThostFtdcRspRepealField>>),
    OnRtnRepealFromFutureToBankByBank(Option<Arc<CThostFtdcRspRepealField>>),
    OnRtnFromBankToFutureByFuture(Option<Arc<CThostFtdcRspTransferField>>),
    OnRtnFromFutureToBankByFuture(Option<Arc<CThostFtdcRspTransferField>>),
    OnRtnRepealFromBankToFutureByFutureManual(Option<Arc<CThostFtdcRspRepealField>>),
    OnRtnRepealFromFutureToBankByFutureManual(Option<Arc<CThostFtdcRspRepealField>>),
    OnRtnQueryBankBalanceByFuture(Option<Arc<CThostFtdcNotifyQueryAccountField>>),
    OnRtnRepealFromBankToFutureByFuture(Option<Arc<CThostFtdcRspRepealField>>),
    OnRtnRepealFromFutureToBankByFuture(Option<Arc<CThostFtdcRspRepealField>>),
    OnRtnOpenAccountByBank(Option<Arc<CThostFtdcOpenAccountField>>),
    OnRtnCancelAccountByBank(Option<Arc<CThostFtdcCancelAccountField>>),
    OnRtnChangeAccountByBank(Option<Arc<CThostFtdcChangeAccountField>>),
    OnRtnOffsetSetting(Option<Arc<CThostFtdcOffsetSettingField>>),
}

pub fn cvoid_to_errrtn_arc(evt: EnumOnErrRtnEvent, param: *const c_void) -> OnErrRtnOptArc {
    match evt {
        EnumOnErrRtnEvent::OnErrRtnOrderInsert => {
            let fld = param as *const CThostFtdcInputOrderField;
            return OnErrRtnOptArc::OnErrRtnOrderInsert(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnOrderAction => {
            let fld = param as *const CThostFtdcOrderActionField;
            return OnErrRtnOptArc::OnErrRtnOrderAction(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnExecOrderInsert => {
            let fld = param as *const CThostFtdcInputExecOrderField;
            return OnErrRtnOptArc::OnErrRtnExecOrderInsert(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnExecOrderAction => {
            let fld = param as *const CThostFtdcExecOrderActionField;
            return OnErrRtnOptArc::OnErrRtnExecOrderAction(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnForQuoteInsert => {
            let fld = param as *const CThostFtdcInputForQuoteField;
            return OnErrRtnOptArc::OnErrRtnForQuoteInsert(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnQuoteInsert => {
            let fld = param as *const CThostFtdcInputQuoteField;
            return OnErrRtnOptArc::OnErrRtnQuoteInsert(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnQuoteAction => {
            let fld = param as *const CThostFtdcQuoteActionField;
            return OnErrRtnOptArc::OnErrRtnQuoteAction(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnBatchOrderAction => {
            let fld = param as *const CThostFtdcBatchOrderActionField;
            return OnErrRtnOptArc::OnErrRtnBatchOrderAction(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnOptionSelfCloseInsert => {
            let fld = param as *const CThostFtdcInputOptionSelfCloseField;
            return OnErrRtnOptArc::OnErrRtnOptionSelfCloseInsert(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnOptionSelfCloseAction => {
            let fld = param as *const CThostFtdcOptionSelfCloseActionField;
            return OnErrRtnOptArc::OnErrRtnOptionSelfCloseAction(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnCombActionInsert => {
            let fld = param as *const CThostFtdcInputCombActionField;
            return OnErrRtnOptArc::OnErrRtnCombActionInsert(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnBankToFutureByFuture => {
            let fld = param as *const CThostFtdcReqTransferField;
            return OnErrRtnOptArc::OnErrRtnBankToFutureByFuture(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnFutureToBankByFuture => {
            let fld = param as *const CThostFtdcReqTransferField;
            return OnErrRtnOptArc::OnErrRtnFutureToBankByFuture(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnRepealBankToFutureByFutureManual => {
            let fld = param as *const CThostFtdcReqRepealField;
            return OnErrRtnOptArc::OnErrRtnRepealBankToFutureByFutureManual(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnRepealFutureToBankByFutureManual => {
            let fld = param as *const CThostFtdcReqRepealField;
            return OnErrRtnOptArc::OnErrRtnRepealFutureToBankByFutureManual(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnQueryBankBalanceByFuture => {
            let fld = param as *const CThostFtdcReqQueryAccountField;
            return OnErrRtnOptArc::OnErrRtnQueryBankBalanceByFuture(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnOffsetSetting => {
            let fld = param as *const CThostFtdcInputOffsetSettingField;
            return OnErrRtnOptArc::OnErrRtnOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnCancelOffsetSetting => {
            let fld = param as *const CThostFtdcCancelOffsetSettingField;
            return OnErrRtnOptArc::OnErrRtnCancelOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
    }
}
pub fn cvoid_to_rsp_arc(evt: EnumOnRspEvent, param: *const c_void) -> OnRspOptArc {
    match evt {
        EnumOnRspEvent::OnRspUserLogin => {
            let fld = param as *const CThostFtdcRspUserLoginField;
            return OnRspOptArc::OnRspUserLogin(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspUserLogout => {
            let fld = param as *const CThostFtdcUserLogoutField;
            return OnRspOptArc::OnRspUserLogout(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryMulticastInstrument => {
            let fld = param as *const CThostFtdcMulticastInstrumentField;
            return OnRspOptArc::OnRspQryMulticastInstrument(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspError => {
            let fld = param as *const CThostFtdcRspInfoField;
            return OnRspOptArc::OnRspError(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspSubMarketData => {
            let fld = param as *const CThostFtdcSpecificInstrumentField;
            return OnRspOptArc::OnRspSubMarketData(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspUnSubMarketData => {
            let fld = param as *const CThostFtdcSpecificInstrumentField;
            return OnRspOptArc::OnRspUnSubMarketData(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspSubForQuoteRsp => {
            let fld = param as *const CThostFtdcSpecificInstrumentField;
            return OnRspOptArc::OnRspSubForQuoteRsp(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspUnSubForQuoteRsp => {
            let fld = param as *const CThostFtdcSpecificInstrumentField;
            return OnRspOptArc::OnRspUnSubForQuoteRsp(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspAuthenticate => {
            let fld = param as *const CThostFtdcRspAuthenticateField;
            return OnRspOptArc::OnRspAuthenticate(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspUserPasswordUpdate => {
            let fld = param as *const CThostFtdcUserPasswordUpdateField;
            return OnRspOptArc::OnRspUserPasswordUpdate(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspTradingAccountPasswordUpdate => {
            let fld = param as *const CThostFtdcTradingAccountPasswordUpdateField;
            return OnRspOptArc::OnRspTradingAccountPasswordUpdate(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspUserAuthMethod => {
            let fld = param as *const CThostFtdcRspUserAuthMethodField;
            return OnRspOptArc::OnRspUserAuthMethod(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspGenUserCaptcha => {
            let fld = param as *const CThostFtdcRspGenUserCaptchaField;
            return OnRspOptArc::OnRspGenUserCaptcha(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspGenUserText => {
            let fld = param as *const CThostFtdcRspGenUserTextField;
            return OnRspOptArc::OnRspGenUserText(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspOrderInsert => {
            let fld = param as *const CThostFtdcInputOrderField;
            return OnRspOptArc::OnRspOrderInsert(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspParkedOrderInsert => {
            let fld = param as *const CThostFtdcParkedOrderField;
            return OnRspOptArc::OnRspParkedOrderInsert(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspParkedOrderAction => {
            let fld = param as *const CThostFtdcParkedOrderActionField;
            return OnRspOptArc::OnRspParkedOrderAction(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspOrderAction => {
            let fld = param as *const CThostFtdcInputOrderActionField;
            return OnRspOptArc::OnRspOrderAction(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryMaxOrderVolume => {
            let fld = param as *const CThostFtdcQryMaxOrderVolumeField;
            return OnRspOptArc::OnRspQryMaxOrderVolume(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspSettlementInfoConfirm => {
            let fld = param as *const CThostFtdcSettlementInfoConfirmField;
            return OnRspOptArc::OnRspSettlementInfoConfirm(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspRemoveParkedOrder => {
            let fld = param as *const CThostFtdcRemoveParkedOrderField;
            return OnRspOptArc::OnRspRemoveParkedOrder(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspRemoveParkedOrderAction => {
            let fld = param as *const CThostFtdcRemoveParkedOrderActionField;
            return OnRspOptArc::OnRspRemoveParkedOrderAction(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspExecOrderInsert => {
            let fld = param as *const CThostFtdcInputExecOrderField;
            return OnRspOptArc::OnRspExecOrderInsert(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspExecOrderAction => {
            let fld = param as *const CThostFtdcInputExecOrderActionField;
            return OnRspOptArc::OnRspExecOrderAction(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspForQuoteInsert => {
            let fld = param as *const CThostFtdcInputForQuoteField;
            return OnRspOptArc::OnRspForQuoteInsert(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQuoteInsert => {
            let fld = param as *const CThostFtdcInputQuoteField;
            return OnRspOptArc::OnRspQuoteInsert(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQuoteAction => {
            let fld = param as *const CThostFtdcInputQuoteActionField;
            return OnRspOptArc::OnRspQuoteAction(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspBatchOrderAction => {
            let fld = param as *const CThostFtdcInputBatchOrderActionField;
            return OnRspOptArc::OnRspBatchOrderAction(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspOptionSelfCloseInsert => {
            let fld = param as *const CThostFtdcInputOptionSelfCloseField;
            return OnRspOptArc::OnRspOptionSelfCloseInsert(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspOptionSelfCloseAction => {
            let fld = param as *const CThostFtdcInputOptionSelfCloseActionField;
            return OnRspOptArc::OnRspOptionSelfCloseAction(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspCombActionInsert => {
            let fld = param as *const CThostFtdcInputCombActionField;
            return OnRspOptArc::OnRspCombActionInsert(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryOrder => {
            let fld = param as *const CThostFtdcOrderField;
            return OnRspOptArc::OnRspQryOrder(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTrade => {
            let fld = param as *const CThostFtdcTradeField;
            return OnRspOptArc::OnRspQryTrade(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPosition => {
            let fld = param as *const CThostFtdcInvestorPositionField;
            return OnRspOptArc::OnRspQryInvestorPosition(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTradingAccount => {
            let fld = param as *const CThostFtdcTradingAccountField;
            return OnRspOptArc::OnRspQryTradingAccount(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestor => {
            let fld = param as *const CThostFtdcInvestorField;
            return OnRspOptArc::OnRspQryInvestor(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTradingCode => {
            let fld = param as *const CThostFtdcTradingCodeField;
            return OnRspOptArc::OnRspQryTradingCode(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInstrumentMarginRate => {
            let fld = param as *const CThostFtdcInstrumentMarginRateField;
            return OnRspOptArc::OnRspQryInstrumentMarginRate(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInstrumentCommissionRate => {
            let fld = param as *const CThostFtdcInstrumentCommissionRateField;
            return OnRspOptArc::OnRspQryInstrumentCommissionRate(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryUserSession => {
            let fld = param as *const CThostFtdcUserSessionField;
            return OnRspOptArc::OnRspQryUserSession(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryExchange => {
            let fld = param as *const CThostFtdcExchangeField;
            return OnRspOptArc::OnRspQryExchange(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryProduct => {
            let fld = param as *const CThostFtdcProductField;
            return OnRspOptArc::OnRspQryProduct(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInstrument => {
            let fld = param as *const CThostFtdcInstrumentField;
            return OnRspOptArc::OnRspQryInstrument(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryDepthMarketData => {
            let fld = param as *const CThostFtdcDepthMarketDataField;
            return OnRspOptArc::OnRspQryDepthMarketData(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTraderOffer => {
            let fld = param as *const CThostFtdcTraderOfferField;
            return OnRspOptArc::OnRspQryTraderOffer(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySettlementInfo => {
            let fld = param as *const CThostFtdcSettlementInfoField;
            return OnRspOptArc::OnRspQrySettlementInfo(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTransferBank => {
            let fld = param as *const CThostFtdcTransferBankField;
            return OnRspOptArc::OnRspQryTransferBank(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPositionDetail => {
            let fld = param as *const CThostFtdcInvestorPositionDetailField;
            return OnRspOptArc::OnRspQryInvestorPositionDetail(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryNotice => {
            let fld = param as *const CThostFtdcNoticeField;
            return OnRspOptArc::OnRspQryNotice(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySettlementInfoConfirm => {
            let fld = param as *const CThostFtdcSettlementInfoConfirmField;
            return OnRspOptArc::OnRspQrySettlementInfoConfirm(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPositionCombineDetail => {
            let fld = param as *const CThostFtdcInvestorPositionCombineDetailField;
            return OnRspOptArc::OnRspQryInvestorPositionCombineDetail(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryCFMMCTradingAccountKey => {
            let fld = param as *const CThostFtdcCFMMCTradingAccountKeyField;
            return OnRspOptArc::OnRspQryCFMMCTradingAccountKey(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryEWarrantOffset => {
            let fld = param as *const CThostFtdcEWarrantOffsetField;
            return OnRspOptArc::OnRspQryEWarrantOffset(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorProductGroupMargin => {
            let fld = param as *const CThostFtdcInvestorProductGroupMarginField;
            return OnRspOptArc::OnRspQryInvestorProductGroupMargin(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryExchangeMarginRate => {
            let fld = param as *const CThostFtdcExchangeMarginRateField;
            return OnRspOptArc::OnRspQryExchangeMarginRate(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryExchangeMarginRateAdjust => {
            let fld = param as *const CThostFtdcExchangeMarginRateAdjustField;
            return OnRspOptArc::OnRspQryExchangeMarginRateAdjust(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryExchangeRate => {
            let fld = param as *const CThostFtdcExchangeRateField;
            return OnRspOptArc::OnRspQryExchangeRate(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySecAgentACIDMap => {
            let fld = param as *const CThostFtdcSecAgentACIDMapField;
            return OnRspOptArc::OnRspQrySecAgentACIDMap(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryProductExchRate => {
            let fld = param as *const CThostFtdcProductExchRateField;
            return OnRspOptArc::OnRspQryProductExchRate(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryProductGroup => {
            let fld = param as *const CThostFtdcProductGroupField;
            return OnRspOptArc::OnRspQryProductGroup(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryMMInstrumentCommissionRate => {
            let fld = param as *const CThostFtdcMMInstrumentCommissionRateField;
            return OnRspOptArc::OnRspQryMMInstrumentCommissionRate(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryMMOptionInstrCommRate => {
            let fld = param as *const CThostFtdcMMOptionInstrCommRateField;
            return OnRspOptArc::OnRspQryMMOptionInstrCommRate(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInstrumentOrderCommRate => {
            let fld = param as *const CThostFtdcInstrumentOrderCommRateField;
            return OnRspOptArc::OnRspQryInstrumentOrderCommRate(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySecAgentTradingAccount => {
            let fld = param as *const CThostFtdcTradingAccountField;
            return OnRspOptArc::OnRspQrySecAgentTradingAccount(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySecAgentCheckMode => {
            let fld = param as *const CThostFtdcSecAgentCheckModeField;
            return OnRspOptArc::OnRspQrySecAgentCheckMode(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySecAgentTradeInfo => {
            let fld = param as *const CThostFtdcSecAgentTradeInfoField;
            return OnRspOptArc::OnRspQrySecAgentTradeInfo(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryOptionInstrTradeCost => {
            let fld = param as *const CThostFtdcOptionInstrTradeCostField;
            return OnRspOptArc::OnRspQryOptionInstrTradeCost(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryOptionInstrCommRate => {
            let fld = param as *const CThostFtdcOptionInstrCommRateField;
            return OnRspOptArc::OnRspQryOptionInstrCommRate(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryExecOrder => {
            let fld = param as *const CThostFtdcExecOrderField;
            return OnRspOptArc::OnRspQryExecOrder(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryForQuote => {
            let fld = param as *const CThostFtdcForQuoteField;
            return OnRspOptArc::OnRspQryForQuote(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryQuote => {
            let fld = param as *const CThostFtdcQuoteField;
            return OnRspOptArc::OnRspQryQuote(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryOptionSelfClose => {
            let fld = param as *const CThostFtdcOptionSelfCloseField;
            return OnRspOptArc::OnRspQryOptionSelfClose(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestUnit => {
            let fld = param as *const CThostFtdcInvestUnitField;
            return OnRspOptArc::OnRspQryInvestUnit(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryCombInstrumentGuard => {
            let fld = param as *const CThostFtdcCombInstrumentGuardField;
            return OnRspOptArc::OnRspQryCombInstrumentGuard(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryCombAction => {
            let fld = param as *const CThostFtdcCombActionField;
            return OnRspOptArc::OnRspQryCombAction(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTransferSerial => {
            let fld = param as *const CThostFtdcTransferSerialField;
            return OnRspOptArc::OnRspQryTransferSerial(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryAccountregister => {
            let fld = param as *const CThostFtdcAccountregisterField;
            return OnRspOptArc::OnRspQryAccountregister(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryContractBank => {
            let fld = param as *const CThostFtdcContractBankField;
            return OnRspOptArc::OnRspQryContractBank(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryParkedOrder => {
            let fld = param as *const CThostFtdcParkedOrderField;
            return OnRspOptArc::OnRspQryParkedOrder(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryParkedOrderAction => {
            let fld = param as *const CThostFtdcParkedOrderActionField;
            return OnRspOptArc::OnRspQryParkedOrderAction(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTradingNotice => {
            let fld = param as *const CThostFtdcTradingNoticeField;
            return OnRspOptArc::OnRspQryTradingNotice(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryBrokerTradingParams => {
            let fld = param as *const CThostFtdcBrokerTradingParamsField;
            return OnRspOptArc::OnRspQryBrokerTradingParams(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryBrokerTradingAlgos => {
            let fld = param as *const CThostFtdcBrokerTradingAlgosField;
            return OnRspOptArc::OnRspQryBrokerTradingAlgos(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQueryCFMMCTradingAccountToken => {
            let fld = param as *const CThostFtdcQueryCFMMCTradingAccountTokenField;
            return OnRspOptArc::OnRspQueryCFMMCTradingAccountToken(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspFromBankToFutureByFuture => {
            let fld = param as *const CThostFtdcReqTransferField;
            return OnRspOptArc::OnRspFromBankToFutureByFuture(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspFromFutureToBankByFuture => {
            let fld = param as *const CThostFtdcReqTransferField;
            return OnRspOptArc::OnRspFromFutureToBankByFuture(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQueryBankAccountMoneyByFuture => {
            let fld = param as *const CThostFtdcReqQueryAccountField;
            return OnRspOptArc::OnRspQueryBankAccountMoneyByFuture(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryClassifiedInstrument => {
            let fld = param as *const CThostFtdcInstrumentField;
            return OnRspOptArc::OnRspQryClassifiedInstrument(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryCombPromotionParam => {
            let fld = param as *const CThostFtdcCombPromotionParamField;
            return OnRspOptArc::OnRspQryCombPromotionParam(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRiskSettleInvstPosition => {
            let fld = param as *const CThostFtdcRiskSettleInvstPositionField;
            return OnRspOptArc::OnRspQryRiskSettleInvstPosition(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRiskSettleProductStatus => {
            let fld = param as *const CThostFtdcRiskSettleProductStatusField;
            return OnRspOptArc::OnRspQryRiskSettleProductStatus(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPBMFutureParameter => {
            let fld = param as *const CThostFtdcSPBMFutureParameterField;
            return OnRspOptArc::OnRspQrySPBMFutureParameter(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPBMOptionParameter => {
            let fld = param as *const CThostFtdcSPBMOptionParameterField;
            return OnRspOptArc::OnRspQrySPBMOptionParameter(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPBMIntraParameter => {
            let fld = param as *const CThostFtdcSPBMIntraParameterField;
            return OnRspOptArc::OnRspQrySPBMIntraParameter(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPBMInterParameter => {
            let fld = param as *const CThostFtdcSPBMInterParameterField;
            return OnRspOptArc::OnRspQrySPBMInterParameter(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPBMPortfDefinition => {
            let fld = param as *const CThostFtdcSPBMPortfDefinitionField;
            return OnRspOptArc::OnRspQrySPBMPortfDefinition(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPBMInvestorPortfDef => {
            let fld = param as *const CThostFtdcSPBMInvestorPortfDefField;
            return OnRspOptArc::OnRspQrySPBMInvestorPortfDef(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPortfMarginRatio => {
            let fld = param as *const CThostFtdcInvestorPortfMarginRatioField;
            return OnRspOptArc::OnRspQryInvestorPortfMarginRatio(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorProdSPBMDetail => {
            let fld = param as *const CThostFtdcInvestorProdSPBMDetailField;
            return OnRspOptArc::OnRspQryInvestorProdSPBMDetail(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorCommoditySPMMMargin => {
            let fld = param as *const CThostFtdcInvestorCommoditySPMMMarginField;
            return OnRspOptArc::OnRspQryInvestorCommoditySPMMMargin(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorCommodityGroupSPMMMargin => {
            let fld = param as *const CThostFtdcInvestorCommodityGroupSPMMMarginField;
            return OnRspOptArc::OnRspQryInvestorCommodityGroupSPMMMargin(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPMMInstParam => {
            let fld = param as *const CThostFtdcSPMMInstParamField;
            return OnRspOptArc::OnRspQrySPMMInstParam(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPMMProductParam => {
            let fld = param as *const CThostFtdcSPMMProductParamField;
            return OnRspOptArc::OnRspQrySPMMProductParam(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPBMAddOnInterParameter => {
            let fld = param as *const CThostFtdcSPBMAddOnInterParameterField;
            return OnRspOptArc::OnRspQrySPBMAddOnInterParameter(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSCombProductInfo => {
            let fld = param as *const CThostFtdcRCAMSCombProductInfoField;
            return OnRspOptArc::OnRspQryRCAMSCombProductInfo(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSInstrParameter => {
            let fld = param as *const CThostFtdcRCAMSInstrParameterField;
            return OnRspOptArc::OnRspQryRCAMSInstrParameter(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSIntraParameter => {
            let fld = param as *const CThostFtdcRCAMSIntraParameterField;
            return OnRspOptArc::OnRspQryRCAMSIntraParameter(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSInterParameter => {
            let fld = param as *const CThostFtdcRCAMSInterParameterField;
            return OnRspOptArc::OnRspQryRCAMSInterParameter(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSShortOptAdjustParam => {
            let fld = param as *const CThostFtdcRCAMSShortOptAdjustParamField;
            return OnRspOptArc::OnRspQryRCAMSShortOptAdjustParam(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSInvestorCombPosition => {
            let fld = param as *const CThostFtdcRCAMSInvestorCombPositionField;
            return OnRspOptArc::OnRspQryRCAMSInvestorCombPosition(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorProdRCAMSMargin => {
            let fld = param as *const CThostFtdcInvestorProdRCAMSMarginField;
            return OnRspOptArc::OnRspQryInvestorProdRCAMSMargin(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRULEInstrParameter => {
            let fld = param as *const CThostFtdcRULEInstrParameterField;
            return OnRspOptArc::OnRspQryRULEInstrParameter(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRULEIntraParameter => {
            let fld = param as *const CThostFtdcRULEIntraParameterField;
            return OnRspOptArc::OnRspQryRULEIntraParameter(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRULEInterParameter => {
            let fld = param as *const CThostFtdcRULEInterParameterField;
            return OnRspOptArc::OnRspQryRULEInterParameter(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorProdRULEMargin => {
            let fld = param as *const CThostFtdcInvestorProdRULEMarginField;
            return OnRspOptArc::OnRspQryInvestorProdRULEMargin(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPortfSetting => {
            let fld = param as *const CThostFtdcInvestorPortfSettingField;
            return OnRspOptArc::OnRspQryInvestorPortfSetting(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorInfoCommRec => {
            let fld = param as *const CThostFtdcInvestorInfoCommRecField;
            return OnRspOptArc::OnRspQryInvestorInfoCommRec(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryCombLeg => {
            let fld = param as *const CThostFtdcCombLegField;
            return OnRspOptArc::OnRspQryCombLeg(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspOffsetSetting => {
            let fld = param as *const CThostFtdcInputOffsetSettingField;
            return OnRspOptArc::OnRspOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspCancelOffsetSetting => {
            let fld = param as *const CThostFtdcInputOffsetSettingField;
            return OnRspOptArc::OnRspCancelOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryOffsetSetting => {
            let fld = param as *const CThostFtdcOffsetSettingField;
            return OnRspOptArc::OnRspQryOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
    }
}
pub fn cvoid_to_rtn_arc(evt: EnumOnRtnEvent, param: *const c_void) -> OnRtnOptArc {
    match evt {
        EnumOnRtnEvent::OnRtnDepthMarketData => {
            let fld = param as *const CThostFtdcDepthMarketDataField;
            return OnRtnOptArc::OnRtnDepthMarketData(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnForQuoteRsp => {
            let fld = param as *const CThostFtdcForQuoteRspField;
            return OnRtnOptArc::OnRtnForQuoteRsp(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnOrder => {
            let fld = param as *const CThostFtdcOrderField;
            return OnRtnOptArc::OnRtnOrder(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnTrade => {
            let fld = param as *const CThostFtdcTradeField;
            return OnRtnOptArc::OnRtnTrade(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnInstrumentStatus => {
            let fld = param as *const CThostFtdcInstrumentStatusField;
            return OnRtnOptArc::OnRtnInstrumentStatus(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnBulletin => {
            let fld = param as *const CThostFtdcBulletinField;
            return OnRtnOptArc::OnRtnBulletin(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnTradingNotice => {
            let fld = param as *const CThostFtdcTradingNoticeInfoField;
            return OnRtnOptArc::OnRtnTradingNotice(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnErrorConditionalOrder => {
            let fld = param as *const CThostFtdcErrorConditionalOrderField;
            return OnRtnOptArc::OnRtnErrorConditionalOrder(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnExecOrder => {
            let fld = param as *const CThostFtdcExecOrderField;
            return OnRtnOptArc::OnRtnExecOrder(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnQuote => {
            let fld = param as *const CThostFtdcQuoteField;
            return OnRtnOptArc::OnRtnQuote(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnCFMMCTradingAccountToken => {
            let fld = param as *const CThostFtdcCFMMCTradingAccountTokenField;
            return OnRtnOptArc::OnRtnCFMMCTradingAccountToken(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnOptionSelfClose => {
            let fld = param as *const CThostFtdcOptionSelfCloseField;
            return OnRtnOptArc::OnRtnOptionSelfClose(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnCombAction => {
            let fld = param as *const CThostFtdcCombActionField;
            return OnRtnOptArc::OnRtnCombAction(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnFromBankToFutureByBank => {
            let fld = param as *const CThostFtdcRspTransferField;
            return OnRtnOptArc::OnRtnFromBankToFutureByBank(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnFromFutureToBankByBank => {
            let fld = param as *const CThostFtdcRspTransferField;
            return OnRtnOptArc::OnRtnFromFutureToBankByBank(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromBankToFutureByBank => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptArc::OnRtnRepealFromBankToFutureByBank(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromFutureToBankByBank => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptArc::OnRtnRepealFromFutureToBankByBank(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnFromBankToFutureByFuture => {
            let fld = param as *const CThostFtdcRspTransferField;
            return OnRtnOptArc::OnRtnFromBankToFutureByFuture(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnFromFutureToBankByFuture => {
            let fld = param as *const CThostFtdcRspTransferField;
            return OnRtnOptArc::OnRtnFromFutureToBankByFuture(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromBankToFutureByFutureManual => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptArc::OnRtnRepealFromBankToFutureByFutureManual(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromFutureToBankByFutureManual => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptArc::OnRtnRepealFromFutureToBankByFutureManual(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnQueryBankBalanceByFuture => {
            let fld = param as *const CThostFtdcNotifyQueryAccountField;
            return OnRtnOptArc::OnRtnQueryBankBalanceByFuture(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromBankToFutureByFuture => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptArc::OnRtnRepealFromBankToFutureByFuture(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromFutureToBankByFuture => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptArc::OnRtnRepealFromFutureToBankByFuture(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnOpenAccountByBank => {
            let fld = param as *const CThostFtdcOpenAccountField;
            return OnRtnOptArc::OnRtnOpenAccountByBank(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnCancelAccountByBank => {
            let fld = param as *const CThostFtdcCancelAccountField;
            return OnRtnOptArc::OnRtnCancelAccountByBank(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnChangeAccountByBank => {
            let fld = param as *const CThostFtdcChangeAccountField;
            return OnRtnOptArc::OnRtnChangeAccountByBank(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnOffsetSetting => {
            let fld = param as *const CThostFtdcOffsetSettingField;
            return OnRtnOptArc::OnRtnOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(Arc::new(unsafe { (*fld).clone() }))
            });
        }
    }
}
