#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use anyhow::{anyhow, Result};
use myctp::ctp::*;
use myctp::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::rc::Rc;

struct Spi {}
impl CtpSpiTrait for Spi {
    fn on_rtn_event(&mut self, evt: EnumOnRtnEvent, param: *mut c_void) {
        let md = param as *const CThostFtdcDepthMarketDataField;
        if !md.is_null() {
            let md = unsafe { &*md };
            println!("==> on_rtn_event, {:?}, {:?}\n\n", evt, md);
        }
    }
}

fn main() {
    let md_api = MDApi::new("", false, false, Box::new(Spi {}));
    // let spi = Box::new(Spi {});
    // md_api.register_spi(spi);
    // md_api.register_front("tcp://180.168.146.187:10111");
    md_api.register_front("tcp://101.230.192.179:42213");
    println!("Hello ctp");
    md_api.init();
    std::thread::sleep(std::time::Duration::from_secs(2));
    match md_api.req_user_login(&Default::default(), 1) {
        Ok(()) => println!("req_user_login ok"),
        Err(err) => println!("req_user_login err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    let instrument_ids = vec!["IF2106", "au2109", "m2109", "CF109"];
    match md_api.subscribe_market_data(&instrument_ids.clone()) {
        Ok(()) => println!("subscribe_market_data ok"),
        Err(err) => println!("subscribe_market_data err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    match md_api.subscribe_for_quote_rsp(&instrument_ids) {
        Ok(()) => println!("subscribe_for_quote_rsp ok"),
        Err(err) => println!("subscribe_for_quote_rsp err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(5));
    /*
    match md_api.req_user_logout(&Default::default(), 2) {
        Ok(()) => println!("req_user_logout ok"),
        Err(err) => println!("req_user_logout err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    */
}
