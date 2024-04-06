use pingora::services::listening::Service;

use crate::app::echo::{
    EchoApp,
    HttpEchoApp,
};

pub fn echo_service() -> Service<EchoApp> {
    Service::new("Echo Service".into(), EchoApp::new())
}
pub fn echo_service_http() -> Service<HttpEchoApp> {
    Service::new("Echo Service HTTP".into(), HttpEchoApp::new())
}
