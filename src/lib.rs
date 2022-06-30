pub mod configuration;
pub mod startup;
use pb::{
    news_service_server::{NewsService, NewsServiceServer},
    ListNewsResponse, FILE_DESCRIPTOR_SET,
};
use tonic::{
    transport::{server::Router, Server},
    Request, Response, Status,
};

pub mod pb {
    tonic::include_proto!("kb.v1");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("kb_descriptor");
}
#[derive(Debug, Default)]
pub struct NewsServicePb {}

#[tonic::async_trait]
impl NewsService for NewsServicePb {
    async fn list_news(
        &self,
        request: Request<pb::ListNewsRequest>,
    ) -> Result<Response<ListNewsResponse>, Status> {
        let news = vec![pb::News {
            title: "test".into(),
            contents: "cont".into(),
        }];
        Ok(Response::new(ListNewsResponse { news }))
    }
}

pub fn run() -> Result<Router, Box<dyn std::error::Error>> {
    let news = NewsServicePb::default();
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let server = Server::builder()
        .add_service(NewsServiceServer::new(news))
        .add_service(reflection);
    Ok(server)
}
