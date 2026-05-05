use services::transaction_service_server::{TransactionService, TransactionServiceServer};
use services::{TransactionRequest, TransactionResponse};
use services::chat_service_server::{ChatService, ChatServiceServer};
use services::ChatMessage;
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

pub mod services {
    tonic::include_proto!("services");
}

use services::payment_service_server::{PaymentService, PaymentServiceServer};
use services::{PaymentRequest, PaymentResponse};

#[derive(Default)]
pub struct MyPaymentService;

#[tonic::async_trait]
impl PaymentService for MyPaymentService {
    async fn process_payment(
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {

        let req = request.into_inner();

        println!("💰 Payment dari user: {}", req.user_id);

        let response = PaymentResponse {
            success: true,
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr = "[::1]:50051".parse()?;
    let service = MyPaymentService::default();

    println!("🚀 Server jalan di {}", addr);

    Server::builder()
        .add_service(PaymentServiceServer::new(MyPaymentService::default()))
        .add_service(TransactionServiceServer::new(MyTransactionService::default()))
        .add_service(ChatServiceServer::new(MyChatService::default()))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Default)]
pub struct MyTransactionService;

#[tonic::async_trait]
impl TransactionService for MyTransactionService {

    type GetTransactionHistoryStream = ReceiverStream<Result<TransactionResponse, Status>>;

    async fn get_transaction_history(
        &self,
        request: Request<TransactionRequest>,
    ) -> Result<Response<Self::GetTransactionHistoryStream>, Status> {

        let req = request.into_inner();
        println!("📜 Ambil transaksi user: {}", req.user_id);

        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            let transactions = vec![
                ("trx1", "SUCCESS", 100.0),
                ("trx2", "FAILED", 50.0),
                ("trx3", "SUCCESS", 200.0),
            ];

            for (id, status, amount) in transactions {
                let response = TransactionResponse {
                    transaction_id: id.to_string(),
                    status: status.to_string(),
                    amount,
                    timestamp: "2025-01-01".to_string(),
                };

                tx.send(Ok(response)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}   

#[derive(Default)]
pub struct MyChatService;

#[tonic::async_trait]
impl ChatService for MyChatService {

    type ChatStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn chat(
        &self,
        request: Request<tonic::Streaming<ChatMessage>>,
    ) -> Result<Response<Self::ChatStream>, Status> {

        let mut stream = request.into_inner();

        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            while let Some(msg) = stream.message().await.unwrap() {
                println!("💬 Dari {}: {}", msg.user_id, msg.message);

                let reply = ChatMessage {
                    user_id: "server".to_string(),
                    message: format!("Echo: {}", msg.message),
                };

                tx.send(Ok(reply)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}