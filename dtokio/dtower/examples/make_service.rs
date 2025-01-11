use tokio;
#[tokio::main]
async fn main() {}

#[cfg(test)]
mod test {
    use std::convert::Infallible;

    use tower::service_fn;
    use tower::MakeService;
    use tower::Service;

    #[tokio::test]
    async fn test_make_service() {
        let make_service = service_fn(|_make_req: ()| async {
            Ok::<_, Infallible>(service_fn(|req: String| async { Ok::<_, Infallible>(req) }))
        });

        let mut inner = make_service.into_service();

        let mut new_service = inner.call(()).await.unwrap();
        let ret = new_service.call(String::from("Hello")).await.unwrap();

        println!("{ret}");
    }
}
