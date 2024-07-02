use minimint::minimint_server::{Minimint, MinimintServer};
use minimint::{HelloReply, HelloRequest};
use tonic::{Request, Response, Status};

pub mod minimint {
    tonic::include_proto!("minimint");
}

#[derive(Debug, Default)]
pub struct Server {}

#[tonic::async_trait]
impl Minimint for Server {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = Server::default();

    tonic::transport::Server::builder()
        .add_service(MinimintServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
