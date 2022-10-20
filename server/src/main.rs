use tonic::{transport::Server, Request, Response, Status};
use std::time::SystemTime;
use uuid::Uuid;

pub mod grpc_page {
    tonic::include_proto!("page");
}

use grpc_page::page_server::{Page, PageServer};
use grpc_page::{PageRequest, PageReply, Block, ParagraphBlock, ParagraphData, HeaderBlock, HeaderData};

#[derive(Debug, Default)]
pub struct MyPage {}

#[tonic::async_trait]
impl Page for MyPage {
    async fn get_page(
        &self,
        request: Request<PageRequest>,
    ) -> Result<Response<PageReply>, Status> {
        let reply = match request.into_inner().name.as_str() {
            "home" => PageReply { 
                name: "home".into(),
                time: Some(SystemTime::now().into()),
                blocks: vec![
                    Block { block: Some(grpc_page::block::Block::HeaderBlock(HeaderBlock {
                        id: Uuid::new_v4().to_string(),
                        r#type: "header".into(),
                        data: Some(HeaderData {
                            text: "Data Engineer, Problem Solver".into(),
                            level: 2
                        })
                    }))},
                    Block { block: Some(grpc_page::block::Block::ParagraphBlock(ParagraphBlock {
                        id: Uuid::new_v4().to_string(),
                        r#type: "paragraph".into(),
                        data: Some(ParagraphData {
                            text: "<span class=\"highlight\">Data and processes accompany me through my entire professional life. As an expert in data and processes, especially in supply chain management, production and their interfaces, who speaks both the technical and the business language and can interpret in between, I contribute strongly to the understanding and better communication of problems.</span>".into()
                        })
                    }))}
                ],
                version: "0.1.0".into()
            },
            "impressum" => PageReply { 
                name: "impressum".into(),
                time: Some(SystemTime::now().into()),
                blocks: vec![
                    Block { block: Some(grpc_page::block::Block::HeaderBlock(HeaderBlock {
                        id: Uuid::new_v4().to_string(),
                        r#type: "header".into(),
                        data: Some(HeaderData {
                            text: "Angaben gemäß §5 TMG".into(),
                            level: 2
                        })
                    }))},
                    Block { block: Some(grpc_page::block::Block::ParagraphBlock(ParagraphBlock {
                        id: Uuid::new_v4().to_string(),
                        r#type: "paragraph".into(),
                        data: Some(ParagraphData {
                            text: "Christopher Scholz<br>An der Dahme 3<br>12527 Berlin".into()
                        })
                    }))},
                    Block { block: Some(grpc_page::block::Block::HeaderBlock(HeaderBlock {
                        id: Uuid::new_v4().to_string(),
                        r#type: "header".into(),
                        data: Some(HeaderData {
                            text: "Kontakt".into(),
                            level: 2
                        })
                    }))},
                    Block { block: Some(grpc_page::block::Block::ParagraphBlock(ParagraphBlock {
                        id: Uuid::new_v4().to_string(),
                        r#type: "paragraph".into(),
                        data: Some(ParagraphData {
                            text: "Email: <a href=\"mailto:website@christopher-scholz.com\">website@christopher-scholz.com</a>".into()
                        })
                    }))}
                ],
                version: "0.1.0".into()
            },
            _ => PageReply { 
                name: "home".into(),
                time: Some(SystemTime::now().into()),
                blocks: vec![
                    Block { block: Some(grpc_page::block::Block::HeaderBlock(HeaderBlock {
                        id: Uuid::new_v4().to_string(),
                        r#type: "header".into(),
                        data: Some(HeaderData {
                            text: "Unknown".into(),
                            level: 2
                        })
                    }))},
                    Block { block: Some(grpc_page::block::Block::ParagraphBlock(ParagraphBlock {
                        id: Uuid::new_v4().to_string(),
                        r#type: "paragraph".into(),
                        data: Some(ParagraphData {
                            text: "This page is not known.".into()
                        })
                    }))}
                ],
                version: "0.1.0".into()
            },
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8000".parse().unwrap();
    let page = MyPage::default();
    let page = PageServer::new(page);
    let page = tonic_web::config()
        .allow_all_origins()
        .enable(page);

    println!("listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .add_service(page)
        .serve(addr)
        .await?;

    Ok(())
}