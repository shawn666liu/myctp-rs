use std::io::Write;

use myctp::ctp::*;
use myctp::*;

// 7x24
const TRADER_FRONT: &'static str = "tcp://182.254.243.31:40001";

// const TRADER_FRONT: &'static str = "tcp://182.254.243.31:30001";
const BROKER_ID: &'static str = "9999";

struct Spi {}
impl CtpSpiTrait for Spi {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn on_rsp_event(
        &mut self,
        evt: EnumOnRspEvent,
        param: *mut c_void,
        _rsp_result: RspResult,
        _request_id: i32,
        _is_last: bool,
    ) {
        // 如果需要在本线程直接处理，则使用引用的方式，不用复制一次内存
        match cvoid_to_rsp_ref(evt, param) {
            OnRspOptRef::OnRspQryInvestorPosition(fld_opt) => {
                if let Some(fld) = fld_opt {
                    let dbg = DebugInvestorPositionField(fld);
                    println!("==> OnRspQryInvestorPosition:\n{:?}\n", dbg);
                }
            }
            OnRspOptRef::OnRspQryInstrument(fld_opt) => {
                if let Some(fld) = fld_opt {
                    if fld.ProductClass == THOST_FTDC_PC_Futures {
                        println!(
                            "==> OnRspQryInstrument: {}",
                            &gb18030_cstr_to_str(&fld.InstrumentID)
                        );
                    }
                }
            }
            OnRspOptRef::OnRspQryProduct(fld_opt) => {
                if let Some(fld) = fld_opt {
                    println!(
                        "==> OnRspQryProduct: {}, {}, {}, tick {}, factor {}",
                        &gb18030_cstr_to_str(&fld.ProductID),
                        &gb18030_cstr_to_str(&fld.ExchangeID),
                        &gb18030_cstr_to_str(&fld.ProductName),
                        fld.PriceTick,
                        fld.VolumeMultiple,
                    );
                }
            }
            _ => {}
        }

        // 如果需要转发到其他线程，则使用Box方式，复制一次内存
        let rspbox = cvoid_to_rsp_box(evt, param);

        // 此时可发送rspbox对象到其他线程

        match rspbox {
            OnRspOptBox::OnRspQryTradingAccount(fld_opt) => {
                if let Some(fld) = fld_opt {
                    let dbg = DebugTradingAccountField(&*fld);
                    println!("OnRspQryTradingAccount:\n{:#?}", dbg);
                }
            }
            _ => {}
        }

        // println!(
        //     "==> on_rsp_event, {:?}, {:#?} req_id {}, last? {}",
        //     evt, rsp_result, request_id, is_last
        // );
    }
}

fn main() {
    println!(
        "Going to connect to simnow {} with broker_id {}",
        TRADER_FRONT, BROKER_ID
    );
    let mut user_id = String::new();
    print!("user_id: ");
    std::io::stdout().flush().unwrap();
    match std::io::stdin().read_line(&mut user_id) {
        Ok(_) => (),
        Err(e) => {
            println!("invalid user_id, {}", e);
        }
    }
    user_id = user_id.trim_end().to_string();
    let mut password = String::new();
    print!("password: ");
    std::io::stdout().flush().unwrap();
    match std::io::stdin().read_line(&mut password) {
        Ok(_) => (),
        Err(e) => {
            println!("invalid password, {}", e);
        }
    }
    password = password.trim_end().to_string();
    let mut last_request_id = 0;
    // let flow_path = ::std::ffi::CString::new("").unwrap();

    let trader_api = TraderApi::new("", true, Box::new(Spi {}));
    trader_api.register_front(TRADER_FRONT);
    trader_api.subscribe_private_topic(THOST_TE_RESUME_TYPE::THOST_TERT_QUICK);
    trader_api.subscribe_public_topic(THOST_TE_RESUME_TYPE::THOST_TERT_QUICK);
    trader_api.init();
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_user_login(&new_login(BROKER_ID, &user_id, &password), last_request_id) {
        Ok(()) => println!("req_user_login ok"),
        Err(err) => println!("req_user_login err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_instrument(&new_qry_instrument("", ""), last_request_id) {
        Ok(()) => println!("req_qry_instrument ok"),
        Err(err) => println!("req_qry_instrument err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(2));
    last_request_id += 1;
    match trader_api.req_qry_product(&new_qry_product(""), last_request_id) {
        Ok(()) => println!("req_qry_product ok"),
        Err(err) => println!("req_qry_product err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_exchange(&new_qry_exchange(""), last_request_id) {
        Ok(()) => println!("req_qry_exchange ok"),
        Err(err) => println!("req_qry_exchange err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_settlement_info(
        &new_qry_settlement_info(BROKER_ID, &user_id),
        last_request_id,
    ) {
        Ok(()) => println!("req_qry_settlement_info ok"),
        Err(err) => println!("req_qry_settlement_info err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_settlement_info_confirm(
        &new_qry_settlement_info_confirm(BROKER_ID, &user_id),
        last_request_id,
    ) {
        Ok(()) => println!("req_qry_settlement_info_confirm ok"),
        Err(err) => println!("req_qry_settlement_info_confirm err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_settlement_info_confirm(
        &new_settlement_info_confirm(BROKER_ID, &user_id),
        last_request_id,
    ) {
        Ok(()) => println!("req_settlement_info_confirm ok"),
        Err(err) => println!("req_settlement_info_confirm err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_order(&new_qry_order(BROKER_ID, &user_id), last_request_id) {
        Ok(()) => println!("req_qry_order ok"),
        Err(err) => println!("req_qry_order err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_trade(&new_qry_trade(BROKER_ID, &user_id), last_request_id) {
        Ok(()) => println!("req_qry_trade ok"),
        Err(err) => println!("req_qry_trade err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_investor_position(
        &new_qry_investor_position(BROKER_ID, &user_id, ""),
        last_request_id,
    ) {
        Ok(()) => println!("req_qry_investor_position ok"),
        Err(err) => println!("req_qry_investor_position err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_trading_account(
        &new_qry_trading_account(BROKER_ID, &user_id),
        last_request_id,
    ) {
        Ok(()) => println!("req_qry_trading_account ok"),
        Err(err) => println!("req_qry_trading_account err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_investor(&new_qry_investor(BROKER_ID, &user_id), last_request_id) {
        Ok(()) => println!("req_qry_investor ok"),
        Err(err) => println!("req_qry_investor err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api
        .req_qry_trading_code(&new_qry_trading_code(BROKER_ID, &user_id), last_request_id)
    {
        Ok(()) => println!("req_qry_trading_code ok"),
        Err(err) => println!("req_qry_trading_code err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    // last_request_id += 1;
    // match trader_api.req_order_insert(
    //     &new_input_order(
    //         BROKER_ID,
    //         &user_id,
    //         "IF2601",
    //         THOST_FTDC_D_Buy,
    //         500f64,
    //         1,
    //         last_request_id,
    //     ),
    //     last_request_id,
    // ) {
    //     Ok(()) => println!("req_order_insert ok"),
    //     Err(err) => println!("req_order_insert err: {:?}", err),
    // };
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // last_request_id += 1;
    // match trader_api.req_order_action(&new_input_order_action(), last_request_id) {
    //     Ok(()) => println!("req_order_action ok"),
    //     Err(err) => println!("req_order_action err: {:?}", err),
    // };
    // std::thread::sleep(std::time::Duration::from_secs(1));
}
