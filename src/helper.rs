use crate::ctp::*;

pub fn new_login(broker_id: &str, user_id: &str, password: &str) -> CThostFtdcReqUserLoginField {
    let mut f: CThostFtdcReqUserLoginField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, broker_id);
    set_cstr_from_str_truncate(&mut f.UserID, user_id);
    set_cstr_from_str_truncate(&mut f.Password, password);
    f
}

pub fn new_qry_settlement_info(
    broker_id: &str,
    investor_id: &str,
) -> CThostFtdcQrySettlementInfoField {
    let mut f: CThostFtdcQrySettlementInfoField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, broker_id);
    set_cstr_from_str_truncate(&mut f.InvestorID, investor_id);
    f
}

pub fn new_settlement_info_confirm(
    broker_id: &str,
    investor_id: &str,
) -> CThostFtdcSettlementInfoConfirmField {
    let mut f: CThostFtdcSettlementInfoConfirmField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, broker_id);
    set_cstr_from_str_truncate(&mut f.InvestorID, investor_id);
    f
}

pub fn new_qry_settlement_info_confirm(
    broker_id: &str,
    investor_id: &str,
) -> CThostFtdcQrySettlementInfoConfirmField {
    let mut f: CThostFtdcQrySettlementInfoConfirmField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, broker_id);
    set_cstr_from_str_truncate(&mut f.InvestorID, investor_id);
    f
}

pub fn new_qry_instrument(pattern: &str) -> CThostFtdcQryInstrumentField {
    let mut f: CThostFtdcQryInstrumentField = Default::default();
    set_cstr_from_str_truncate(&mut f.InstrumentID, pattern);
    f
}

pub fn new_qry_exchange(pattern: &str) -> CThostFtdcQryExchangeField {
    let mut f: CThostFtdcQryExchangeField = Default::default();
    set_cstr_from_str_truncate(&mut f.ExchangeID, pattern);
    f
}

pub fn new_qry_product(pattern: &str) -> CThostFtdcQryProductField {
    let mut f: CThostFtdcQryProductField = Default::default();
    set_cstr_from_str_truncate(&mut f.ProductID, pattern);
    f
}

pub fn new_qry_order(broker_id: &str, investor_id: &str) -> CThostFtdcQryOrderField {
    let mut f: CThostFtdcQryOrderField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, broker_id);
    set_cstr_from_str_truncate(&mut f.InvestorID, investor_id);
    f
}

pub fn new_qry_trade(broker_id: &str, investor_id: &str) -> CThostFtdcQryTradeField {
    let mut f: CThostFtdcQryTradeField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, broker_id);
    set_cstr_from_str_truncate(&mut f.InvestorID, investor_id);
    f
}

pub fn new_qry_investor_position(
    broker_id: &str,
    investor_id: &str,
) -> CThostFtdcQryInvestorPositionField {
    let mut f: CThostFtdcQryInvestorPositionField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, broker_id);
    set_cstr_from_str_truncate(&mut f.InvestorID, investor_id);
    f
}

pub fn new_qry_trading_account(
    broker_id: &str,
    investor_id: &str,
) -> CThostFtdcQryTradingAccountField {
    let mut f: CThostFtdcQryTradingAccountField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, broker_id);
    set_cstr_from_str_truncate(&mut f.InvestorID, investor_id);
    f
}

pub fn new_qry_investor(broker_id: &str, investor_id: &str) -> CThostFtdcQryInvestorField {
    let mut f: CThostFtdcQryInvestorField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, broker_id);
    set_cstr_from_str_truncate(&mut f.InvestorID, investor_id);
    f
}

pub fn new_qry_trading_code(broker_id: &str, investor_id: &str) -> CThostFtdcQryTradingCodeField {
    let mut f: CThostFtdcQryTradingCodeField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, broker_id);
    set_cstr_from_str_truncate(&mut f.InvestorID, investor_id);
    f
}

pub fn new_input_order(
    broker_id: &str,
    user_id: &str,
    investor_id: &str,
) -> CThostFtdcInputOrderField {
    let mut f: CThostFtdcInputOrderField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, broker_id);
    set_cstr_from_str_truncate(&mut f.InvestorID, investor_id);
    set_cstr_from_str_truncate(&mut f.InstrumentID, "IF1703");
    set_cstr_from_str_truncate(&mut f.UserID, user_id);
    f.Direction = THOST_FTDC_D_Buy;
    f.OrderPriceType = THOST_FTDC_OPT_LimitPrice;
    f.LimitPrice = 1f64;
    f.VolumeTotalOriginal = 1;
    f.CombOffsetFlag[0] = THOST_FTDC_OF_Open;
    f.CombHedgeFlag[0] = THOST_FTDC_HF_Speculation;
    f.TimeCondition = THOST_FTDC_TC_GFD;
    f.VolumeCondition = THOST_FTDC_VC_AV;
    f.MinVolume = 1;
    f.ContingentCondition = THOST_FTDC_CC_Immediately;
    f.ForceCloseReason = THOST_FTDC_FCC_NotForceClose;
    f.RequestID = 20;
    f
}

pub fn new_input_order_action() -> CThostFtdcInputOrderActionField {
    let mut f: CThostFtdcInputOrderActionField = Default::default();
    f.ActionFlag = THOST_FTDC_AF_Delete;
    f
}
