use {
    actix_web::HttpResponse,
    actix_web::web::{Data, Json, Path},
    uuid::Uuid,

    crate::DBPool,
    crate::miner::*,
    crate::util::{NotFoundMessage, ResponseType},
    
};


// List all Miners
#[get("/miners")]
pub async fn list_miners(pool: Data<DBPool>) -> HttpResponse {
    let mut conn = crate::get_connection_to_pool(pool);
    let miners: Vec<Miner> = fetch_all_miners(&mut conn);
    ResponseType::Ok(miners).get_response()
}

// Get a miner with the corresponding id
#[get("/miners/{id}")]
pub async fn get_miner(path: Path<String>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = crate::get_connection_to_pool(pool);
    let miner_id = path.into_inner();
    let miner: Option<Miner> = fetch_miner_by_id(
        Uuid::parse_str(miner_id.as_str()).unwrap(), &mut conn);
    //let miner: Option<Miner> = fetch_miner_by_id(
      //  Uuid::parse_str(path.0.0.as_str()).unwrap(), &mut conn);
    match miner {
        Some(miner) => ResponseType::Ok(miner).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Miner not found.".to_string())
        ).get_response(),
    }
}

// Create new Miner
#[post("/wallets/{id}/miners")]
pub async fn create_miner(path: Path<(String,String)>, 
                        miner_request: Json<NewMinerRequest>, 
                        pool: Data<DBPool>) -> HttpResponse {
    let (miner_id, miners) = path.into_inner();                        
    let mut conn = crate::get_connection_to_pool(pool);
    match create_new_miner(miner_request.0, Uuid::parse_str(miner_id.as_str()).unwrap(), &mut conn) {
        Ok(created_miner) => ResponseType::Created(created_miner).get_response(),
        Err(_) => ResponseType::NotFound(
            NotFoundMessage::new("Error creating miner.".to_string())
        ).get_response(),
    }
}