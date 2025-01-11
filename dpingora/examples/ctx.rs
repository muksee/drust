use std::sync::Mutex;

use pingora::{
    http::RequestHeader,
    proxy::{
        http_proxy_service,
        ProxyHttp,
        Session,
    },
    server::{
        configuration::Opt,
        Server,
    },
    upstreams::peer::HttpPeer,
    Result,
};
use structopt::StructOpt;

#[derive(Default)]
pub struct MyProxy {
    beta_counter: Mutex<usize>,
}

impl MyProxy {
    pub fn new() -> Self {
        MyProxy {
            beta_counter: Mutex::new(0),
        }
    }
}

pub struct MyCTX {
    beta_user: bool,
}

fn check_beta_user(req: &RequestHeader) -> bool {
    req.headers
        .get("beta-flag")
        .is_some()
}

#[async_trait::async_trait]
impl ProxyHttp for MyProxy {
    type CTX = MyCTX;

    fn new_ctx(&self) -> Self::CTX {
        MyCTX { beta_user: false }
    }

    async fn request_filter(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<bool> {
        ctx.beta_user = check_beta_user(session.req_header());
        Ok(false)
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let addr = if ctx.beta_user {
            ("44.217.66.36", 443)
        } else {
            ("httpbin.org", 443)
        };

        println!("{:?}", addr);

        Ok(Box::new(HttpPeer::new(addr, true, "httpbin.org".to_string())))
    }
}

fn main() {
    let mut my_server = Server::new(Some(Opt::from_args())).unwrap();
    my_server.bootstrap();

    let mut my_service =
        http_proxy_service(&my_server.configuration, MyProxy::new());
    my_service.add_tcp("0.0.0.0:1080");

    my_server.add_service(my_service);
    my_server.run_forever();
}
