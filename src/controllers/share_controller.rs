use crate::db::share::insert_share_data;
use crate::models::page_data::ApiResponse;
use crate::{db::share::fetch_share_data_by_address, models::ShareDataTable};

use crate::db::pages_db::{
     delete_all_data, delete_all_user_files, delete_page_data,
    fetch_all_pages_data_per_user, fetch_page_data, insert_page_data, update_page_data,
};
use crate::models::share_data::{CreateShareData, ShareDataResponse};
use crate::util::{
    check_if_page_data_revision_are_okay, check_or_generate_domain, compute_content_hash,
    get_content_type, get_file_info, make_empty_hash, update_env_file,
};
use crate::Db;
use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart, Path, Request, State},
    handler::HandlerWithoutStateExt,
    http::{HeaderMap, StatusCode},
    response::{Html, Redirect},
    routing::{get, post},
    BoxError, Form, Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing_subscriber::fmt::format;

// Handler with path and query parameters combined
pub async fn get_share_data(
    State(server_database): State<Db>,
    Path(share_identifier): Path<String>,
) -> (StatusCode, Json<ShareDataResponse>) {
    let mut log_data: Vec<String> = Vec::new();
    let mut res: ShareDataResponse = ShareDataResponse {
        logs: log_data.clone(),
        share_data: None,
        file_data: None,
    };

    if share_identifier.len() == 0 {
        res.logs.push(format!("Error: identifier not found "));
        return (StatusCode::BAD_REQUEST, Json(res));
    }
    println!("share_identifier provided {}", share_identifier);

    let mut conn = match server_database.pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            // error!("Failed to get database connection: {}", e);
            log_data.push("Failed to get database connection".to_string());
            println!("Error Fetching connection {:#?}", res);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    let share_payload = fetch_share_data_by_address(&share_identifier.as_str(), &mut conn);

    if share_payload.is_err() {
        log_data.push("Error fetching share data from system".to_string());
        return (StatusCode::NOT_FOUND, Json(res));
    }

    let share_payload_data = share_payload.unwrap();

    if share_payload_data.len() == 0 {
        log_data.push("Error  share data not found in  system".to_string());
        return (StatusCode::NOT_FOUND, Json(res));
    }

    let firs_share_payload_data = share_payload_data.first().unwrap();


    if firs_share_payload_data.id.is_none(){
        log_data.push("Error  id not found in system".to_string());
        return (StatusCode::NOT_FOUND, Json(res));  
    }

    let page_data_result = fetch_page_data(firs_share_payload_data.id.unwrap(), &mut conn);

    if page_data_result.is_err() {
        tracing::error!("Failed not found ",);

        res.logs
            .push("Failed data not found in database".to_string());

        return (StatusCode::NOT_FOUND, Json(res));
    }
    let page_data = page_data_result.unwrap();

    res.file_data = Some(page_data);
    res.share_data = Some(firs_share_payload_data.clone());

    return (StatusCode::OK, Json(res));
}

pub async fn save_share_data(
    State(server_database): State<Db>,
    Form(input): Form<CreateShareData>,
) -> (StatusCode, Json<ApiResponse>) {
    tracing::debug!("explorer_delete_file");
    let mut log_data: Vec<String> = Vec::new();

    let mut res: ApiResponse = ApiResponse {
        logs: log_data,
        file: None,
        files: Vec::new(),
    };

    // Get the name parameter from the input
    if input.file_id == 0  {
        res.logs.push("Error : file name is empty".to_string());

        return (StatusCode::BAD_REQUEST, Json(res));
    };

    if input.identifier.is_empty() {
        res.logs.push("Error : file name is empty".to_string());

        return (StatusCode::BAD_REQUEST, Json(res));
    };

    let mut conn = match server_database.pool.get() {
        Ok(connection) => connection,
        Err(e) => {
          res.logs.push("Failed data not found in database".to_string());
          res.logs.push("Failed to get database connection".to_string());
            // let res: ApiResponse = ApiResponse {
            //     logs: log_data,
            //     file: None,
            //     files: Vec::new(),
            // };

            println!("Error Fetching connection {:#?}", res);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    // confirm if page data exist

    let page_data_result = fetch_page_data(input.file_id.clone(), &mut conn);

    if page_data_result.is_err() {
        tracing::error!("Failed not found ",);

        res.logs
            .push("Failed data not found in database".to_string());

        return (StatusCode::NOT_FOUND, Json(res));
    }
    
    // 1. Get current UTC time as a DateTime<Utc>
    let current_utc = Utc::now();
    let time_data = current_utc.format("%Y-%m-%d %H:%M:%S UTC");
    let time_data_str = format!("{:?}", time_data);
    println!("Custom format 1: {} str {}", time_data, time_data_str);
  
    // insert share data to db
    let share_payload = ShareDataTable {
        id: None,
        file_id: input.file_id.clone(),
        identifier: input.identifier.clone(),
        created_time: time_data_str,
    };

    let insert_result =  insert_share_data(share_payload,&mut conn);

    if insert_result.is_err(){
        
        res.logs
            .push(format!("insert error  {:#?}",insert_result.err() ));

        return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
    }

    let mut page_data =  page_data_result.unwrap();
    page_data.is_shared = true;

    // update db file is  shared 
    let  update_result =  update_page_data(page_data, & mut conn);

    if update_result.is_err(){
        
        res.logs
            .push(format!("error updating system  {:#?}",update_result.err() ));

        return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
    }

    return (StatusCode::OK, Json(res));
}
