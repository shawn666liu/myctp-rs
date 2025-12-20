use crate::ctp::*;
use std::ffi::c_void;

// 除了提供Box的版本，用户也可以在CTP返回的引用上直接处理，提供更高的性能和灵活性，但是引用不能跨线程传递

#[derive(Debug, Clone)]
pub enum OnErrRtnOptRef<'a> {
    OnErrRtnOrderInsert(Option<&'a CThostFtdcInputOrderField>),
    OnErrRtnOrderAction(Option<&'a CThostFtdcOrderActionField>),
    OnErrRtnExecOrderInsert(Option<&'a CThostFtdcInputExecOrderField>),
    OnErrRtnExecOrderAction(Option<&'a CThostFtdcExecOrderActionField>),
    OnErrRtnForQuoteInsert(Option<&'a CThostFtdcInputForQuoteField>),
    OnErrRtnQuoteInsert(Option<&'a CThostFtdcInputQuoteField>),
    OnErrRtnQuoteAction(Option<&'a CThostFtdcQuoteActionField>),
    OnErrRtnBatchOrderAction(Option<&'a CThostFtdcBatchOrderActionField>),
    OnErrRtnOptionSelfCloseInsert(Option<&'a CThostFtdcInputOptionSelfCloseField>),
    OnErrRtnOptionSelfCloseAction(Option<&'a CThostFtdcOptionSelfCloseActionField>),
    OnErrRtnCombActionInsert(Option<&'a CThostFtdcInputCombActionField>),
    OnErrRtnBankToFutureByFuture(Option<&'a CThostFtdcReqTransferField>),
    OnErrRtnFutureToBankByFuture(Option<&'a CThostFtdcReqTransferField>),
    OnErrRtnRepealBankToFutureByFutureManual(Option<&'a CThostFtdcReqRepealField>),
    OnErrRtnRepealFutureToBankByFutureManual(Option<&'a CThostFtdcReqRepealField>),
    OnErrRtnQueryBankBalanceByFuture(Option<&'a CThostFtdcReqQueryAccountField>),
    OnErrRtnOffsetSetting(Option<&'a CThostFtdcInputOffsetSettingField>),
    OnErrRtnCancelOffsetSetting(Option<&'a CThostFtdcCancelOffsetSettingField>),
}

#[derive(Debug, Clone)]
pub enum OnRspOptRef<'a> {
    OnRspUserLogin(Option<&'a CThostFtdcRspUserLoginField>),
    OnRspUserLogout(Option<&'a CThostFtdcUserLogoutField>),
    OnRspQryMulticastInstrument(Option<&'a CThostFtdcMulticastInstrumentField>),
    OnRspError(Option<&'a CThostFtdcRspInfoField>),
    OnRspSubMarketData(Option<&'a CThostFtdcSpecificInstrumentField>),
    OnRspUnSubMarketData(Option<&'a CThostFtdcSpecificInstrumentField>),
    OnRspSubForQuoteRsp(Option<&'a CThostFtdcSpecificInstrumentField>),
    OnRspUnSubForQuoteRsp(Option<&'a CThostFtdcSpecificInstrumentField>),
    OnRspAuthenticate(Option<&'a CThostFtdcRspAuthenticateField>),
    OnRspUserPasswordUpdate(Option<&'a CThostFtdcUserPasswordUpdateField>),
    OnRspTradingAccountPasswordUpdate(Option<&'a CThostFtdcTradingAccountPasswordUpdateField>),
    OnRspUserAuthMethod(Option<&'a CThostFtdcRspUserAuthMethodField>),
    OnRspGenUserCaptcha(Option<&'a CThostFtdcRspGenUserCaptchaField>),
    OnRspGenUserText(Option<&'a CThostFtdcRspGenUserTextField>),
    OnRspOrderInsert(Option<&'a CThostFtdcInputOrderField>),
    OnRspParkedOrderInsert(Option<&'a CThostFtdcParkedOrderField>),
    OnRspParkedOrderAction(Option<&'a CThostFtdcParkedOrderActionField>),
    OnRspOrderAction(Option<&'a CThostFtdcInputOrderActionField>),
    OnRspQryMaxOrderVolume(Option<&'a CThostFtdcQryMaxOrderVolumeField>),
    OnRspSettlementInfoConfirm(Option<&'a CThostFtdcSettlementInfoConfirmField>),
    OnRspRemoveParkedOrder(Option<&'a CThostFtdcRemoveParkedOrderField>),
    OnRspRemoveParkedOrderAction(Option<&'a CThostFtdcRemoveParkedOrderActionField>),
    OnRspExecOrderInsert(Option<&'a CThostFtdcInputExecOrderField>),
    OnRspExecOrderAction(Option<&'a CThostFtdcInputExecOrderActionField>),
    OnRspForQuoteInsert(Option<&'a CThostFtdcInputForQuoteField>),
    OnRspQuoteInsert(Option<&'a CThostFtdcInputQuoteField>),
    OnRspQuoteAction(Option<&'a CThostFtdcInputQuoteActionField>),
    OnRspBatchOrderAction(Option<&'a CThostFtdcInputBatchOrderActionField>),
    OnRspOptionSelfCloseInsert(Option<&'a CThostFtdcInputOptionSelfCloseField>),
    OnRspOptionSelfCloseAction(Option<&'a CThostFtdcInputOptionSelfCloseActionField>),
    OnRspCombActionInsert(Option<&'a CThostFtdcInputCombActionField>),
    OnRspQryOrder(Option<&'a CThostFtdcOrderField>),
    OnRspQryTrade(Option<&'a CThostFtdcTradeField>),
    OnRspQryInvestorPosition(Option<&'a CThostFtdcInvestorPositionField>),
    OnRspQryTradingAccount(Option<&'a CThostFtdcTradingAccountField>),
    OnRspQryInvestor(Option<&'a CThostFtdcInvestorField>),
    OnRspQryTradingCode(Option<&'a CThostFtdcTradingCodeField>),
    OnRspQryInstrumentMarginRate(Option<&'a CThostFtdcInstrumentMarginRateField>),
    OnRspQryInstrumentCommissionRate(Option<&'a CThostFtdcInstrumentCommissionRateField>),
    OnRspQryUserSession(Option<&'a CThostFtdcUserSessionField>),
    OnRspQryExchange(Option<&'a CThostFtdcExchangeField>),
    OnRspQryProduct(Option<&'a CThostFtdcProductField>),
    OnRspQryInstrument(Option<&'a CThostFtdcInstrumentField>),
    OnRspQryDepthMarketData(Option<&'a CThostFtdcDepthMarketDataField>),
    OnRspQryTraderOffer(Option<&'a CThostFtdcTraderOfferField>),
    OnRspQrySettlementInfo(Option<&'a CThostFtdcSettlementInfoField>),
    OnRspQryTransferBank(Option<&'a CThostFtdcTransferBankField>),
    OnRspQryInvestorPositionDetail(Option<&'a CThostFtdcInvestorPositionDetailField>),
    OnRspQryNotice(Option<&'a CThostFtdcNoticeField>),
    OnRspQrySettlementInfoConfirm(Option<&'a CThostFtdcSettlementInfoConfirmField>),
    OnRspQryInvestorPositionCombineDetail(Option<&'a CThostFtdcInvestorPositionCombineDetailField>),
    OnRspQryCFMMCTradingAccountKey(Option<&'a CThostFtdcCFMMCTradingAccountKeyField>),
    OnRspQryEWarrantOffset(Option<&'a CThostFtdcEWarrantOffsetField>),
    OnRspQryInvestorProductGroupMargin(Option<&'a CThostFtdcInvestorProductGroupMarginField>),
    OnRspQryExchangeMarginRate(Option<&'a CThostFtdcExchangeMarginRateField>),
    OnRspQryExchangeMarginRateAdjust(Option<&'a CThostFtdcExchangeMarginRateAdjustField>),
    OnRspQryExchangeRate(Option<&'a CThostFtdcExchangeRateField>),
    OnRspQrySecAgentACIDMap(Option<&'a CThostFtdcSecAgentACIDMapField>),
    OnRspQryProductExchRate(Option<&'a CThostFtdcProductExchRateField>),
    OnRspQryProductGroup(Option<&'a CThostFtdcProductGroupField>),
    OnRspQryMMInstrumentCommissionRate(Option<&'a CThostFtdcMMInstrumentCommissionRateField>),
    OnRspQryMMOptionInstrCommRate(Option<&'a CThostFtdcMMOptionInstrCommRateField>),
    OnRspQryInstrumentOrderCommRate(Option<&'a CThostFtdcInstrumentOrderCommRateField>),
    OnRspQrySecAgentTradingAccount(Option<&'a CThostFtdcTradingAccountField>),
    OnRspQrySecAgentCheckMode(Option<&'a CThostFtdcSecAgentCheckModeField>),
    OnRspQrySecAgentTradeInfo(Option<&'a CThostFtdcSecAgentTradeInfoField>),
    OnRspQryOptionInstrTradeCost(Option<&'a CThostFtdcOptionInstrTradeCostField>),
    OnRspQryOptionInstrCommRate(Option<&'a CThostFtdcOptionInstrCommRateField>),
    OnRspQryExecOrder(Option<&'a CThostFtdcExecOrderField>),
    OnRspQryForQuote(Option<&'a CThostFtdcForQuoteField>),
    OnRspQryQuote(Option<&'a CThostFtdcQuoteField>),
    OnRspQryOptionSelfClose(Option<&'a CThostFtdcOptionSelfCloseField>),
    OnRspQryInvestUnit(Option<&'a CThostFtdcInvestUnitField>),
    OnRspQryCombInstrumentGuard(Option<&'a CThostFtdcCombInstrumentGuardField>),
    OnRspQryCombAction(Option<&'a CThostFtdcCombActionField>),
    OnRspQryTransferSerial(Option<&'a CThostFtdcTransferSerialField>),
    OnRspQryAccountregister(Option<&'a CThostFtdcAccountregisterField>),
    OnRspQryContractBank(Option<&'a CThostFtdcContractBankField>),
    OnRspQryParkedOrder(Option<&'a CThostFtdcParkedOrderField>),
    OnRspQryParkedOrderAction(Option<&'a CThostFtdcParkedOrderActionField>),
    OnRspQryTradingNotice(Option<&'a CThostFtdcTradingNoticeField>),
    OnRspQryBrokerTradingParams(Option<&'a CThostFtdcBrokerTradingParamsField>),
    OnRspQryBrokerTradingAlgos(Option<&'a CThostFtdcBrokerTradingAlgosField>),
    OnRspQueryCFMMCTradingAccountToken(Option<&'a CThostFtdcQueryCFMMCTradingAccountTokenField>),
    OnRspFromBankToFutureByFuture(Option<&'a CThostFtdcReqTransferField>),
    OnRspFromFutureToBankByFuture(Option<&'a CThostFtdcReqTransferField>),
    OnRspQueryBankAccountMoneyByFuture(Option<&'a CThostFtdcReqQueryAccountField>),
    OnRspQryClassifiedInstrument(Option<&'a CThostFtdcInstrumentField>),
    OnRspQryCombPromotionParam(Option<&'a CThostFtdcCombPromotionParamField>),
    OnRspQryRiskSettleInvstPosition(Option<&'a CThostFtdcRiskSettleInvstPositionField>),
    OnRspQryRiskSettleProductStatus(Option<&'a CThostFtdcRiskSettleProductStatusField>),
    OnRspQrySPBMFutureParameter(Option<&'a CThostFtdcSPBMFutureParameterField>),
    OnRspQrySPBMOptionParameter(Option<&'a CThostFtdcSPBMOptionParameterField>),
    OnRspQrySPBMIntraParameter(Option<&'a CThostFtdcSPBMIntraParameterField>),
    OnRspQrySPBMInterParameter(Option<&'a CThostFtdcSPBMInterParameterField>),
    OnRspQrySPBMPortfDefinition(Option<&'a CThostFtdcSPBMPortfDefinitionField>),
    OnRspQrySPBMInvestorPortfDef(Option<&'a CThostFtdcSPBMInvestorPortfDefField>),
    OnRspQryInvestorPortfMarginRatio(Option<&'a CThostFtdcInvestorPortfMarginRatioField>),
    OnRspQryInvestorProdSPBMDetail(Option<&'a CThostFtdcInvestorProdSPBMDetailField>),
    OnRspQryInvestorCommoditySPMMMargin(Option<&'a CThostFtdcInvestorCommoditySPMMMarginField>),
    OnRspQryInvestorCommodityGroupSPMMMargin(
        Option<&'a CThostFtdcInvestorCommodityGroupSPMMMarginField>,
    ),
    OnRspQrySPMMInstParam(Option<&'a CThostFtdcSPMMInstParamField>),
    OnRspQrySPMMProductParam(Option<&'a CThostFtdcSPMMProductParamField>),
    OnRspQrySPBMAddOnInterParameter(Option<&'a CThostFtdcSPBMAddOnInterParameterField>),
    OnRspQryRCAMSCombProductInfo(Option<&'a CThostFtdcRCAMSCombProductInfoField>),
    OnRspQryRCAMSInstrParameter(Option<&'a CThostFtdcRCAMSInstrParameterField>),
    OnRspQryRCAMSIntraParameter(Option<&'a CThostFtdcRCAMSIntraParameterField>),
    OnRspQryRCAMSInterParameter(Option<&'a CThostFtdcRCAMSInterParameterField>),
    OnRspQryRCAMSShortOptAdjustParam(Option<&'a CThostFtdcRCAMSShortOptAdjustParamField>),
    OnRspQryRCAMSInvestorCombPosition(Option<&'a CThostFtdcRCAMSInvestorCombPositionField>),
    OnRspQryInvestorProdRCAMSMargin(Option<&'a CThostFtdcInvestorProdRCAMSMarginField>),
    OnRspQryRULEInstrParameter(Option<&'a CThostFtdcRULEInstrParameterField>),
    OnRspQryRULEIntraParameter(Option<&'a CThostFtdcRULEIntraParameterField>),
    OnRspQryRULEInterParameter(Option<&'a CThostFtdcRULEInterParameterField>),
    OnRspQryInvestorProdRULEMargin(Option<&'a CThostFtdcInvestorProdRULEMarginField>),
    OnRspQryInvestorPortfSetting(Option<&'a CThostFtdcInvestorPortfSettingField>),
    OnRspQryInvestorInfoCommRec(Option<&'a CThostFtdcInvestorInfoCommRecField>),
    OnRspQryCombLeg(Option<&'a CThostFtdcCombLegField>),
    OnRspOffsetSetting(Option<&'a CThostFtdcInputOffsetSettingField>),
    OnRspCancelOffsetSetting(Option<&'a CThostFtdcInputOffsetSettingField>),
    OnRspQryOffsetSetting(Option<&'a CThostFtdcOffsetSettingField>),
}

#[derive(Debug, Clone)]
pub enum OnRtnOptRef<'a> {
    OnRtnDepthMarketData(Option<&'a CThostFtdcDepthMarketDataField>),
    OnRtnForQuoteRsp(Option<&'a CThostFtdcForQuoteRspField>),
    OnRtnOrder(Option<&'a CThostFtdcOrderField>),
    OnRtnTrade(Option<&'a CThostFtdcTradeField>),
    OnRtnInstrumentStatus(Option<&'a CThostFtdcInstrumentStatusField>),
    OnRtnBulletin(Option<&'a CThostFtdcBulletinField>),
    OnRtnTradingNotice(Option<&'a CThostFtdcTradingNoticeInfoField>),
    OnRtnErrorConditionalOrder(Option<&'a CThostFtdcErrorConditionalOrderField>),
    OnRtnExecOrder(Option<&'a CThostFtdcExecOrderField>),
    OnRtnQuote(Option<&'a CThostFtdcQuoteField>),
    OnRtnCFMMCTradingAccountToken(Option<&'a CThostFtdcCFMMCTradingAccountTokenField>),
    OnRtnOptionSelfClose(Option<&'a CThostFtdcOptionSelfCloseField>),
    OnRtnCombAction(Option<&'a CThostFtdcCombActionField>),
    OnRtnFromBankToFutureByBank(Option<&'a CThostFtdcRspTransferField>),
    OnRtnFromFutureToBankByBank(Option<&'a CThostFtdcRspTransferField>),
    OnRtnRepealFromBankToFutureByBank(Option<&'a CThostFtdcRspRepealField>),
    OnRtnRepealFromFutureToBankByBank(Option<&'a CThostFtdcRspRepealField>),
    OnRtnFromBankToFutureByFuture(Option<&'a CThostFtdcRspTransferField>),
    OnRtnFromFutureToBankByFuture(Option<&'a CThostFtdcRspTransferField>),
    OnRtnRepealFromBankToFutureByFutureManual(Option<&'a CThostFtdcRspRepealField>),
    OnRtnRepealFromFutureToBankByFutureManual(Option<&'a CThostFtdcRspRepealField>),
    OnRtnQueryBankBalanceByFuture(Option<&'a CThostFtdcNotifyQueryAccountField>),
    OnRtnRepealFromBankToFutureByFuture(Option<&'a CThostFtdcRspRepealField>),
    OnRtnRepealFromFutureToBankByFuture(Option<&'a CThostFtdcRspRepealField>),
    OnRtnOpenAccountByBank(Option<&'a CThostFtdcOpenAccountField>),
    OnRtnCancelAccountByBank(Option<&'a CThostFtdcCancelAccountField>),
    OnRtnChangeAccountByBank(Option<&'a CThostFtdcChangeAccountField>),
    OnRtnOffsetSetting(Option<&'a CThostFtdcOffsetSettingField>),
}

pub fn cvoid_to_errrtn_ref<'a>(evt: EnumOnErrRtnEvent, param: *const c_void) -> OnErrRtnOptRef<'a> {
    match evt {
        EnumOnErrRtnEvent::OnErrRtnOrderInsert => {
            let fld = param as *const CThostFtdcInputOrderField;
            return OnErrRtnOptRef::OnErrRtnOrderInsert(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnOrderAction => {
            let fld = param as *const CThostFtdcOrderActionField;
            return OnErrRtnOptRef::OnErrRtnOrderAction(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnExecOrderInsert => {
            let fld = param as *const CThostFtdcInputExecOrderField;
            return OnErrRtnOptRef::OnErrRtnExecOrderInsert(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnExecOrderAction => {
            let fld = param as *const CThostFtdcExecOrderActionField;
            return OnErrRtnOptRef::OnErrRtnExecOrderAction(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnForQuoteInsert => {
            let fld = param as *const CThostFtdcInputForQuoteField;
            return OnErrRtnOptRef::OnErrRtnForQuoteInsert(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnQuoteInsert => {
            let fld = param as *const CThostFtdcInputQuoteField;
            return OnErrRtnOptRef::OnErrRtnQuoteInsert(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnQuoteAction => {
            let fld = param as *const CThostFtdcQuoteActionField;
            return OnErrRtnOptRef::OnErrRtnQuoteAction(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnBatchOrderAction => {
            let fld = param as *const CThostFtdcBatchOrderActionField;
            return OnErrRtnOptRef::OnErrRtnBatchOrderAction(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnOptionSelfCloseInsert => {
            let fld = param as *const CThostFtdcInputOptionSelfCloseField;
            return OnErrRtnOptRef::OnErrRtnOptionSelfCloseInsert(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnOptionSelfCloseAction => {
            let fld = param as *const CThostFtdcOptionSelfCloseActionField;
            return OnErrRtnOptRef::OnErrRtnOptionSelfCloseAction(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnCombActionInsert => {
            let fld = param as *const CThostFtdcInputCombActionField;
            return OnErrRtnOptRef::OnErrRtnCombActionInsert(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnBankToFutureByFuture => {
            let fld = param as *const CThostFtdcReqTransferField;
            return OnErrRtnOptRef::OnErrRtnBankToFutureByFuture(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnFutureToBankByFuture => {
            let fld = param as *const CThostFtdcReqTransferField;
            return OnErrRtnOptRef::OnErrRtnFutureToBankByFuture(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnRepealBankToFutureByFutureManual => {
            let fld = param as *const CThostFtdcReqRepealField;
            return OnErrRtnOptRef::OnErrRtnRepealBankToFutureByFutureManual(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnRepealFutureToBankByFutureManual => {
            let fld = param as *const CThostFtdcReqRepealField;
            return OnErrRtnOptRef::OnErrRtnRepealFutureToBankByFutureManual(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnQueryBankBalanceByFuture => {
            let fld = param as *const CThostFtdcReqQueryAccountField;
            return OnErrRtnOptRef::OnErrRtnQueryBankBalanceByFuture(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnOffsetSetting => {
            let fld = param as *const CThostFtdcInputOffsetSettingField;
            return OnErrRtnOptRef::OnErrRtnOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnErrRtnEvent::OnErrRtnCancelOffsetSetting => {
            let fld = param as *const CThostFtdcCancelOffsetSettingField;
            return OnErrRtnOptRef::OnErrRtnCancelOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
    }
}
pub fn cvoid_to_rsp_ref<'a>(evt: EnumOnRspEvent, param: *const c_void) -> OnRspOptRef<'a> {
    match evt {
        EnumOnRspEvent::OnRspUserLogin => {
            let fld = param as *const CThostFtdcRspUserLoginField;
            return OnRspOptRef::OnRspUserLogin(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspUserLogout => {
            let fld = param as *const CThostFtdcUserLogoutField;
            return OnRspOptRef::OnRspUserLogout(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryMulticastInstrument => {
            let fld = param as *const CThostFtdcMulticastInstrumentField;
            return OnRspOptRef::OnRspQryMulticastInstrument(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspError => {
            let fld = param as *const CThostFtdcRspInfoField;
            return OnRspOptRef::OnRspError(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspSubMarketData => {
            let fld = param as *const CThostFtdcSpecificInstrumentField;
            return OnRspOptRef::OnRspSubMarketData(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspUnSubMarketData => {
            let fld = param as *const CThostFtdcSpecificInstrumentField;
            return OnRspOptRef::OnRspUnSubMarketData(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspSubForQuoteRsp => {
            let fld = param as *const CThostFtdcSpecificInstrumentField;
            return OnRspOptRef::OnRspSubForQuoteRsp(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspUnSubForQuoteRsp => {
            let fld = param as *const CThostFtdcSpecificInstrumentField;
            return OnRspOptRef::OnRspUnSubForQuoteRsp(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspAuthenticate => {
            let fld = param as *const CThostFtdcRspAuthenticateField;
            return OnRspOptRef::OnRspAuthenticate(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspUserPasswordUpdate => {
            let fld = param as *const CThostFtdcUserPasswordUpdateField;
            return OnRspOptRef::OnRspUserPasswordUpdate(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspTradingAccountPasswordUpdate => {
            let fld = param as *const CThostFtdcTradingAccountPasswordUpdateField;
            return OnRspOptRef::OnRspTradingAccountPasswordUpdate(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspUserAuthMethod => {
            let fld = param as *const CThostFtdcRspUserAuthMethodField;
            return OnRspOptRef::OnRspUserAuthMethod(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspGenUserCaptcha => {
            let fld = param as *const CThostFtdcRspGenUserCaptchaField;
            return OnRspOptRef::OnRspGenUserCaptcha(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspGenUserText => {
            let fld = param as *const CThostFtdcRspGenUserTextField;
            return OnRspOptRef::OnRspGenUserText(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspOrderInsert => {
            let fld = param as *const CThostFtdcInputOrderField;
            return OnRspOptRef::OnRspOrderInsert(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspParkedOrderInsert => {
            let fld = param as *const CThostFtdcParkedOrderField;
            return OnRspOptRef::OnRspParkedOrderInsert(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspParkedOrderAction => {
            let fld = param as *const CThostFtdcParkedOrderActionField;
            return OnRspOptRef::OnRspParkedOrderAction(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspOrderAction => {
            let fld = param as *const CThostFtdcInputOrderActionField;
            return OnRspOptRef::OnRspOrderAction(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryMaxOrderVolume => {
            let fld = param as *const CThostFtdcQryMaxOrderVolumeField;
            return OnRspOptRef::OnRspQryMaxOrderVolume(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspSettlementInfoConfirm => {
            let fld = param as *const CThostFtdcSettlementInfoConfirmField;
            return OnRspOptRef::OnRspSettlementInfoConfirm(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspRemoveParkedOrder => {
            let fld = param as *const CThostFtdcRemoveParkedOrderField;
            return OnRspOptRef::OnRspRemoveParkedOrder(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspRemoveParkedOrderAction => {
            let fld = param as *const CThostFtdcRemoveParkedOrderActionField;
            return OnRspOptRef::OnRspRemoveParkedOrderAction(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspExecOrderInsert => {
            let fld = param as *const CThostFtdcInputExecOrderField;
            return OnRspOptRef::OnRspExecOrderInsert(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspExecOrderAction => {
            let fld = param as *const CThostFtdcInputExecOrderActionField;
            return OnRspOptRef::OnRspExecOrderAction(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspForQuoteInsert => {
            let fld = param as *const CThostFtdcInputForQuoteField;
            return OnRspOptRef::OnRspForQuoteInsert(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQuoteInsert => {
            let fld = param as *const CThostFtdcInputQuoteField;
            return OnRspOptRef::OnRspQuoteInsert(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQuoteAction => {
            let fld = param as *const CThostFtdcInputQuoteActionField;
            return OnRspOptRef::OnRspQuoteAction(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspBatchOrderAction => {
            let fld = param as *const CThostFtdcInputBatchOrderActionField;
            return OnRspOptRef::OnRspBatchOrderAction(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspOptionSelfCloseInsert => {
            let fld = param as *const CThostFtdcInputOptionSelfCloseField;
            return OnRspOptRef::OnRspOptionSelfCloseInsert(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspOptionSelfCloseAction => {
            let fld = param as *const CThostFtdcInputOptionSelfCloseActionField;
            return OnRspOptRef::OnRspOptionSelfCloseAction(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspCombActionInsert => {
            let fld = param as *const CThostFtdcInputCombActionField;
            return OnRspOptRef::OnRspCombActionInsert(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryOrder => {
            let fld = param as *const CThostFtdcOrderField;
            return OnRspOptRef::OnRspQryOrder(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryTrade => {
            let fld = param as *const CThostFtdcTradeField;
            return OnRspOptRef::OnRspQryTrade(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPosition => {
            let fld = param as *const CThostFtdcInvestorPositionField;
            return OnRspOptRef::OnRspQryInvestorPosition(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryTradingAccount => {
            let fld = param as *const CThostFtdcTradingAccountField;
            return OnRspOptRef::OnRspQryTradingAccount(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInvestor => {
            let fld = param as *const CThostFtdcInvestorField;
            return OnRspOptRef::OnRspQryInvestor(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryTradingCode => {
            let fld = param as *const CThostFtdcTradingCodeField;
            return OnRspOptRef::OnRspQryTradingCode(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInstrumentMarginRate => {
            let fld = param as *const CThostFtdcInstrumentMarginRateField;
            return OnRspOptRef::OnRspQryInstrumentMarginRate(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInstrumentCommissionRate => {
            let fld = param as *const CThostFtdcInstrumentCommissionRateField;
            return OnRspOptRef::OnRspQryInstrumentCommissionRate(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryUserSession => {
            let fld = param as *const CThostFtdcUserSessionField;
            return OnRspOptRef::OnRspQryUserSession(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryExchange => {
            let fld = param as *const CThostFtdcExchangeField;
            return OnRspOptRef::OnRspQryExchange(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryProduct => {
            let fld = param as *const CThostFtdcProductField;
            return OnRspOptRef::OnRspQryProduct(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInstrument => {
            let fld = param as *const CThostFtdcInstrumentField;
            return OnRspOptRef::OnRspQryInstrument(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryDepthMarketData => {
            let fld = param as *const CThostFtdcDepthMarketDataField;
            return OnRspOptRef::OnRspQryDepthMarketData(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryTraderOffer => {
            let fld = param as *const CThostFtdcTraderOfferField;
            return OnRspOptRef::OnRspQryTraderOffer(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQrySettlementInfo => {
            let fld = param as *const CThostFtdcSettlementInfoField;
            return OnRspOptRef::OnRspQrySettlementInfo(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryTransferBank => {
            let fld = param as *const CThostFtdcTransferBankField;
            return OnRspOptRef::OnRspQryTransferBank(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPositionDetail => {
            let fld = param as *const CThostFtdcInvestorPositionDetailField;
            return OnRspOptRef::OnRspQryInvestorPositionDetail(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryNotice => {
            let fld = param as *const CThostFtdcNoticeField;
            return OnRspOptRef::OnRspQryNotice(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQrySettlementInfoConfirm => {
            let fld = param as *const CThostFtdcSettlementInfoConfirmField;
            return OnRspOptRef::OnRspQrySettlementInfoConfirm(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPositionCombineDetail => {
            let fld = param as *const CThostFtdcInvestorPositionCombineDetailField;
            return OnRspOptRef::OnRspQryInvestorPositionCombineDetail(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryCFMMCTradingAccountKey => {
            let fld = param as *const CThostFtdcCFMMCTradingAccountKeyField;
            return OnRspOptRef::OnRspQryCFMMCTradingAccountKey(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryEWarrantOffset => {
            let fld = param as *const CThostFtdcEWarrantOffsetField;
            return OnRspOptRef::OnRspQryEWarrantOffset(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInvestorProductGroupMargin => {
            let fld = param as *const CThostFtdcInvestorProductGroupMarginField;
            return OnRspOptRef::OnRspQryInvestorProductGroupMargin(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryExchangeMarginRate => {
            let fld = param as *const CThostFtdcExchangeMarginRateField;
            return OnRspOptRef::OnRspQryExchangeMarginRate(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryExchangeMarginRateAdjust => {
            let fld = param as *const CThostFtdcExchangeMarginRateAdjustField;
            return OnRspOptRef::OnRspQryExchangeMarginRateAdjust(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryExchangeRate => {
            let fld = param as *const CThostFtdcExchangeRateField;
            return OnRspOptRef::OnRspQryExchangeRate(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQrySecAgentACIDMap => {
            let fld = param as *const CThostFtdcSecAgentACIDMapField;
            return OnRspOptRef::OnRspQrySecAgentACIDMap(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryProductExchRate => {
            let fld = param as *const CThostFtdcProductExchRateField;
            return OnRspOptRef::OnRspQryProductExchRate(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryProductGroup => {
            let fld = param as *const CThostFtdcProductGroupField;
            return OnRspOptRef::OnRspQryProductGroup(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryMMInstrumentCommissionRate => {
            let fld = param as *const CThostFtdcMMInstrumentCommissionRateField;
            return OnRspOptRef::OnRspQryMMInstrumentCommissionRate(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryMMOptionInstrCommRate => {
            let fld = param as *const CThostFtdcMMOptionInstrCommRateField;
            return OnRspOptRef::OnRspQryMMOptionInstrCommRate(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInstrumentOrderCommRate => {
            let fld = param as *const CThostFtdcInstrumentOrderCommRateField;
            return OnRspOptRef::OnRspQryInstrumentOrderCommRate(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQrySecAgentTradingAccount => {
            let fld = param as *const CThostFtdcTradingAccountField;
            return OnRspOptRef::OnRspQrySecAgentTradingAccount(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQrySecAgentCheckMode => {
            let fld = param as *const CThostFtdcSecAgentCheckModeField;
            return OnRspOptRef::OnRspQrySecAgentCheckMode(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQrySecAgentTradeInfo => {
            let fld = param as *const CThostFtdcSecAgentTradeInfoField;
            return OnRspOptRef::OnRspQrySecAgentTradeInfo(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryOptionInstrTradeCost => {
            let fld = param as *const CThostFtdcOptionInstrTradeCostField;
            return OnRspOptRef::OnRspQryOptionInstrTradeCost(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryOptionInstrCommRate => {
            let fld = param as *const CThostFtdcOptionInstrCommRateField;
            return OnRspOptRef::OnRspQryOptionInstrCommRate(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryExecOrder => {
            let fld = param as *const CThostFtdcExecOrderField;
            return OnRspOptRef::OnRspQryExecOrder(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryForQuote => {
            let fld = param as *const CThostFtdcForQuoteField;
            return OnRspOptRef::OnRspQryForQuote(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryQuote => {
            let fld = param as *const CThostFtdcQuoteField;
            return OnRspOptRef::OnRspQryQuote(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryOptionSelfClose => {
            let fld = param as *const CThostFtdcOptionSelfCloseField;
            return OnRspOptRef::OnRspQryOptionSelfClose(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInvestUnit => {
            let fld = param as *const CThostFtdcInvestUnitField;
            return OnRspOptRef::OnRspQryInvestUnit(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryCombInstrumentGuard => {
            let fld = param as *const CThostFtdcCombInstrumentGuardField;
            return OnRspOptRef::OnRspQryCombInstrumentGuard(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryCombAction => {
            let fld = param as *const CThostFtdcCombActionField;
            return OnRspOptRef::OnRspQryCombAction(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryTransferSerial => {
            let fld = param as *const CThostFtdcTransferSerialField;
            return OnRspOptRef::OnRspQryTransferSerial(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryAccountregister => {
            let fld = param as *const CThostFtdcAccountregisterField;
            return OnRspOptRef::OnRspQryAccountregister(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryContractBank => {
            let fld = param as *const CThostFtdcContractBankField;
            return OnRspOptRef::OnRspQryContractBank(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryParkedOrder => {
            let fld = param as *const CThostFtdcParkedOrderField;
            return OnRspOptRef::OnRspQryParkedOrder(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryParkedOrderAction => {
            let fld = param as *const CThostFtdcParkedOrderActionField;
            return OnRspOptRef::OnRspQryParkedOrderAction(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryTradingNotice => {
            let fld = param as *const CThostFtdcTradingNoticeField;
            return OnRspOptRef::OnRspQryTradingNotice(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryBrokerTradingParams => {
            let fld = param as *const CThostFtdcBrokerTradingParamsField;
            return OnRspOptRef::OnRspQryBrokerTradingParams(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryBrokerTradingAlgos => {
            let fld = param as *const CThostFtdcBrokerTradingAlgosField;
            return OnRspOptRef::OnRspQryBrokerTradingAlgos(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQueryCFMMCTradingAccountToken => {
            let fld = param as *const CThostFtdcQueryCFMMCTradingAccountTokenField;
            return OnRspOptRef::OnRspQueryCFMMCTradingAccountToken(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspFromBankToFutureByFuture => {
            let fld = param as *const CThostFtdcReqTransferField;
            return OnRspOptRef::OnRspFromBankToFutureByFuture(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspFromFutureToBankByFuture => {
            let fld = param as *const CThostFtdcReqTransferField;
            return OnRspOptRef::OnRspFromFutureToBankByFuture(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQueryBankAccountMoneyByFuture => {
            let fld = param as *const CThostFtdcReqQueryAccountField;
            return OnRspOptRef::OnRspQueryBankAccountMoneyByFuture(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryClassifiedInstrument => {
            let fld = param as *const CThostFtdcInstrumentField;
            return OnRspOptRef::OnRspQryClassifiedInstrument(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryCombPromotionParam => {
            let fld = param as *const CThostFtdcCombPromotionParamField;
            return OnRspOptRef::OnRspQryCombPromotionParam(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryRiskSettleInvstPosition => {
            let fld = param as *const CThostFtdcRiskSettleInvstPositionField;
            return OnRspOptRef::OnRspQryRiskSettleInvstPosition(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryRiskSettleProductStatus => {
            let fld = param as *const CThostFtdcRiskSettleProductStatusField;
            return OnRspOptRef::OnRspQryRiskSettleProductStatus(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQrySPBMFutureParameter => {
            let fld = param as *const CThostFtdcSPBMFutureParameterField;
            return OnRspOptRef::OnRspQrySPBMFutureParameter(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQrySPBMOptionParameter => {
            let fld = param as *const CThostFtdcSPBMOptionParameterField;
            return OnRspOptRef::OnRspQrySPBMOptionParameter(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQrySPBMIntraParameter => {
            let fld = param as *const CThostFtdcSPBMIntraParameterField;
            return OnRspOptRef::OnRspQrySPBMIntraParameter(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQrySPBMInterParameter => {
            let fld = param as *const CThostFtdcSPBMInterParameterField;
            return OnRspOptRef::OnRspQrySPBMInterParameter(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQrySPBMPortfDefinition => {
            let fld = param as *const CThostFtdcSPBMPortfDefinitionField;
            return OnRspOptRef::OnRspQrySPBMPortfDefinition(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQrySPBMInvestorPortfDef => {
            let fld = param as *const CThostFtdcSPBMInvestorPortfDefField;
            return OnRspOptRef::OnRspQrySPBMInvestorPortfDef(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPortfMarginRatio => {
            let fld = param as *const CThostFtdcInvestorPortfMarginRatioField;
            return OnRspOptRef::OnRspQryInvestorPortfMarginRatio(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInvestorProdSPBMDetail => {
            let fld = param as *const CThostFtdcInvestorProdSPBMDetailField;
            return OnRspOptRef::OnRspQryInvestorProdSPBMDetail(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInvestorCommoditySPMMMargin => {
            let fld = param as *const CThostFtdcInvestorCommoditySPMMMarginField;
            return OnRspOptRef::OnRspQryInvestorCommoditySPMMMargin(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInvestorCommodityGroupSPMMMargin => {
            let fld = param as *const CThostFtdcInvestorCommodityGroupSPMMMarginField;
            return OnRspOptRef::OnRspQryInvestorCommodityGroupSPMMMargin(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQrySPMMInstParam => {
            let fld = param as *const CThostFtdcSPMMInstParamField;
            return OnRspOptRef::OnRspQrySPMMInstParam(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQrySPMMProductParam => {
            let fld = param as *const CThostFtdcSPMMProductParamField;
            return OnRspOptRef::OnRspQrySPMMProductParam(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQrySPBMAddOnInterParameter => {
            let fld = param as *const CThostFtdcSPBMAddOnInterParameterField;
            return OnRspOptRef::OnRspQrySPBMAddOnInterParameter(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSCombProductInfo => {
            let fld = param as *const CThostFtdcRCAMSCombProductInfoField;
            return OnRspOptRef::OnRspQryRCAMSCombProductInfo(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSInstrParameter => {
            let fld = param as *const CThostFtdcRCAMSInstrParameterField;
            return OnRspOptRef::OnRspQryRCAMSInstrParameter(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSIntraParameter => {
            let fld = param as *const CThostFtdcRCAMSIntraParameterField;
            return OnRspOptRef::OnRspQryRCAMSIntraParameter(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSInterParameter => {
            let fld = param as *const CThostFtdcRCAMSInterParameterField;
            return OnRspOptRef::OnRspQryRCAMSInterParameter(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSShortOptAdjustParam => {
            let fld = param as *const CThostFtdcRCAMSShortOptAdjustParamField;
            return OnRspOptRef::OnRspQryRCAMSShortOptAdjustParam(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryRCAMSInvestorCombPosition => {
            let fld = param as *const CThostFtdcRCAMSInvestorCombPositionField;
            return OnRspOptRef::OnRspQryRCAMSInvestorCombPosition(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInvestorProdRCAMSMargin => {
            let fld = param as *const CThostFtdcInvestorProdRCAMSMarginField;
            return OnRspOptRef::OnRspQryInvestorProdRCAMSMargin(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryRULEInstrParameter => {
            let fld = param as *const CThostFtdcRULEInstrParameterField;
            return OnRspOptRef::OnRspQryRULEInstrParameter(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryRULEIntraParameter => {
            let fld = param as *const CThostFtdcRULEIntraParameterField;
            return OnRspOptRef::OnRspQryRULEIntraParameter(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryRULEInterParameter => {
            let fld = param as *const CThostFtdcRULEInterParameterField;
            return OnRspOptRef::OnRspQryRULEInterParameter(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInvestorProdRULEMargin => {
            let fld = param as *const CThostFtdcInvestorProdRULEMarginField;
            return OnRspOptRef::OnRspQryInvestorProdRULEMargin(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInvestorPortfSetting => {
            let fld = param as *const CThostFtdcInvestorPortfSettingField;
            return OnRspOptRef::OnRspQryInvestorPortfSetting(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryInvestorInfoCommRec => {
            let fld = param as *const CThostFtdcInvestorInfoCommRecField;
            return OnRspOptRef::OnRspQryInvestorInfoCommRec(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryCombLeg => {
            let fld = param as *const CThostFtdcCombLegField;
            return OnRspOptRef::OnRspQryCombLeg(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspOffsetSetting => {
            let fld = param as *const CThostFtdcInputOffsetSettingField;
            return OnRspOptRef::OnRspOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspCancelOffsetSetting => {
            let fld = param as *const CThostFtdcInputOffsetSettingField;
            return OnRspOptRef::OnRspCancelOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRspEvent::OnRspQryOffsetSetting => {
            let fld = param as *const CThostFtdcOffsetSettingField;
            return OnRspOptRef::OnRspQryOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
    }
}
pub fn cvoid_to_rtn_ref<'a>(evt: EnumOnRtnEvent, param: *const c_void) -> OnRtnOptRef<'a> {
    match evt {
        EnumOnRtnEvent::OnRtnDepthMarketData => {
            let fld = param as *const CThostFtdcDepthMarketDataField;
            return OnRtnOptRef::OnRtnDepthMarketData(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnForQuoteRsp => {
            let fld = param as *const CThostFtdcForQuoteRspField;
            return OnRtnOptRef::OnRtnForQuoteRsp(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnOrder => {
            let fld = param as *const CThostFtdcOrderField;
            return OnRtnOptRef::OnRtnOrder(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnTrade => {
            let fld = param as *const CThostFtdcTradeField;
            return OnRtnOptRef::OnRtnTrade(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnInstrumentStatus => {
            let fld = param as *const CThostFtdcInstrumentStatusField;
            return OnRtnOptRef::OnRtnInstrumentStatus(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnBulletin => {
            let fld = param as *const CThostFtdcBulletinField;
            return OnRtnOptRef::OnRtnBulletin(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnTradingNotice => {
            let fld = param as *const CThostFtdcTradingNoticeInfoField;
            return OnRtnOptRef::OnRtnTradingNotice(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnErrorConditionalOrder => {
            let fld = param as *const CThostFtdcErrorConditionalOrderField;
            return OnRtnOptRef::OnRtnErrorConditionalOrder(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnExecOrder => {
            let fld = param as *const CThostFtdcExecOrderField;
            return OnRtnOptRef::OnRtnExecOrder(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnQuote => {
            let fld = param as *const CThostFtdcQuoteField;
            return OnRtnOptRef::OnRtnQuote(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnCFMMCTradingAccountToken => {
            let fld = param as *const CThostFtdcCFMMCTradingAccountTokenField;
            return OnRtnOptRef::OnRtnCFMMCTradingAccountToken(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnOptionSelfClose => {
            let fld = param as *const CThostFtdcOptionSelfCloseField;
            return OnRtnOptRef::OnRtnOptionSelfClose(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnCombAction => {
            let fld = param as *const CThostFtdcCombActionField;
            return OnRtnOptRef::OnRtnCombAction(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnFromBankToFutureByBank => {
            let fld = param as *const CThostFtdcRspTransferField;
            return OnRtnOptRef::OnRtnFromBankToFutureByBank(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnFromFutureToBankByBank => {
            let fld = param as *const CThostFtdcRspTransferField;
            return OnRtnOptRef::OnRtnFromFutureToBankByBank(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromBankToFutureByBank => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptRef::OnRtnRepealFromBankToFutureByBank(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromFutureToBankByBank => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptRef::OnRtnRepealFromFutureToBankByBank(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnFromBankToFutureByFuture => {
            let fld = param as *const CThostFtdcRspTransferField;
            return OnRtnOptRef::OnRtnFromBankToFutureByFuture(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnFromFutureToBankByFuture => {
            let fld = param as *const CThostFtdcRspTransferField;
            return OnRtnOptRef::OnRtnFromFutureToBankByFuture(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromBankToFutureByFutureManual => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptRef::OnRtnRepealFromBankToFutureByFutureManual(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromFutureToBankByFutureManual => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptRef::OnRtnRepealFromFutureToBankByFutureManual(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnQueryBankBalanceByFuture => {
            let fld = param as *const CThostFtdcNotifyQueryAccountField;
            return OnRtnOptRef::OnRtnQueryBankBalanceByFuture(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromBankToFutureByFuture => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptRef::OnRtnRepealFromBankToFutureByFuture(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnRepealFromFutureToBankByFuture => {
            let fld = param as *const CThostFtdcRspRepealField;
            return OnRtnOptRef::OnRtnRepealFromFutureToBankByFuture(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnOpenAccountByBank => {
            let fld = param as *const CThostFtdcOpenAccountField;
            return OnRtnOptRef::OnRtnOpenAccountByBank(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnCancelAccountByBank => {
            let fld = param as *const CThostFtdcCancelAccountField;
            return OnRtnOptRef::OnRtnCancelAccountByBank(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnChangeAccountByBank => {
            let fld = param as *const CThostFtdcChangeAccountField;
            return OnRtnOptRef::OnRtnChangeAccountByBank(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
        EnumOnRtnEvent::OnRtnOffsetSetting => {
            let fld = param as *const CThostFtdcOffsetSettingField;
            return OnRtnOptRef::OnRtnOffsetSetting(if fld.is_null() {
                None
            } else {
                Some(unsafe { &*fld })
            });
        }
    }
}
