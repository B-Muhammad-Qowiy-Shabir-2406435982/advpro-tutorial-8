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

    Ok(())
}