use actix_web::{HttpResponse, web::Data};

pub async fn get_tx_data(bundlers: Data<Vec<String>>) -> actix_web::Result<HttpResponse> {
    
}