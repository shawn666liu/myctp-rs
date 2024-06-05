#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use anyhow::{anyhow, Result};
use myctp::ctp::*;
use myctp::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::rc::Rc;

struct MdSpi {}
impl Drop for MdSpi {
    fn drop(&mut self) {
        println!("MdSpi::drop()");
    }
}
impl CtpSpiTrait for MdSpi {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn on_rtn_event(&mut self, evt: EnumOnRtnEvent, param: *mut c_void) {
        let md = param as *const CThostFtdcDepthMarketDataField;
        if !md.is_null() {
            let md = unsafe { &*md };
            // println!("==> on_rtn_event");
            println!("==> on_rtn_event, {:?}, {:?}\n\n", evt, md);
        }
    }
}

#[derive(Default)]
struct TestApp {
    raw_api: Option<MDApi>,
}
impl TestApp {
    pub fn check_api(&mut self) {
        if self.raw_api.is_none() {
            let md_api = MDApi::new("", false, false, Box::new(MdSpi {}));
            self.raw_api = Some(md_api);
            println!("raw api created.");
        }
    }

    pub fn connect(&mut self) {
        if let Some(md_api) = &self.raw_api {
            md_api.register_front("tcp://101.230.192.179:42213");
            println!("Hello ctp");
            md_api.init();
            std::thread::sleep(std::time::Duration::from_secs(2));
            match md_api.req_user_login(&Default::default(), 1) {
                Ok(()) => println!("req_user_login ok"),
                Err(err) => println!("req_user_login err: {:?}", err),
            };
            std::thread::sleep(std::time::Duration::from_secs(1));
            let instrument_ids = vec!["IF2108", "au2112", "m2109", "CF109"];
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
        }
    }
    pub fn disconnect(&mut self) {
        if self.raw_api.is_none() {
            return;
        }

        {
            let _ = self.raw_api.take();
            println!("raw api will destroy.")
        }
        println!("raw api destroyed.")
    }
}

fn main2() {
    let mut app = TestApp::default();
    app.check_api();
    app.connect();
    // std::thread::sleep(std::time::Duration::from_secs(5));
    app.disconnect();
}

fn main() {
    let md_api = MDApi::new("", false, false, Box::new(MdSpi {}));
    // let spi = Box::new(MdSpi {});
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
    let instrument_ids = vec!["IF2409", "au2412", "m2409", "CF409"];
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
