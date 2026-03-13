#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use anyhow::{Result, anyhow};
use myctp::ctp::*;
use myctp::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::rc::Rc;

const MD_FRONT: &'static str = "tcp://182.254.243.31:30011";

struct MdSpi {}
impl Drop for MdSpi {
    fn drop(&mut self) {
        println!("MdSpi::drop()");
    }
}
impl CtpSpiCallback for MdSpi {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn on_rtn_callback(&mut self, evt: EnumOnRtnEvent, param: *mut c_void) {
        // 如果需要在本线程直接处理，则使用引用的方式，不用复制一次内存
        match cvoid_to_rtn_ref(evt, param) {
            OnRtnOptRef::OnRtnDepthMarketData(fld_opt) => {
                if let Some(md) = fld_opt {
                    let dbg = DebugDepthMarketData(md);
                    println!("==> OnRtnDepthMarketData_Ref:\n{:?}\n\n", dbg);
                }
            }
            _ => {}
        }

        // 如果需要转发到其他线程，则使用Box或者Arc方式，复制一次内存
        // cvoid_to_rtn_arc
        // let rtnbox = cvoid_to_rtn_box(evt, param);

        // // 此时可发送rtnbox对象到其他线程

        // match rtnbox {
        //     OnRtnOptBox::OnRtnDepthMarketData(fld_opt) => {
        //         if let Some(md) = fld_opt {
        //             let dbg = DebugDepthMarketData(md.as_ref());
        //             println!("==> OnRtnDepthMarketData_Box:\n{:#?}\n\n", dbg);
        //         }
        //     }
        //     _ => {}
        // }

        let rtnbox = cvoid_to_rtn_arc(evt, param);

        // 此时可发送rtnbox对象到其他线程

        match rtnbox {
            OnRtnOptArc::OnRtnDepthMarketData(fld_opt) => {
                if let Some(md) = fld_opt {
                    let dbg = DebugDepthMarketData(md.as_ref());
                    println!("==> OnRtnDepthMarketData_Arc:\n{:#?}\n\n", dbg);
                }
            }
            _ => {}
        }

        // let md = param as *const CThostFtdcDepthMarketDataField;
        // if !md.is_null() {
        //     let md = unsafe { &*md };
        //     println!("==> on_rtn_callback, {:?}, {:?}\n\n", evt, md);
        // }
    }
}

#[derive(Default)]
struct TestApp {
    raw_api: Option<MDApi>,
}
impl TestApp {
    pub fn check_api(&mut self) {
        if self.raw_api.is_none() {
            let md_api = MDApi::new("", false, false, true, Box::new(MdSpi {}));
            self.raw_api = Some(md_api);
            println!("raw api created.");
        }
    }

    pub fn connect(&mut self) {
        if let Some(md_api) = &self.raw_api {
            md_api.register_front(MD_FRONT);
            println!("Hello ctp");
            md_api.init();
            std::thread::sleep(std::time::Duration::from_secs(2));
            match md_api.req_user_login(&Default::default(), 1) {
                Ok(()) => println!("req_user_login ok"),
                Err(err) => println!("req_user_login err: {:?}", err),
            };
            std::thread::sleep(std::time::Duration::from_secs(1));
            let instrument_ids = vec!["IF2603", "au2603", "m2605", "CF605"];
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

fn main() {
    let mut app = TestApp::default();
    app.check_api();
    app.connect();
    std::thread::sleep(std::time::Duration::from_secs(5));
    app.disconnect();
}

fn main1() {
    let md_api = MDApi::new("", false, false, true, Box::new(MdSpi {}));
    md_api.register_front(MD_FRONT);
    println!("Hello ctp");
    md_api.init();
    std::thread::sleep(std::time::Duration::from_secs(2));
    match md_api.req_user_login(&Default::default(), 1) {
        Ok(()) => println!("req_user_login ok"),
        Err(err) => println!("req_user_login err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    let instrument_ids = vec!["IF2603", "au2603", "m2605", "CF605"];
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
