
use std::cell::{RefCell, RefMut};
use futures::prelude::*;
use std::collections::HashMap;
use std::future::Future;
use futures::executor::block_on;


//#[tokio::main]
async fn HTTP_join_all() -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..5 {
        let url = "http://www.taobao.com";
        let mut vec = Vec::new();

        let mut f = reqwest::Client::builder().build()?.get(url).send();    
        vec.push(f);
        
        let mut f = reqwest::Client::builder().build()?.get("http://www.baidu.com").send();
        vec.push(f);

        futures::future::join_all(vec).await.iter().for_each(|x| println!("aaaa: {:?}", x));
        println!("on loop...")
    }
    Ok(())
}


async fn HTTP1_1() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://www.taobao.com";

    let mut fut = reqwest::Client::builder().build()?.get(url).send();    
    
    loop {
        let poll = std::future::poll_with_tls_context(
            unsafe { std::pin::Pin::new_unchecked(&mut fut)});

        match poll {
            core::task::Poll::Ready(r) => {
                println!("is ready: {:?}", r);
                r.map(|x| {
                    println!("response {:?}", x);
                });
                break;
            }
            core::task::Poll::Pending => 
                println!("pending.....")
        };
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}


#[tokio::main]
async fn HTTP1() -> Result<(), Box<dyn std::error::Error>> {
    // let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")
    let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip").await?
        .json().await?;
    println!("{:#?}", resp);
    Ok(())
}


async fn HTTP2() -> Result<(), Box<dyn std::error::Error>> {
    // let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")

    //let url = "https://httpbin.org/ip";
    let url = "http://www.taobao.com";
    let mut vec = Vec::new();
    //let f = reqwest::get(url);
    //let f = f.and_then::<reqwest::Result<reqwest::Response>>(|x| async move {x});
    let mut f = reqwest::Client::builder().build()?.get(url).send();
    //let f = f.and_then(|x| async move {Ok(x)});

    let f = HttpTask {
        url: url,
        fut: RefCell::new(f),
        proc: Some(Box::new(|r| {}))
    };
    
    vec.push(f);
    //let f = reqwest::get("http://www.baidu.com");
    let mut f = reqwest::Client::builder().build()?.get("http://www.baidu.com").send();
    //let f = f.and_then(|x| async move {Ok(x)});
    Ok(())
}


struct HttpTask<F: Future> {
    url: &'static str,
    fut: RefCell<F>,
    proc: Option<Box<dyn Fn(reqwest::Response)>>,
}

async fn HTTP3() -> Result<(), Box<dyn std::error::Error>> {

    let url = "http://www.taobao.com";
    let mut vec = Vec::new();
    
    let mut f = reqwest::Client::builder().build()?.get(url).send();    
    let f = HttpTask {
        url: url,
        fut: RefCell::new(f),
        proc: Some(Box::new(|r| { 
            println!("pppp ccaaaooo lllll : {:?}", r);
        }))
    };
    
    vec.push(f);

    /*
    let mut f = reqwest::Client::builder().build()?.get("http://www.baidu.com").send();    
    let f = HttpTask {
        url: url,
        fut: RefCell::new(f),
        proc: None
    };
    
    vec.push(f);
    */

    /*
    let mut f = reqwest::Client::builder().build()?.get("https://openapi.xxx.com/v2/time").send();
    let f = HttpTask {
        url: url,
        fut: f,
        proc: None
    };
    vec.push(f);
    */

    loop {
        let mut i = 0;
        let mut d = false;

        if vec.is_empty() {
            println!("vec is empty...");
            break;
        }
    
        for item in vec.iter() {
            let mut fut = &mut *item.fut.borrow_mut();
            let proc = &item.proc;
            
            let poll = std::future::poll_with_tls_context(
                unsafe { std::pin::Pin::new_unchecked(fut)});
            
            match poll {
                core::task::Poll::Ready(r) => {
                    println!("{:?} is ready: {:?}", i, r);
                    r.map(|x| {
                        println!("heeelllll {:?}", x);
                        if let Some(p) = proc {
                            p(x);
                        }
                    });
                    d = true;
                    break
                }
                core::task::Poll::Pending => 
                    println!("{:?} pending.....", i)
            };
            
            i += 1;
        }
        if d {
            vec.remove(i);
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    Ok(())
}


#[derive(Debug)]
struct Test1 {
    a: String
}

struct Test2 {
    test1: Option<Test1>
}

fn test_use_as_ref() {
    let a = Test2 {
        test1: Some(Test1 {a: String::from("aaaa")})
    };

    if let Some(tt) = a.test1.as_ref() {
        println!("hahaah {:?}", tt);
    }

    println!("hahaah {:?}", a.test1);
    a.test1.map(|x| println!("ggggg {:?}", x));
}

pub fn backup_test_http() {
    //tokio::runtime::Runtime::new().unwrap().block_on(HTTP3());
    //tokio::runtime::Runtime::new().unwrap().block_on(HTTP_join_all());
    //HTTP1();
    tokio::runtime::Runtime::new().unwrap().block_on(HTTP1_1());
    //tokio::runtime::Runtime::new().unwrap().block_on(HTTP3());
}


