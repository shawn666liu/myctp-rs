#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::unreadable_literal)]
// include!("generated.rs");

use super::generated::*;

pub const THOST_FTDC_BOOL_True: TThostFtdcBoolType = 1;
pub const THOST_FTDC_BOOL_False: TThostFtdcBoolType = 0;

pub const THOST_FTDC_COMB_FLAG_LENGTH: usize = 5;

use super::{gb18030_cstr_to_str, maybe_char, normalize_double, reduce_comb_flags};
use std::fmt;

pub trait MyDebug<'a> {
    type Debug: 'a;
    fn my_dbg(self) -> Self::Debug;
}
pub struct DebugDepthMarketData<'a>(pub &'a CThostFtdcDepthMarketDataField);

impl<'a> MyDebug<'a> for &'a CThostFtdcDepthMarketDataField {
    type Debug = DebugDepthMarketData<'a>;

    fn my_dbg(self) -> Self::Debug {
        DebugDepthMarketData(self)
    }
}

impl<'a> fmt::Debug for DebugDepthMarketData<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = fmt.debug_struct("CThostFtdcDepthMarketDataField");
        debug
            .field("TradingDay", &gb18030_cstr_to_str(&self.0.TradingDay))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.0.InstrumentID))
            .field("ExchangeID", &gb18030_cstr_to_str(&self.0.ExchangeID))
            .field(
                "ExchangeInstID",
                &gb18030_cstr_to_str(&self.0.ExchangeInstID),
            )
            .field("LastPrice", &normalize_double(self.0.LastPrice))
            .field(
                "PreSettlementPrice",
                &normalize_double(self.0.PreSettlementPrice),
            )
            .field("PreClosePrice", &normalize_double(self.0.PreClosePrice))
            .field("PreOpenInterest", &self.0.PreOpenInterest)
            .field("OpenPrice", &normalize_double(self.0.OpenPrice))
            .field("HighestPrice", &normalize_double(self.0.HighestPrice))
            .field("LowestPrice", &normalize_double(self.0.LowestPrice))
            .field("Volume", &self.0.Volume)
            .field("Turnover", &self.0.Turnover)
            .field("OpenInterest", &self.0.OpenInterest)
            .field("ClosePrice", &normalize_double(self.0.ClosePrice))
            .field("SettlementPrice", &normalize_double(self.0.SettlementPrice))
            .field("UpperLimitPrice", &normalize_double(self.0.UpperLimitPrice))
            .field("LowerLimitPrice", &normalize_double(self.0.LowerLimitPrice))
            .field("PreDelta", &normalize_double(self.0.PreDelta))
            .field("CurrDelta", &normalize_double(self.0.CurrDelta))
            .field("UpdateTime", &gb18030_cstr_to_str(&self.0.UpdateTime))
            .field("UpdateMillisec", &self.0.UpdateMillisec)
            .field("BidPrice1", &normalize_double(self.0.BidPrice1))
            .field("BidVolume1", &self.0.BidVolume1)
            .field("AskPrice1", &normalize_double(self.0.AskPrice1))
            .field("AskVolume1", &self.0.AskVolume1);
        if !(self.0.BidVolume2 == 0
            && self.0.AskVolume2 == 0
            && self.0.BidVolume3 == 0
            && self.0.AskVolume3 == 0
            && self.0.BidVolume4 == 0
            && self.0.AskVolume4 == 0
            && self.0.BidVolume5 == 0
            && self.0.AskVolume5 == 0)
        {
            debug
                .field("BidPrice2", &normalize_double(self.0.BidPrice2))
                .field("BidVolume2", &self.0.BidVolume2)
                .field("AskPrice2", &normalize_double(self.0.AskPrice2))
                .field("AskVolume2", &self.0.AskVolume2)
                .field("BidPrice3", &normalize_double(self.0.BidPrice3))
                .field("BidVolume3", &self.0.BidVolume3)
                .field("AskPrice3", &normalize_double(self.0.AskPrice3))
                .field("AskVolume3", &self.0.AskVolume3)
                .field("BidPrice4", &normalize_double(self.0.BidPrice4))
                .field("BidVolume4", &self.0.BidVolume4)
                .field("AskPrice4", &normalize_double(self.0.AskPrice4))
                .field("AskVolume4", &self.0.AskVolume4)
                .field("BidPrice5", &normalize_double(self.0.BidPrice5))
                .field("BidVolume5", &self.0.BidVolume5)
                .field("AskPrice5", &normalize_double(self.0.AskPrice5))
                .field("AskVolume5", &self.0.AskVolume5);
        }
        debug
            .field("AveragePrice", &normalize_double(self.0.AveragePrice))
            .field("ActionDay", &gb18030_cstr_to_str(&self.0.ActionDay))
            .finish()
    }
}

pub struct DebugTradingAccountField<'a>(pub &'a CThostFtdcTradingAccountField);

impl<'a> MyDebug<'a> for &'a CThostFtdcTradingAccountField {
    type Debug = DebugTradingAccountField<'a>;

    fn my_dbg(self) -> Self::Debug {
        DebugTradingAccountField(self)
    }
}

impl<'a> fmt::Debug for DebugTradingAccountField<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcTradingAccountField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.0.BrokerID))
            .field("AccountID", &gb18030_cstr_to_str(&self.0.AccountID))
            .field("PreMortgage", &self.0.PreMortgage)
            .field("PreCredit", &self.0.PreCredit)
            .field("PreDeposit", &self.0.PreDeposit)
            .field("PreBalance", &self.0.PreBalance)
            .field("PreMargin", &self.0.PreMargin)
            .field("InterestBase", &self.0.InterestBase)
            .field("Interest", &self.0.Interest)
            .field("Deposit", &self.0.Deposit)
            .field("Withdraw", &self.0.Withdraw)
            .field("FrozenMargin", &self.0.FrozenMargin)
            .field("FrozenCash", &self.0.FrozenCash)
            .field("FrozenCommission", &self.0.FrozenCommission)
            .field("CurrMargin", &self.0.CurrMargin)
            .field("CashIn", &self.0.CashIn)
            .field("Commission", &self.0.Commission)
            .field("CloseProfit", &self.0.CloseProfit)
            .field("PositionProfit", &self.0.PositionProfit)
            .field("Balance", &self.0.Balance)
            .field("Available", &self.0.Available)
            .field("WithdrawQuota", &self.0.WithdrawQuota)
            .field("Reserve", &self.0.Reserve)
            .field("TradingDay", &gb18030_cstr_to_str(&self.0.TradingDay))
            .field("SettlementID", &self.0.SettlementID)
            .field("Credit", &self.0.Credit)
            .field("Mortgage", &self.0.Mortgage)
            .field("ExchangeMargin", &self.0.ExchangeMargin)
            .field("DeliveryMargin", &self.0.DeliveryMargin)
            .field("ExchangeDeliveryMargin", &self.0.ExchangeDeliveryMargin)
            .field("ReserveBalance", &self.0.ReserveBalance)
            .field("CurrencyID", &gb18030_cstr_to_str(&self.0.CurrencyID))
            .field("PreFundMortgageIn", &self.0.PreFundMortgageIn)
            .field("PreFundMortgageOut", &self.0.PreFundMortgageOut)
            .field("FundMortgageIn", &self.0.FundMortgageIn)
            .field("FundMortgageOut", &self.0.FundMortgageOut)
            .field("FundMortgageAvailable", &self.0.FundMortgageAvailable)
            .field("MortgageableFund", &self.0.MortgageableFund)
            .field("SpecProductMargin", &self.0.SpecProductMargin)
            .field("SpecProductFrozenMargin", &self.0.SpecProductFrozenMargin)
            .field("SpecProductCommission", &self.0.SpecProductCommission)
            .field(
                "SpecProductFrozenCommission",
                &self.0.SpecProductFrozenCommission,
            )
            .field(
                "SpecProductPositionProfit",
                &self.0.SpecProductPositionProfit,
            )
            .field("SpecProductCloseProfit", &self.0.SpecProductCloseProfit)
            .field(
                "SpecProductPositionProfitByAlg",
                &self.0.SpecProductPositionProfitByAlg,
            )
            .field(
                "SpecProductExchangeMargin",
                &self.0.SpecProductExchangeMargin,
            )
            .finish()
    }
}

pub struct DebugInvestorPositionField<'a>(pub &'a CThostFtdcInvestorPositionField);

impl<'a> MyDebug<'a> for &'a CThostFtdcInvestorPositionField {
    type Debug = DebugInvestorPositionField<'a>;

    fn my_dbg(self) -> Self::Debug {
        DebugInvestorPositionField(self)
    }
}

impl<'a> fmt::Debug for DebugInvestorPositionField<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcInvestorPositionField")
            .field("InstrumentID", &gb18030_cstr_to_str(&self.0.InstrumentID))
            .field("BrokerID", &gb18030_cstr_to_str(&self.0.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.0.InvestorID))
            .field("PosiDirection", &char::from(self.0.PosiDirection))
            .field("HedgeFlag", &char::from(self.0.HedgeFlag))
            .field("PositionDate", &char::from(self.0.PositionDate))
            .field("YdPosition", &self.0.YdPosition)
            .field("Position", &self.0.Position)
            .field("LongFrozen", &self.0.LongFrozen)
            .field("ShortFrozen", &self.0.ShortFrozen)
            .field("LongFrozenAmount", &self.0.LongFrozenAmount)
            .field("ShortFrozenAmount", &self.0.ShortFrozenAmount)
            .field("OpenVolume", &self.0.OpenVolume)
            .field("CloseVolume", &self.0.CloseVolume)
            .field("OpenAmount", &self.0.OpenAmount)
            .field("CloseAmount", &self.0.CloseAmount)
            .field("PositionCost", &self.0.PositionCost)
            .field("PreMargin", &self.0.PreMargin)
            .field("UseMargin", &self.0.UseMargin)
            .field("FrozenMargin", &self.0.FrozenMargin)
            .field("FrozenCash", &self.0.FrozenCash)
            .field("FrozenCommission", &self.0.FrozenCommission)
            .field("CashIn", &self.0.CashIn)
            .field("Commission", &self.0.Commission)
            .field("CloseProfit", &self.0.CloseProfit)
            .field("PositionProfit", &self.0.PositionProfit)
            .field("PreSettlementPrice", &self.0.PreSettlementPrice)
            .field("SettlementPrice", &self.0.SettlementPrice)
            .field("TradingDay", &gb18030_cstr_to_str(&self.0.TradingDay))
            .field("SettlementID", &self.0.SettlementID)
            .field("OpenCost", &self.0.OpenCost)
            .field("ExchangeMargin", &self.0.ExchangeMargin)
            .field("CombPosition", &self.0.CombPosition)
            .field("CombLongFrozen", &self.0.CombLongFrozen)
            .field("CombShortFrozen", &self.0.CombShortFrozen)
            .field("CloseProfitByDate", &self.0.CloseProfitByDate)
            .field("CloseProfitByTrade", &self.0.CloseProfitByTrade)
            .field("TodayPosition", &self.0.TodayPosition)
            .field("MarginRateByMoney", &self.0.MarginRateByMoney)
            .field("MarginRateByVolume", &self.0.MarginRateByVolume)
            .field("StrikeFrozen", &self.0.StrikeFrozen)
            .field("StrikeFrozenAmount", &self.0.StrikeFrozenAmount)
            .field("AbandonFrozen", &self.0.AbandonFrozen)
            .finish()
    }
}

pub struct DebugFtdcInstrumentField<'a>(pub &'a CThostFtdcInstrumentField);

impl<'a> MyDebug<'a> for &'a CThostFtdcInstrumentField {
    type Debug = DebugFtdcInstrumentField<'a>;

    fn my_dbg(self) -> Self::Debug {
        DebugFtdcInstrumentField(self)
    }
}

impl<'a> fmt::Debug for DebugFtdcInstrumentField<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcInstrumentField")
            .field("InstrumentID", &gb18030_cstr_to_str(&self.0.InstrumentID))
            .field("ExchangeID", &gb18030_cstr_to_str(&self.0.ExchangeID))
            .field(
                "InstrumentName",
                &gb18030_cstr_to_str(&self.0.InstrumentName),
            )
            .field(
                "ExchangeInstID",
                &gb18030_cstr_to_str(&self.0.ExchangeInstID),
            )
            .field("ProductID", &gb18030_cstr_to_str(&self.0.ProductID))
            .field("ProductClass", &char::from(self.0.ProductClass))
            .field("DeliveryYear", &self.0.DeliveryYear)
            .field("DeliveryMonth", &self.0.DeliveryMonth)
            .field("MaxMarketOrderVolume", &self.0.MaxMarketOrderVolume)
            .field("MinMarketOrderVolume", &self.0.MinMarketOrderVolume)
            .field("MaxLimitOrderVolume", &self.0.MaxLimitOrderVolume)
            .field("MinLimitOrderVolume", &self.0.MinLimitOrderVolume)
            .field("VolumeMultiple", &self.0.VolumeMultiple)
            .field("PriceTick", &self.0.PriceTick)
            .field("CreateDate", &gb18030_cstr_to_str(&self.0.CreateDate))
            .field("OpenDate", &gb18030_cstr_to_str(&self.0.OpenDate))
            .field("ExpireDate", &gb18030_cstr_to_str(&self.0.ExpireDate))
            .field(
                "StartDelivDate",
                &gb18030_cstr_to_str(&self.0.StartDelivDate),
            )
            .field("EndDelivDate", &gb18030_cstr_to_str(&self.0.EndDelivDate))
            .field("InstLifePhase", &char::from(self.0.InstLifePhase))
            .field("IsTrading", &self.0.IsTrading)
            .field("PositionType", &char::from(self.0.PositionType))
            .field("PositionDateType", &char::from(self.0.PositionDateType))
            .field("LongMarginRatio", &normalize_double(self.0.LongMarginRatio))
            .field(
                "ShortMarginRatio",
                &normalize_double(self.0.ShortMarginRatio),
            )
            .field(
                "MaxMarginSideAlgorithm",
                &char::from(self.0.MaxMarginSideAlgorithm),
            )
            .field(
                "UnderlyingInstrID",
                &gb18030_cstr_to_str(&self.0.UnderlyingInstrID),
            )
            .field("StrikePrice", &normalize_double(self.0.StrikePrice))
            .field("OptionsType", &maybe_char(self.0.OptionsType))
            .field(
                "UnderlyingMultiple",
                &normalize_double(self.0.UnderlyingMultiple),
            )
            .field("CombinationType", &maybe_char(self.0.CombinationType))
            .finish()
    }
}

/*
impl fmt::Debug for CThostFtdcRspAuthenticateField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcRspAuthenticateField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .field(
                "UserProductInfo",
                &gb18030_cstr_to_str(&self.UserProductInfo),
            )
            .field("AppID", &gb18030_cstr_to_str(&self.AppID))
            .field("AppType", &maybe_char(self.AppType))
            .finish()
    }
}

impl fmt::Debug for CThostFtdcReqUserLoginField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcReqUserLoginField")
            .field("TradingDay", &gb18030_cstr_to_str(&self.TradingDay))
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .field("Password", &gb18030_cstr_to_str(&self.Password))
            .field(
                "UserProductInfo",
                &gb18030_cstr_to_str(&self.UserProductInfo),
            )
            .field(
                "InterfaceProductInfo",
                &gb18030_cstr_to_str(&self.InterfaceProductInfo),
            )
            .field("ProtocolInfo", &gb18030_cstr_to_str(&self.ProtocolInfo))
            .field("MacAddress", &gb18030_cstr_to_str(&self.MacAddress))
            .field(
                "OneTimePassword",
                &gb18030_cstr_to_str(&self.OneTimePassword),
            )
            .field(
                "ClientIPAddress",
                &gb18030_cstr_to_str(&self.ClientIPAddress),
            )
            .finish()
    }
}

impl fmt::Debug for CThostFtdcRspUserLoginField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcRspUserLoginField")
            .field("TradingDay", &gb18030_cstr_to_str(&self.TradingDay))
            .field("LoginTime", &gb18030_cstr_to_str(&self.LoginTime))
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .field("SystemName", &gb18030_cstr_to_str(&self.SystemName))
            .field("FrontID", &self.FrontID)
            .field("SessionID", &self.SessionID)
            .field("MaxOrderRef", &gb18030_cstr_to_str(&self.MaxOrderRef))
            .field("SHFETime", &gb18030_cstr_to_str(&self.SHFETime))
            .field("DCETime", &gb18030_cstr_to_str(&self.DCETime))
            .field("CZCETime", &gb18030_cstr_to_str(&self.CZCETime))
            .field("FFEXTime", &gb18030_cstr_to_str(&self.FFEXTime))
            .field("INETime", &gb18030_cstr_to_str(&self.INETime))
            .finish()
    }
}

impl fmt::Debug for CThostFtdcUserLogoutField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcUserLogoutField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .finish()
    }
}

impl fmt::Debug for CThostFtdcUserPasswordUpdateField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcUserPasswordUpdateField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .field("OldPassword", &gb18030_cstr_to_str(&self.OldPassword))
            .field("NewPassword", &gb18030_cstr_to_str(&self.NewPassword))
            .finish()
    }
}

impl fmt::Debug for CThostFtdcSpecificInstrumentField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcSpecificInstrumentField")
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .finish()
    }
}


impl fmt::Debug for CThostFtdcInstrumentStatusField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcInstrumentStatusField ")
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("ExchangeInstID", &gb18030_cstr_to_str(&self.ExchangeInstID))
            .field(
                "SettlementGroupID",
                &gb18030_cstr_to_str(&self.SettlementGroupID),
            )
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("InstrumentStatus", &char::from(self.InstrumentStatus))
            .field("TradingSegmentSN", &self.TradingSegmentSN)
            .field("EnterTime", &gb18030_cstr_to_str(&self.EnterTime))
            .field("EnterReason", &char::from(self.EnterReason))
            .finish()
    }
}

impl fmt::Debug for CThostFtdcOrderField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcOrderField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("OrderRef", &gb18030_cstr_to_str(&self.OrderRef))
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .field("OrderPriceType", &char::from(self.OrderPriceType))
            .field("Direction", &char::from(self.Direction))
            .field("CombOffsetFlag", &reduce_comb_flags(&self.CombOffsetFlag))
            .field("CombHedgeFlag", &reduce_comb_flags(&self.CombHedgeFlag))
            .field("LimitPrice", &self.LimitPrice)
            .field("VolumeTotalOriginal", &self.VolumeTotalOriginal)
            .field("TimeCondition", &char::from(self.TimeCondition))
            .field("GTDDate", &gb18030_cstr_to_str(&self.GTDDate))
            .field("VolumeCondition", &char::from(self.VolumeCondition))
            .field("MinVolume", &self.MinVolume)
            .field("ContingentCondition", &char::from(self.ContingentCondition))
            .field("StopPrice", &self.StopPrice)
            .field("ForceCloseReason", &char::from(self.ForceCloseReason))
            .field("IsAutoSuspend", &self.IsAutoSuspend)
            .field("BusinessUnit", &gb18030_cstr_to_str(&self.BusinessUnit))
            .field("RequestID", &self.RequestID)
            .field("OrderLocalID", &gb18030_cstr_to_str(&self.OrderLocalID))
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("ParticipantID", &gb18030_cstr_to_str(&self.ParticipantID))
            .field("ClientID", &gb18030_cstr_to_str(&self.ClientID))
            .field("ExchangeInstID", &gb18030_cstr_to_str(&self.ExchangeInstID))
            .field("TraderID", &gb18030_cstr_to_str(&self.TraderID))
            .field("InstallID", &self.InstallID)
            .field("OrderSubmitStatus", &char::from(self.OrderSubmitStatus))
            .field("NotifySequence", &self.NotifySequence)
            .field("TradingDay", &gb18030_cstr_to_str(&self.TradingDay))
            .field("SettlementID", &self.SettlementID)
            .field("OrderSysID", &gb18030_cstr_to_str(&self.OrderSysID))
            .field("OrderSource", &maybe_char(self.OrderSource))
            .field("OrderStatus", &char::from(self.OrderStatus))
            .field("OrderType", &maybe_char(self.OrderType))
            .field("VolumeTraded", &self.VolumeTraded)
            .field("VolumeTotal", &self.VolumeTotal)
            .field("InsertDate", &gb18030_cstr_to_str(&self.InsertDate))
            .field("InsertTime", &gb18030_cstr_to_str(&self.InsertTime))
            .field("ActiveTime", &gb18030_cstr_to_str(&self.ActiveTime))
            .field("SuspendTime", &gb18030_cstr_to_str(&self.SuspendTime))
            .field("UpdateTime", &gb18030_cstr_to_str(&self.UpdateTime))
            .field("CancelTime", &gb18030_cstr_to_str(&self.CancelTime))
            .field("ActiveTraderID", &gb18030_cstr_to_str(&self.ActiveTraderID))
            .field("ClearingPartID", &gb18030_cstr_to_str(&self.ClearingPartID))
            .field("SequenceNo", &self.SequenceNo)
            .field("FrontID", &self.FrontID)
            .field("SessionID", &self.SessionID)
            .field(
                "UserProductInfo",
                &gb18030_cstr_to_str(&self.UserProductInfo),
            )
            .field("StatusMsg", &gb18030_cstr_to_str(&self.StatusMsg))
            .field("UserForceClose", &self.UserForceClose)
            .field("ActiveUserID", &gb18030_cstr_to_str(&self.ActiveUserID))
            .field("BrokerOrderSeq", &self.BrokerOrderSeq)
            .field(
                "RelativeOrderSysID",
                &gb18030_cstr_to_str(&self.RelativeOrderSysID),
            )
            .field("ZCETotalTradedVolume", &self.ZCETotalTradedVolume)
            .field("IsSwapOrder", &self.IsSwapOrder)
            .finish()
    }
}

impl fmt::Debug for CThostFtdcTradeField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcTradeField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("OrderRef", &gb18030_cstr_to_str(&self.OrderRef))
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("TradeID", &gb18030_cstr_to_str(&self.TradeID))
            .field("Direction", &char::from(self.Direction))
            .field("OrderSysID", &gb18030_cstr_to_str(&self.OrderSysID))
            .field("ParticipantID", &gb18030_cstr_to_str(&self.ParticipantID))
            .field("ClientID", &gb18030_cstr_to_str(&self.ClientID))
            .field("TradingRole", &self.TradingRole)
            .field("ExchangeInstID", &gb18030_cstr_to_str(&self.ExchangeInstID))
            .field("OffsetFlag", &char::from(self.OffsetFlag))
            .field("HedgeFlag", &char::from(self.HedgeFlag))
            .field("Price", &self.Price)
            .field("Volume", &self.Volume)
            .field("TradeDate", &gb18030_cstr_to_str(&self.TradeDate))
            .field("TradeTime", &gb18030_cstr_to_str(&self.TradeTime))
            .field("TradeType", &maybe_char(self.TradeType))
            .field("PriceSource", &maybe_char(self.PriceSource))
            .field("TraderID", &gb18030_cstr_to_str(&self.TraderID))
            .field("OrderLocalID", &gb18030_cstr_to_str(&self.OrderLocalID))
            .field("ClearingPartID", &gb18030_cstr_to_str(&self.ClearingPartID))
            .field("BusinessUnit", &gb18030_cstr_to_str(&self.BusinessUnit))
            .field("SequenceNo", &self.SequenceNo)
            .field("TradingDay", &gb18030_cstr_to_str(&self.TradingDay))
            .field("SettlementID", &self.SettlementID)
            .field("BrokerOrderSeq", &self.BrokerOrderSeq)
            .field("TradeSource", &char::from(self.TradeSource))
            .finish()
    }
}

impl fmt::Debug for CThostFtdcSettlementInfoField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcSettlementInfoField")
            .field("TradingDay", &gb18030_cstr_to_str(&self.TradingDay))
            .field("SettlementID", &self.SettlementID)
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("SequenceNo", &self.SequenceNo)
            .field("Content", &gb18030_cstr_to_str(&self.Content))
            .finish()
    }
}

impl fmt::Debug for CThostFtdcExchangeMarginRateField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcExchangeMarginRateField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("HedgeFlag", &char::from(self.HedgeFlag))
            .field(
                "LongMarginRatioByMoney",
                &normalize_double(self.LongMarginRatioByMoney),
            )
            .field(
                "LongMarginRatioByVolume",
                &normalize_double(self.LongMarginRatioByVolume),
            )
            .field(
                "ShortMarginRatioByMoney",
                &normalize_double(self.ShortMarginRatioByMoney),
            )
            .field(
                "ShortMarginRatioByVolume",
                &normalize_double(self.ShortMarginRatioByVolume),
            )
            .finish()
    }
}

impl fmt::Debug for CThostFtdcExchangeMarginRateAdjustField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcExchangeMarginRateAdjustField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("HedgeFlag", &char::from(self.HedgeFlag))
            .field(
                "LongMarginRatioByMoney",
                &normalize_double(self.LongMarginRatioByMoney),
            )
            .field(
                "LongMarginRatioByVolume",
                &normalize_double(self.LongMarginRatioByVolume),
            )
            .field(
                "ShortMarginRatioByMoney",
                &normalize_double(self.ShortMarginRatioByMoney),
            )
            .field(
                "ShortMarginRatioByVolume",
                &normalize_double(self.ShortMarginRatioByVolume),
            )
            .field(
                "ExchLongMarginRatioByMoney",
                &normalize_double(self.ExchLongMarginRatioByMoney),
            )
            .field(
                "ExchLongMarginRatioByVolume",
                &normalize_double(self.ExchLongMarginRatioByVolume),
            )
            .field(
                "ExchShortMarginRatioByMoney",
                &normalize_double(self.ExchShortMarginRatioByMoney),
            )
            .field(
                "ExchShortMarginRatioByVolume",
                &normalize_double(self.ExchShortMarginRatioByVolume),
            )
            .field(
                "NoLongMarginRatioByMoney",
                &normalize_double(self.NoLongMarginRatioByMoney),
            )
            .field(
                "NoLongMarginRatioByVolume",
                &normalize_double(self.NoLongMarginRatioByVolume),
            )
            .field(
                "NoShortMarginRatioByMoney",
                &normalize_double(self.NoShortMarginRatioByMoney),
            )
            .field(
                "NoShortMarginRatioByVolume",
                &normalize_double(self.NoShortMarginRatioByVolume),
            )
            .finish()
    }
}

impl fmt::Debug for CThostFtdcExchangeRateField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcExchangeRateField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("FromCurrencyID", &gb18030_cstr_to_str(&self.FromCurrencyID))
            .field("FromCurrencyUnit", &self.FromCurrencyUnit)
            .field("ToCurrencyID", &gb18030_cstr_to_str(&self.ToCurrencyID))
            .field("ExchangeRate", &self.ExchangeRate)
            .finish()
    }
}

impl fmt::Debug for CThostFtdcSettlementInfoConfirmField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcSettlementInfoConfirmField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("ConfirmDate", &gb18030_cstr_to_str(&self.ConfirmDate))
            .field("ConfirmTime", &gb18030_cstr_to_str(&self.ConfirmTime))
            .finish()
    }
}

impl fmt::Debug for CThostFtdcInputOrderField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcInputOrderField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("OrderRef", &gb18030_cstr_to_str(&self.OrderRef))
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .field("OrderPriceType", &char::from(self.OrderPriceType))
            .field("Direction", &char::from(self.Direction))
            .field("CombOffsetFlag", &reduce_comb_flags(&self.CombOffsetFlag))
            .field("CombHedgeFlag", &reduce_comb_flags(&self.CombHedgeFlag))
            .field("LimitPrice", &self.LimitPrice)
            .field("VolumeTotalOriginal", &self.VolumeTotalOriginal)
            .field("TimeCondition", &char::from(self.TimeCondition))
            .field("GTDDate", &gb18030_cstr_to_str(&self.GTDDate))
            .field("VolumeCondition", &char::from(self.VolumeCondition))
            .field("MinVolume", &self.MinVolume)
            .field("ContingentCondition", &char::from(self.ContingentCondition))
            .field("StopPrice", &self.StopPrice)
            .field("ForceCloseReason", &char::from(self.ForceCloseReason))
            .field("IsAutoSuspend", &self.IsAutoSuspend)
            .field("BusinessUnit", &gb18030_cstr_to_str(&self.BusinessUnit))
            .field("RequestID", &self.RequestID)
            .field("UserForceClose", &self.UserForceClose)
            .field("IsSwapOrder", &self.IsSwapOrder)
            .finish()
    }
}

impl fmt::Debug for CThostFtdcOrderActionField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcOrderActionField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("OrderActionRef", &self.OrderActionRef)
            .field("OrderRef", &gb18030_cstr_to_str(&self.OrderRef))
            .field("RequestID", &self.RequestID)
            .field("FrontID", &self.FrontID)
            .field("SessionID", &self.SessionID)
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("OrderSysID", &gb18030_cstr_to_str(&self.OrderSysID))
            .field("ActionFlag", &char::from(self.ActionFlag))
            .field("LimitPrice", &self.LimitPrice)
            .field("VolumeChange", &self.VolumeChange)
            .field("ActionDate", &gb18030_cstr_to_str(&self.ActionDate))
            .field("ActionTime", &gb18030_cstr_to_str(&self.ActionTime))
            .field("TraderID", &gb18030_cstr_to_str(&self.TraderID))
            .field("InstallID", &self.InstallID)
            .field("OrderLocalID", &gb18030_cstr_to_str(&self.OrderLocalID))
            .field("ActionLocalID", &gb18030_cstr_to_str(&self.ActionLocalID))
            .field("ParticipantID", &gb18030_cstr_to_str(&self.ParticipantID))
            .field("ClientID", &gb18030_cstr_to_str(&self.ClientID))
            .field("BusinessUnit", &gb18030_cstr_to_str(&self.BusinessUnit))
            .field("OrderActionStatus", &self.OrderActionStatus)
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .field("StatusMsg", &gb18030_cstr_to_str(&self.StatusMsg))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .finish()
    }
}

impl fmt::Debug for CThostFtdcInputOrderActionField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcInputOrderActionField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("OrderActionRef", &self.OrderActionRef)
            .field("OrderRef", &gb18030_cstr_to_str(&self.OrderRef))
            .field("RequestID", &self.RequestID)
            .field("FrontID", &self.FrontID)
            .field("SessionID", &self.SessionID)
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("OrderSysID", &gb18030_cstr_to_str(&self.OrderSysID))
            .field("ActionFlag", &char::from(self.ActionFlag))
            .field("LimitPrice", &self.LimitPrice)
            .field("VolumeChange", &self.VolumeChange)
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .finish()
    }
}

impl fmt::Debug for CThostFtdcTradingNoticeInfoField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcTradingNoticeInfoField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("SendTime", &gb18030_cstr_to_str(&self.SendTime))
            .field("FieldContent", &gb18030_cstr_to_str(&self.FieldContent))
            .field("SequenceSeries", &self.SequenceSeries)
            .field("SequenceNo", &self.SequenceNo)
            .finish()
    }
}

impl fmt::Debug for CThostFtdcInvestorField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcInvestorField")
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field(
                "InvestorGroupID",
                &gb18030_cstr_to_str(&self.InvestorGroupID),
            )
            .field("InvestorName", &gb18030_cstr_to_str(&self.InvestorName))
            .field("IdentifiedCardType", &char::from(self.IdentifiedCardType))
            .field(
                "IdentifiedCardNo",
                &gb18030_cstr_to_str(&self.IdentifiedCardNo),
            )
            .field("IsActive", &self.IsActive)
            .field("Telephone", &gb18030_cstr_to_str(&self.Telephone))
            .field("Address", &gb18030_cstr_to_str(&self.Address))
            .field("OpenDate", &gb18030_cstr_to_str(&self.OpenDate))
            .field("Mobile", &gb18030_cstr_to_str(&self.Mobile))
            .field("CommModelID", &gb18030_cstr_to_str(&self.CommModelID))
            .field("MarginModelID", &gb18030_cstr_to_str(&self.MarginModelID))
            .finish()
    }
}

impl fmt::Debug for CThostFtdcTradingCodeField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcTradingCodeField")
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("ClientID", &gb18030_cstr_to_str(&self.ClientID))
            .field("IsActive", &self.IsActive)
            .field("ClientIDType", &char::from(self.ClientIDType))
            .finish()
    }
}


impl fmt::Debug for CThostFtdcInstrumentMarginRateField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcInstrumentMarginRateField")
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("InvestorRange", &self.InvestorRange)
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("HedgeFlag", &char::from(self.HedgeFlag))
            .field(
                "LongMarginRatioByMoney",
                &normalize_double(self.LongMarginRatioByMoney),
            )
            .field(
                "LongMarginRatioByVolume",
                &normalize_double(self.LongMarginRatioByVolume),
            )
            .field(
                "ShortMarginRatioByMoney",
                &normalize_double(self.ShortMarginRatioByMoney),
            )
            .field(
                "ShortMarginRatioByVolume",
                &normalize_double(self.ShortMarginRatioByVolume),
            )
            .field("IsRelative", &self.IsRelative)
            .finish()
    }
}

impl fmt::Debug for CThostFtdcInstrumentCommissionRateField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcInstrumentCommissionRateField")
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("InvestorRange", &self.InvestorRange)
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("OpenRatioByMoney", &self.OpenRatioByMoney)
            .field("OpenRatioByVolume", &self.OpenRatioByVolume)
            .field("CloseRatioByMoney", &self.CloseRatioByMoney)
            .field("CloseRatioByVolume", &self.CloseRatioByVolume)
            .field("CloseTodayRatioByMoney", &self.CloseTodayRatioByMoney)
            .field("CloseTodayRatioByVolume", &self.CloseTodayRatioByVolume)
            .finish()
    }
}



impl fmt::Debug for CThostFtdcExchangeField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcExchangeField")
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("ExchangeName", &gb18030_cstr_to_str(&self.ExchangeName))
            .field("ExchangeProperty", &char::from(self.ExchangeProperty))
            .finish()
    }
}

impl fmt::Debug for CThostFtdcProductField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcProductField")
            .field("ProductID", &gb18030_cstr_to_str(&self.ProductID))
            .field("ProductName", &gb18030_cstr_to_str(&self.ProductName))
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("ProductClass", &char::from(self.ProductClass))
            .field("VolumeMultiple", &self.VolumeMultiple)
            .field("PriceTick", &self.PriceTick)
            .field("MaxMarketOrderVolume", &self.MaxMarketOrderVolume)
            .field("MinMarketOrderVolume", &self.MinMarketOrderVolume)
            .field("MaxLimitOrderVolume", &self.MaxLimitOrderVolume)
            .field("MinLimitOrderVolume", &self.MinLimitOrderVolume)
            .field("PositionType", &char::from(self.PositionType))
            .field("PositionDateType", &char::from(self.PositionDateType))
            .field("CloseDealType", &char::from(self.CloseDealType))
            .field(
                "TradeCurrencyID",
                &gb18030_cstr_to_str(&self.TradeCurrencyID),
            )
            .field(
                "MortgageFundUseRange",
                &char::from(self.MortgageFundUseRange),
            )
            .field(
                "ExchangeProductID",
                &gb18030_cstr_to_str(&self.ExchangeProductID),
            )
            .field(
                "UnderlyingMultiple",
                &normalize_double(self.UnderlyingMultiple),
            )
            .finish()
    }
}

impl fmt::Debug for CThostFtdcForQuoteRspField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CThostFtdcForQuoteRspField")
            .field("TradingDay", &gb18030_cstr_to_str(&self.TradingDay))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("ForQuoteSysID", &gb18030_cstr_to_str(&self.ForQuoteSysID))
            .field("ForQuoteTime", &gb18030_cstr_to_str(&self.ForQuoteTime))
            .field("ActionDay", &gb18030_cstr_to_str(&self.ActionDay))
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .finish()
    }
}
 */
