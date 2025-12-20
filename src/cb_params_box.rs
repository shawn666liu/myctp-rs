use crate::ctp::*;
use std::ffi::c_void;

// CTP回调参数，为避免跨线程传递反复拷贝，使用Box，仅拷贝一次

#[derive(Debug, Clone)]
pub enum OnFrontEvent {
    OnFrontConnected,
    OnFrontDisconnected(i32),
}

#[derive(Debug, Clone)]
pub enum OnErrRtnOptBox {
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
    OnErrRtnOffsetSetting(Option<Box<CThostFtdcInputOffsetSettingField>>),
    OnErrRtnCancelOffsetSetting(Option<Box<CThostFtdcCancelOffsetSettingField>>),
}

#[derive(Debug, Clone)]
pub enum OnRspOptBox {
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
    OnRspQryUserSession(Option<Box<CThostFtdcUserSessionField>>),
    OnRspQryExchange(Option<Box<CThostFtdcExchangeField>>),
    OnRspQryProduct(Option<Box<CThostFtdcProductField>>),
    OnRspQryInstrument(Option<Box<CThostFtdcInstrumentField>>),
    OnRspQryDepthMarketData(Option<Box<CThostFtdcDepthMarketDataField>>),
    OnRspQryTraderOffer(Option<Box<CThostFtdcTraderOfferField>>),
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
    OnRspQryRiskSettleInvstPosition(Option<Box<CThostFtdcRiskSettleInvstPositionField>>),
    OnRspQryRiskSettleProductStatus(Option<Box<CThostFtdcRiskSettleProductStatusField>>),
    OnRspQrySPBMFutureParameter(Option<Box<CThostFtdcSPBMFutureParameterField>>),
    OnRspQrySPBMOptionParameter(Option<Box<CThostFtdcSPBMOptionParameterField>>),
    OnRspQrySPBMIntraParameter(Option<Box<CThostFtdcSPBMIntraParameterField>>),
    OnRspQrySPBMInterParameter(Option<Box<CThostFtdcSPBMInterParameterField>>),
    OnRspQrySPBMPortfDefinition(Option<Box<CThostFtdcSPBMPortfDefinitionField>>),
    OnRspQrySPBMInvestorPortfDef(Option<Box<CThostFtdcSPBMInvestorPortfDefField>>),
    OnRspQryInvestorPortfMarginRatio(Option<Box<CThostFtdcInvestorPortfMarginRatioField>>),
    OnRspQryInvestorProdSPBMDetail(Option<Box<CThostFtdcInvestorProdSPBMDetailField>>),
    OnRspQryInvestorCommoditySPMMMargin(Option<Box<CThostFtdcInvestorCommoditySPMMMarginField>>),
    OnRspQryInvestorCommodityGroupSPMMMargin(
        Option<Box<CThostFtdcInvestorCommodityGroupSPMMMarginField>>,
    ),
    OnRspQrySPMMInstParam(Option<Box<CThostFtdcSPMMInstParamField>>),
    OnRspQrySPMMProductParam(Option<Box<CThostFtdcSPMMProductParamField>>),
    OnRspQrySPBMAddOnInterParameter(Option<Box<CThostFtdcSPBMAddOnInterParameterField>>),
    OnRspQryRCAMSCombProductInfo(Option<Box<CThostFtdcRCAMSCombProductInfoField>>),
    OnRspQryRCAMSInstrParameter(Option<Box<CThostFtdcRCAMSInstrParameterField>>),
    OnRspQryRCAMSIntraParameter(Option<Box<CThostFtdcRCAMSIntraParameterField>>),
    OnRspQryRCAMSInterParameter(Option<Box<CThostFtdcRCAMSInterParameterField>>),
    OnRspQryRCAMSShortOptAdjustParam(Option<Box<CThostFtdcRCAMSShortOptAdjustParamField>>),
    OnRspQryRCAMSInvestorCombPosition(Option<Box<CThostFtdcRCAMSInvestorCombPositionField>>),
    OnRspQryInvestorProdRCAMSMargin(Option<Box<CThostFtdcInvestorProdRCAMSMarginField>>),
    OnRspQryRULEInstrParameter(Option<Box<CThostFtdcRULEInstrParameterField>>),
    OnRspQryRULEIntraParameter(Option<Box<CThostFtdcRULEIntraParameterField>>),
    OnRspQryRULEInterParameter(Option<Box<CThostFtdcRULEInterParameterField>>),
    OnRspQryInvestorProdRULEMargin(Option<Box<CThostFtdcInvestorProdRULEMarginField>>),
    OnRspQryInvestorPortfSetting(Option<Box<CThostFtdcInvestorPortfSettingField>>),
    OnRspQryInvestorInfoCommRec(Option<Box<CThostFtdcInvestorInfoCommRecField>>),
    OnRspQryCombLeg(Option<Box<CThostFtdcCombLegField>>),
    OnRspOffsetSetting(Option<Box<CThostFtdcInputOffsetSettingField>>),
    OnRspCancelOffsetSetting(Option<Box<CThostFtdcInputOffsetSettingField>>),
    OnRspQryOffsetSetting(Option<Box<CThostFtdcOffsetSettingField>>),
}

#[derive(Debug, Clone)]
pub enum OnRtnOptBox {
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
    OnRtnOffsetSetting(Option<Box<CThostFtdcOffsetSettingField>>),
}

pub fn cvoid_to_errrtn_box(evt: EnumOnErrRtnEvent, param: *const c_void) -> OnErrRtnOptBox {
    match evt {
        EnumOnErrRtnEvent::OnErrRtnOrderInsert => {
            let fld = param as *const CThostFtdcInputOrderField;
            return OnErrRtnOptBox::OnErrRtnOrderInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnOrderAction => {
            let fld = param as *const CThostFtdcOrderActionField;
            return OnErrRtnOptBox::OnErrRtnOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnExecOrderInsert => {
            let fld = param as *const CThostFtdcInputExecOrderField;
            return OnErrRtnOptBox::OnErrRtnExecOrderInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnExecOrderAction => {
            let fld = param as *const CThostFtdcExecOrderActionField;
            return OnErrRtnOptBox::OnErrRtnExecOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnForQuoteInsert => {
            let fld = param as *const CThostFtdcInputForQuoteField;
            return OnErrRtnOptBox::OnErrRtnForQuoteInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnQuoteInsert => {
            let fld = param as *const CThostFtdcInputQuoteField;
            return OnErrRtnOptBox::OnErrRtnQuoteInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnQuoteAction => {
            let fld = param as *const CThostFtdcQuoteActionField;
            return OnErrRtnOptBox::OnErrRtnQuoteAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnBatchOrderAction => {
            let fld = param as *const CThostFtdcBatchOrderActionField;
            return OnErrRtnOptBox::OnErrRtnBatchOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnOptionSelfCloseInsert => {
            let fld = param as *const CThostFtdcInputOptionSelfCloseField;
            return OnErrRtnOptBox::OnErrRtnOptionSelfCloseInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnOptionSelfCloseAction => {
            let fld = param as *const CThostFtdcOptionSelfCloseActionField;
            return OnErrRtnOptBox::OnErrRtnOptionSelfCloseAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnCombActionInsert => {
            let fld = param as *const CThostFtdcInputCombActionField;
            return OnErrRtnOptBox::OnErrRtnCombActionInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnBankToFutureByFuture => {
            let fld = param as *const CThostFtdcReqTransferField;
            return OnErrRtnOptBox::OnErrRtnBankToFutureByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnFutureToBankByFuture => {
            let fld = param as *const CThostFtdcReqTransferField;
            return OnErrRtnOptBox::OnErrRtnFutureToBankByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnRepealBankToFutureByFutureManual => {
            let fld = param as *const CThostFtdcReqRepealField;
            return OnErrRtnOptBox::OnErrRtnRepealBankToFutureByFutureManual(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnRepealFutureToBankByFutureManual => {
            let fld = param as *const CThostFtdcReqRepealField;
            return OnErrRtnOptBox::OnErrRtnRepealFutureToBankByFutureManual(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnQueryBankBalanceByFuture => {
            let fld = param as *const CThostFtdcReqQueryAccountField;
            return OnErrRtnOptBox::OnErrRtnQueryBankBalanceByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnOffsetSetting => {
            let fld = param as *const CThostFtdcInputOffsetSettingField;
            return OnErrRtnOptBox::OnErrRtnOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnErrRtnEvent::OnErrRtnCancelOffsetSetting => {
            let fld = param as *const CThostFtdcCancelOffsetSettingField;
            return OnErrRtnOptBox::OnErrRtnCancelOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
    }
}
pub fn cvoid_to_rsp_box(evt: EnumOnRspEvent, param: *const c_void) -> OnRspOptBox {
    match evt {
        EnumOnRspEvent::OnRspUserLogin => {
            let fld = param as *const CThostFtdcRspUserLoginField;
            return OnRspOptBox::OnRspUserLogin(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspUserLogout => {
            let fld = param as *const CThostFtdcUserLogoutField;
            return OnRspOptBox::OnRspUserLogout(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryMulticastInstrument => {
            let fld = param as *const CThostFtdcMulticastInstrumentField;
            return OnRspOptBox::OnRspQryMulticastInstrument(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspError => {
            let fld = param as *const CThostFtdcRspInfoField;
            return OnRspOptBox::OnRspError(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspSubMarketData => {
            let fld = param as *const CThostFtdcSpecificInstrumentField;
            return OnRspOptBox::OnRspSubMarketData(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspUnSubMarketData => {
            let fld = param as *const CThostFtdcSpecificInstrumentField;
            return OnRspOptBox::OnRspUnSubMarketData(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspSubForQuoteRsp => {
            let fld = param as *const CThostFtdcSpecificInstrumentField;
            return OnRspOptBox::OnRspSubForQuoteRsp(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspUnSubForQuoteRsp => {
            let fld = param as *const CThostFtdcSpecificInstrumentField;
            return OnRspOptBox::OnRspUnSubForQuoteRsp(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspAuthenticate => {
            let fld = param as *const CThostFtdcRspAuthenticateField;
            return OnRspOptBox::OnRspAuthenticate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspUserPasswordUpdate => {
            let fld = param as *const CThostFtdcUserPasswordUpdateField;
            return OnRspOptBox::OnRspUserPasswordUpdate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspTradingAccountPasswordUpdate => {
            let fld = param as *const CThostFtdcTradingAccountPasswordUpdateField;
            return OnRspOptBox::OnRspTradingAccountPasswordUpdate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspUserAuthMethod => {
            let fld = param as *const CThostFtdcRspUserAuthMethodField;
            return OnRspOptBox::OnRspUserAuthMethod(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspGenUserCaptcha => {
            let fld = param as *const CThostFtdcRspGenUserCaptchaField;
            return OnRspOptBox::OnRspGenUserCaptcha(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspGenUserText => {
            let fld = param as *const CThostFtdcRspGenUserTextField;
            return OnRspOptBox::OnRspGenUserText(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspOrderInsert => {
            let fld = param as *const CThostFtdcInputOrderField;
            return OnRspOptBox::OnRspOrderInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspParkedOrderInsert => {
            let fld = param as *const CThostFtdcParkedOrderField;
            return OnRspOptBox::OnRspParkedOrderInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspParkedOrderAction => {
            let fld = param as *const CThostFtdcParkedOrderActionField;
            return OnRspOptBox::OnRspParkedOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspOrderAction => {
            let fld = param as *const CThostFtdcInputOrderActionField;
            return OnRspOptBox::OnRspOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryMaxOrderVolume => {
            let fld = param as *const CThostFtdcQryMaxOrderVolumeField;
            return OnRspOptBox::OnRspQryMaxOrderVolume(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspSettlementInfoConfirm => {
            let fld = param as *const CThostFtdcSettlementInfoConfirmField;
            return OnRspOptBox::OnRspSettlementInfoConfirm(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspRemoveParkedOrder => {
            let fld = param as *const CThostFtdcRemoveParkedOrderField;
            return OnRspOptBox::OnRspRemoveParkedOrder(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspRemoveParkedOrderAction => {
            let fld = param as *const CThostFtdcRemoveParkedOrderActionField;
            return OnRspOptBox::OnRspRemoveParkedOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspExecOrderInsert => {
            let fld = param as *const CThostFtdcInputExecOrderField;
            return OnRspOptBox::OnRspExecOrderInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspExecOrderAction => {
            let fld = param as *const CThostFtdcInputExecOrderActionField;
            return OnRspOptBox::OnRspExecOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspForQuoteInsert => {
            let fld = param as *const CThostFtdcInputForQuoteField;
            return OnRspOptBox::OnRspForQuoteInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQuoteInsert => {
            let fld = param as *const CThostFtdcInputQuoteField;
            return OnRspOptBox::OnRspQuoteInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQuoteAction => {
            let fld = param as *const CThostFtdcInputQuoteActionField;
            return OnRspOptBox::OnRspQuoteAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspBatchOrderAction => {
            let fld = param as *const CThostFtdcInputBatchOrderActionField;
            return OnRspOptBox::OnRspBatchOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspOptionSelfCloseInsert => {
            let fld = param as *const CThostFtdcInputOptionSelfCloseField;
            return OnRspOptBox::OnRspOptionSelfCloseInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspOptionSelfCloseAction => {
            let fld = param as *const CThostFtdcInputOptionSelfCloseActionField;
            return OnRspOptBox::OnRspOptionSelfCloseAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspCombActionInsert => {
            let fld = param as *const CThostFtdcInputCombActionField;
            return OnRspOptBox::OnRspCombActionInsert(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryOrder => {
            let fld = param as *const CThostFtdcOrderField;
            return OnRspOptBox::OnRspQryOrder(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTrade => {
            let fld = param as *const CThostFtdcTradeField;
            return OnRspOptBox::OnRspQryTrade(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPosition => {
            let fld = param as *const CThostFtdcInvestorPositionField;
            return OnRspOptBox::OnRspQryInvestorPosition(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTradingAccount => {
            let fld = param as *const CThostFtdcTradingAccountField;
            return OnRspOptBox::OnRspQryTradingAccount(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestor => {
            let fld = param as *const CThostFtdcInvestorField;
            return OnRspOptBox::OnRspQryInvestor(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTradingCode => {
            let fld = param as *const CThostFtdcTradingCodeField;
            return OnRspOptBox::OnRspQryTradingCode(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInstrumentMarginRate => {
            let fld = param as *const CThostFtdcInstrumentMarginRateField;
            return OnRspOptBox::OnRspQryInstrumentMarginRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInstrumentCommissionRate => {
            let fld = param as *const CThostFtdcInstrumentCommissionRateField;
            return OnRspOptBox::OnRspQryInstrumentCommissionRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryUserSession => {
            let fld = param as *const CThostFtdcUserSessionField;
            return OnRspOptBox::OnRspQryUserSession(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryExchange => {
            let fld = param as *const CThostFtdcExchangeField;
            return OnRspOptBox::OnRspQryExchange(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryProduct => {
            let fld = param as *const CThostFtdcProductField;
            return OnRspOptBox::OnRspQryProduct(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInstrument => {
            let fld = param as *const CThostFtdcInstrumentField;
            return OnRspOptBox::OnRspQryInstrument(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryDepthMarketData => {
            let fld = param as *const CThostFtdcDepthMarketDataField;
            return OnRspOptBox::OnRspQryDepthMarketData(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTraderOffer => {
            let fld = param as *const CThostFtdcTraderOfferField;
            return OnRspOptBox::OnRspQryTraderOffer(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySettlementInfo => {
            let fld = param as *const CThostFtdcSettlementInfoField;
            return OnRspOptBox::OnRspQrySettlementInfo(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTransferBank => {
            let fld = param as *const CThostFtdcTransferBankField;
            return OnRspOptBox::OnRspQryTransferBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPositionDetail => {
            let fld = param as *const CThostFtdcInvestorPositionDetailField;
            return OnRspOptBox::OnRspQryInvestorPositionDetail(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryNotice => {
            let fld = param as *const CThostFtdcNoticeField;
            return OnRspOptBox::OnRspQryNotice(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySettlementInfoConfirm => {
            let fld = param as *const CThostFtdcSettlementInfoConfirmField;
            return OnRspOptBox::OnRspQrySettlementInfoConfirm(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPositionCombineDetail => {
            let fld = param as *const CThostFtdcInvestorPositionCombineDetailField;
            return OnRspOptBox::OnRspQryInvestorPositionCombineDetail(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryCFMMCTradingAccountKey => {
            let fld = param as *const CThostFtdcCFMMCTradingAccountKeyField;
            return OnRspOptBox::OnRspQryCFMMCTradingAccountKey(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryEWarrantOffset => {
            let fld = param as *const CThostFtdcEWarrantOffsetField;
            return OnRspOptBox::OnRspQryEWarrantOffset(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorProductGroupMargin => {
            let fld = param as *const CThostFtdcInvestorProductGroupMarginField;
            return OnRspOptBox::OnRspQryInvestorProductGroupMargin(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryExchangeMarginRate => {
            let fld = param as *const CThostFtdcExchangeMarginRateField;
            return OnRspOptBox::OnRspQryExchangeMarginRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryExchangeMarginRateAdjust => {
            let fld = param as *const CThostFtdcExchangeMarginRateAdjustField;
            return OnRspOptBox::OnRspQryExchangeMarginRateAdjust(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryExchangeRate => {
            let fld = param as *const CThostFtdcExchangeRateField;
            return OnRspOptBox::OnRspQryExchangeRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySecAgentACIDMap => {
            let fld = param as *const CThostFtdcSecAgentACIDMapField;
            return OnRspOptBox::OnRspQrySecAgentACIDMap(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryProductExchRate => {
            let fld = param as *const CThostFtdcProductExchRateField;
            return OnRspOptBox::OnRspQryProductExchRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryProductGroup => {
            let fld = param as *const CThostFtdcProductGroupField;
            return OnRspOptBox::OnRspQryProductGroup(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryMMInstrumentCommissionRate => {
            let fld = param as *const CThostFtdcMMInstrumentCommissionRateField;
            return OnRspOptBox::OnRspQryMMInstrumentCommissionRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryMMOptionInstrCommRate => {
            let fld = param as *const CThostFtdcMMOptionInstrCommRateField;
            return OnRspOptBox::OnRspQryMMOptionInstrCommRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInstrumentOrderCommRate => {
            let fld = param as *const CThostFtdcInstrumentOrderCommRateField;
            return OnRspOptBox::OnRspQryInstrumentOrderCommRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySecAgentTradingAccount => {
            let fld = param as *const CThostFtdcTradingAccountField;
            return OnRspOptBox::OnRspQrySecAgentTradingAccount(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySecAgentCheckMode => {
            let fld = param as *const CThostFtdcSecAgentCheckModeField;
            return OnRspOptBox::OnRspQrySecAgentCheckMode(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySecAgentTradeInfo => {
            let fld = param as *const CThostFtdcSecAgentTradeInfoField;
            return OnRspOptBox::OnRspQrySecAgentTradeInfo(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryOptionInstrTradeCost => {
            let fld = param as *const CThostFtdcOptionInstrTradeCostField;
            return OnRspOptBox::OnRspQryOptionInstrTradeCost(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryOptionInstrCommRate => {
            let fld = param as *const CThostFtdcOptionInstrCommRateField;
            return OnRspOptBox::OnRspQryOptionInstrCommRate(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryExecOrder => {
            let fld = param as *const CThostFtdcExecOrderField;
            return OnRspOptBox::OnRspQryExecOrder(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryForQuote => {
            let fld = param as *const CThostFtdcForQuoteField;
            return OnRspOptBox::OnRspQryForQuote(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryQuote => {
            let fld = param as *const CThostFtdcQuoteField;
            return OnRspOptBox::OnRspQryQuote(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryOptionSelfClose => {
            let fld = param as *const CThostFtdcOptionSelfCloseField;
            return OnRspOptBox::OnRspQryOptionSelfClose(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestUnit => {
            let fld = param as *const CThostFtdcInvestUnitField;
            return OnRspOptBox::OnRspQryInvestUnit(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryCombInstrumentGuard => {
            let fld = param as *const CThostFtdcCombInstrumentGuardField;
            return OnRspOptBox::OnRspQryCombInstrumentGuard(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryCombAction => {
            let fld = param as *const CThostFtdcCombActionField;
            return OnRspOptBox::OnRspQryCombAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTransferSerial => {
            let fld = param as *const CThostFtdcTransferSerialField;
            return OnRspOptBox::OnRspQryTransferSerial(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryAccountregister => {
            let fld = param as *const CThostFtdcAccountregisterField;
            return OnRspOptBox::OnRspQryAccountregister(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryContractBank => {
            let fld = param as *const CThostFtdcContractBankField;
            return OnRspOptBox::OnRspQryContractBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryParkedOrder => {
            let fld = param as *const CThostFtdcParkedOrderField;
            return OnRspOptBox::OnRspQryParkedOrder(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryParkedOrderAction => {
            let fld = param as *const CThostFtdcParkedOrderActionField;
            return OnRspOptBox::OnRspQryParkedOrderAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryTradingNotice => {
            let fld = param as *const CThostFtdcTradingNoticeField;
            return OnRspOptBox::OnRspQryTradingNotice(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryBrokerTradingParams => {
            let fld = param as *const CThostFtdcBrokerTradingParamsField;
            return OnRspOptBox::OnRspQryBrokerTradingParams(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryBrokerTradingAlgos => {
            let fld = param as *const CThostFtdcBrokerTradingAlgosField;
            return OnRspOptBox::OnRspQryBrokerTradingAlgos(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQueryCFMMCTradingAccountToken => {
            let fld = param as *const CThostFtdcQueryCFMMCTradingAccountTokenField;
            return OnRspOptBox::OnRspQueryCFMMCTradingAccountToken(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspFromBankToFutureByFuture => {
            let fld = param as *const CThostFtdcReqTransferField;
            return OnRspOptBox::OnRspFromBankToFutureByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspFromFutureToBankByFuture => {
            let fld = param as *const CThostFtdcReqTransferField;
            return OnRspOptBox::OnRspFromFutureToBankByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQueryBankAccountMoneyByFuture => {
            let fld = param as *const CThostFtdcReqQueryAccountField;
            return OnRspOptBox::OnRspQueryBankAccountMoneyByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryClassifiedInstrument => {
            let fld = param as *const CThostFtdcInstrumentField;
            return OnRspOptBox::OnRspQryClassifiedInstrument(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryCombPromotionParam => {
            let fld = param as *const CThostFtdcCombPromotionParamField;
            return OnRspOptBox::OnRspQryCombPromotionParam(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRiskSettleInvstPosition => {
            let fld = param as *const CThostFtdcRiskSettleInvstPositionField;
            return OnRspOptBox::OnRspQryRiskSettleInvstPosition(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRiskSettleProductStatus => {
            let fld = param as *const CThostFtdcRiskSettleProductStatusField;
            return OnRspOptBox::OnRspQryRiskSettleProductStatus(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPBMFutureParameter => {
            let fld = param as *const CThostFtdcSPBMFutureParameterField;
            return OnRspOptBox::OnRspQrySPBMFutureParameter(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPBMOptionParameter => {
            let fld = param as *const CThostFtdcSPBMOptionParameterField;
            return OnRspOptBox::OnRspQrySPBMOptionParameter(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPBMIntraParameter => {
            let fld = param as *const CThostFtdcSPBMIntraParameterField;
            return OnRspOptBox::OnRspQrySPBMIntraParameter(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPBMInterParameter => {
            let fld = param as *const CThostFtdcSPBMInterParameterField;
            return OnRspOptBox::OnRspQrySPBMInterParameter(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPBMPortfDefinition => {
            let fld = param as *const CThostFtdcSPBMPortfDefinitionField;
            return OnRspOptBox::OnRspQrySPBMPortfDefinition(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPBMInvestorPortfDef => {
            let fld = param as *const CThostFtdcSPBMInvestorPortfDefField;
            return OnRspOptBox::OnRspQrySPBMInvestorPortfDef(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPortfMarginRatio => {
            let fld = param as *const CThostFtdcInvestorPortfMarginRatioField;
            return OnRspOptBox::OnRspQryInvestorPortfMarginRatio(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorProdSPBMDetail => {
            let fld = param as *const CThostFtdcInvestorProdSPBMDetailField;
            return OnRspOptBox::OnRspQryInvestorProdSPBMDetail(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorCommoditySPMMMargin => {
            let fld = param as *const CThostFtdcInvestorCommoditySPMMMarginField;
            return OnRspOptBox::OnRspQryInvestorCommoditySPMMMargin(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorCommodityGroupSPMMMargin => {
            let fld = param as *const CThostFtdcInvestorCommodityGroupSPMMMarginField;
            return OnRspOptBox::OnRspQryInvestorCommodityGroupSPMMMargin(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPMMInstParam => {
            let fld = param as *const CThostFtdcSPMMInstParamField;
            return OnRspOptBox::OnRspQrySPMMInstParam(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPMMProductParam => {
            let fld = param as *const CThostFtdcSPMMProductParamField;
            return OnRspOptBox::OnRspQrySPMMProductParam(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQrySPBMAddOnInterParameter => {
            let fld = param as *const CThostFtdcSPBMAddOnInterParameterField;
            return OnRspOptBox::OnRspQrySPBMAddOnInterParameter(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSCombProductInfo => {
            let fld = param as *const CThostFtdcRCAMSCombProductInfoField;
            return OnRspOptBox::OnRspQryRCAMSCombProductInfo(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSInstrParameter => {
            let fld = param as *const CThostFtdcRCAMSInstrParameterField;
            return OnRspOptBox::OnRspQryRCAMSInstrParameter(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSIntraParameter => {
            let fld = param as *const CThostFtdcRCAMSIntraParameterField;
            return OnRspOptBox::OnRspQryRCAMSIntraParameter(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSInterParameter => {
            let fld = param as *const CThostFtdcRCAMSInterParameterField;
            return OnRspOptBox::OnRspQryRCAMSInterParameter(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSShortOptAdjustParam => {
            let fld = param as *const CThostFtdcRCAMSShortOptAdjustParamField;
            return OnRspOptBox::OnRspQryRCAMSShortOptAdjustParam(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSInvestorCombPosition => {
            let fld = param as *const CThostFtdcRCAMSInvestorCombPositionField;
            return OnRspOptBox::OnRspQryRCAMSInvestorCombPosition(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorProdRCAMSMargin => {
            let fld = param as *const CThostFtdcInvestorProdRCAMSMarginField;
            return OnRspOptBox::OnRspQryInvestorProdRCAMSMargin(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRULEInstrParameter => {
            let fld = param as *const CThostFtdcRULEInstrParameterField;
            return OnRspOptBox::OnRspQryRULEInstrParameter(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRULEIntraParameter => {
            let fld = param as *const CThostFtdcRULEIntraParameterField;
            return OnRspOptBox::OnRspQryRULEIntraParameter(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryRULEInterParameter => {
            let fld = param as *const CThostFtdcRULEInterParameterField;
            return OnRspOptBox::OnRspQryRULEInterParameter(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorProdRULEMargin => {
            let fld = param as *const CThostFtdcInvestorProdRULEMarginField;
            return OnRspOptBox::OnRspQryInvestorProdRULEMargin(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPortfSetting => {
            let fld = param as *const CThostFtdcInvestorPortfSettingField;
            return OnRspOptBox::OnRspQryInvestorPortfSetting(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryInvestorInfoCommRec => {
            let fld = param as *const CThostFtdcInvestorInfoCommRecField;
            return OnRspOptBox::OnRspQryInvestorInfoCommRec(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryCombLeg => {
            let fld = param as *const CThostFtdcCombLegField;
            return OnRspOptBox::OnRspQryCombLeg(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspOffsetSetting => {
            let fld = param as *const CThostFtdcInputOffsetSettingField;
            return OnRspOptBox::OnRspOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspCancelOffsetSetting => {
            let fld = param as *const CThostFtdcInputOffsetSettingField;
            return OnRspOptBox::OnRspCancelOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRspEvent::OnRspQryOffsetSetting => {
            let fld = param as *const CThostFtdcOffsetSettingField;
            return OnRspOptBox::OnRspQryOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
    }
}
pub fn cvoid_to_rtn_box(evt: EnumOnRtnEvent, param: *const c_void) -> OnRtnOptBox {
    match evt {
        EnumOnRtnEvent::OnRtnDepthMarketData => {
            let fld = param as *const CThostFtdcDepthMarketDataField;
            return OnRtnOptBox::OnRtnDepthMarketData(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnForQuoteRsp => {
            let fld = param as *const CThostFtdcForQuoteRspField;
            return OnRtnOptBox::OnRtnForQuoteRsp(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnOrder => {
            let fld = param as *const CThostFtdcOrderField;
            return OnRtnOptBox::OnRtnOrder(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnTrade => {
            let fld = param as *const CThostFtdcTradeField;
            return OnRtnOptBox::OnRtnTrade(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnInstrumentStatus => {
            let fld = param as *const CThostFtdcInstrumentStatusField;
            return OnRtnOptBox::OnRtnInstrumentStatus(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnBulletin => {
            let fld = param as *const CThostFtdcBulletinField;
            return OnRtnOptBox::OnRtnBulletin(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnTradingNotice => {
            let fld = param as *const CThostFtdcTradingNoticeInfoField;
            return OnRtnOptBox::OnRtnTradingNotice(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnErrorConditionalOrder => {
            let fld = param as *const CThostFtdcErrorConditionalOrderField;
            return OnRtnOptBox::OnRtnErrorConditionalOrder(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnExecOrder => {
            let fld = param as *const CThostFtdcExecOrderField;
            return OnRtnOptBox::OnRtnExecOrder(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnQuote => {
            let fld = param as *const CThostFtdcQuoteField;
            return OnRtnOptBox::OnRtnQuote(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnCFMMCTradingAccountToken => {
            let fld = param as *const CThostFtdcCFMMCTradingAccountTokenField;
            return OnRtnOptBox::OnRtnCFMMCTradingAccountToken(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnOptionSelfClose => {
            let fld = param as *const CThostFtdcOptionSelfCloseField;
            return OnRtnOptBox::OnRtnOptionSelfClose(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnCombAction => {
            let fld = param as *const CThostFtdcCombActionField;
            return OnRtnOptBox::OnRtnCombAction(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnFromBankToFutureByBank => {
            let fld = param as *const CThostFtdcRspTransferField;
            return OnRtnOptBox::OnRtnFromBankToFutureByBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnFromFutureToBankByBank => {
            let fld = param as *const CThostFtdcRspTransferField;
            return OnRtnOptBox::OnRtnFromFutureToBankByBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromBankToFutureByBank => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptBox::OnRtnRepealFromBankToFutureByBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromFutureToBankByBank => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptBox::OnRtnRepealFromFutureToBankByBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnFromBankToFutureByFuture => {
            let fld = param as *const CThostFtdcRspTransferField;
            return OnRtnOptBox::OnRtnFromBankToFutureByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnFromFutureToBankByFuture => {
            let fld = param as *const CThostFtdcRspTransferField;
            return OnRtnOptBox::OnRtnFromFutureToBankByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromBankToFutureByFutureManual => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptBox::OnRtnRepealFromBankToFutureByFutureManual(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromFutureToBankByFutureManual => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptBox::OnRtnRepealFromFutureToBankByFutureManual(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnQueryBankBalanceByFuture => {
            let fld = param as *const CThostFtdcNotifyQueryAccountField;
            return OnRtnOptBox::OnRtnQueryBankBalanceByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromBankToFutureByFuture => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptBox::OnRtnRepealFromBankToFutureByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromFutureToBankByFuture => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptBox::OnRtnRepealFromFutureToBankByFuture(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnOpenAccountByBank => {
            let fld = param as *const CThostFtdcOpenAccountField;
            return OnRtnOptBox::OnRtnOpenAccountByBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnCancelAccountByBank => {
            let fld = param as *const CThostFtdcCancelAccountField;
            return OnRtnOptBox::OnRtnCancelAccountByBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnChangeAccountByBank => {
            let fld = param as *const CThostFtdcChangeAccountField;
            return OnRtnOptBox::OnRtnChangeAccountByBank(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
        EnumOnRtnEvent::OnRtnOffsetSetting => {
            let fld = param as *const CThostFtdcOffsetSettingField;
            return OnRtnOptBox::OnRtnOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(Box::new(unsafe { (*fld).clone() }))
            });
        }
    }
}
