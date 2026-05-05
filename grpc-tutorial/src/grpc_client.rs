use services::chat_service_client::ChatServiceClient;
use services::ChatMessage;
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use services::transaction_service_client::TransactionServiceClient;
use services::TransactionRequest;
use tonic::Request;

pub mod services {
    tonic::include_proto!("services");
}

use services::payment_service_client::PaymentServiceClient;
use services::PaymentRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = PaymentServiceClient::connect("http://[::1]:50051").await?;

    let request = Request::new(PaymentRequest {
        user_id: "user_1".to_string(),
        amount: 500.0,
    });

    let response = client.process_payment(request).await?;

    println!("✅ Response: {:?}", response.into_inner());

    let mut client = TransactionServiceClient::connect("http://[::1]:50051").await?;

    let request = Request::new(TransactionRequest {
        user_id: "user_1".to_string(),
    });

    let mut stream = client.get_transaction_history(request).await?.into_inner();

    println!("📥 Transaction History:");

    while let Some(response) = stream.message().await? {
        println!("➡️ {:?}", response);
    }

    let mut client = ChatServiceClient::connect("http://[::1]:50051").await?;

    let (tx, rx) = mpsc::channel(4);

    tokio::spawn(async move {
        let messages = vec![
            "Halo",
            "Ini pesan kedua",
            "Testing streaming",
        ];

        for msg in messages {
            let chat_msg = ChatMessage {
                user_id: "client".to_string(),
                message: msg.to_string(),
            };

            tx.send(chat_msg).await.unwrap();
        }
    });

    let response = client.chat(ReceiverStream::new(rx)).await?;
    let mut inbound = response.into_inner();

    println!("📨 Chat dimulai:");

    while let Some(msg) = inbound.message().await? {
        println!("⬅️ {}: {}", msg.user_id, msg.message);
    }

    Ok(())
}