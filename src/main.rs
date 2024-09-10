#![allow(warnings)]

use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart, Path, Request, State},
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    response::{Html, Redirect},
    routing::{get, post},
    BoxError, Form, Router,
};

use chrono::{NaiveDateTime, Utc};

use futures::{Stream, TryStreamExt};
use sha3::*;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::net::SocketAddr;
use std::time::SystemTime;
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;
use tower::ServiceExt;
use tower_http::{
    limit::RequestBodyLimitLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use bonsaidb::core::keyvalue::{KeyStatus, KeyValue};
use bonsaidb::core::schema::{Collection, SerializedCollection};
use bonsaidb::local::config::{Builder, StorageConfiguration};
use bonsaidb::local::Database;
use serde::{Deserialize, Serialize};

use guardian_common::{crypt, custom_types::*};
use verifier::v1_1::hashes::*;

const UPLOADS_DIRECTORY: &str = "uploads";

#[derive(Debug, Serialize, Deserialize, Collection, Clone)]
#[collection(name = "page")]
pub struct PageData {
    pub pages: Vec<HashChain>,
}

#[derive(Debug, Clone)]
struct Db {
    pub db: Database,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let db = Database::open::<PageData>(StorageConfiguration::new("b0nsa1.bonsaidb")).unwrap();

    // save files to a separate directory to not override files in the current directory
    tokio::fs::create_dir(UPLOADS_DIRECTORY).await;

    let server_database = Db { db: db };

    let app = Router::new()
        .route("/", get(show_form).post(save_request_body))
        .route("/json", post(save_json_file))
        //.route("/list", get(show_files_list).post(show_files))
        .with_state(server_database);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3600")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    filename: String,
}

// async fn accept_form(Form(input): Form<Input>) {
//     dbg!(&input);
// }

async fn save_json_file(
    State(server_database): State<Db>,
    Form(input): Form<Input>,
) -> Result<Redirect, (StatusCode, String)> {
    println!("{:#?}", &input);

    let document2: &Option<PageData> = &server_database.db.get_key(&input.filename).into().unwrap();

    if document2.is_some() {
        let doc: PageData = document2.clone().unwrap();
        let (_, rev1) = &doc.pages[0].revisions[0];

        if rev1.content.file.is_some() {
            let file: FileContent = rev1.content.file.clone().unwrap();
            let vu8 = file.data.to_vec();
            // println!("{:#?}", String::from_utf8(vu8));
        }

        fs::write(
            "./export/".to_owned() + &input.filename + ".aqua.json",
            serde_json::to_string(&doc).unwrap(),
        );
    }
    Ok(Redirect::to("/json"))
}

async fn save_request_body(
    State(server_database): State<Db>,
    mut multipart: Multipart,
) -> Result<Redirect, (StatusCode, String)> {
    tracing::debug!("yay2");

    while let Some(field) = multipart.next_field().await.unwrap() {
        // println!("{:#?}", field);

        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let body_bytes = field.bytes().await.unwrap().to_vec();

        // 4.b add rev.metadata.domain_id to hasher {m}

        let b64 = Base64::from(body_bytes);
        let mut file_hasher = sha3::Sha3_512::default();
        file_hasher.update(b64.clone());
        let file_hash_current = Hash::from(file_hasher.finalize());

        let mut content_current = BTreeMap::new();

        content_current.insert("file_hash".to_owned(), file_hash_current.to_string());

        println!("{:#?}", content_current);

        let content_hash_current = content_hash(&content_current.clone());

        println!("{:#?}", content_hash_current);

        let domain_id_current = "0".to_owned();
        let timestamp_current = Timestamp::from(chrono::NaiveDateTime::from_timestamp(
            Utc::now().timestamp(),
            0,
        ));

        let metadata_hash_current =
            metadata_hash(&domain_id_current, &timestamp_current.clone(), None);

        let verification_hash_current =
            verification_hash(&content_hash_current, &metadata_hash_current, None, None);

        let pagedata_current = &PageData {
            pages: vec![HashChain {
                genesis_hash: verification_hash_current.clone().to_string(),
                domain_id: domain_id_current,
                title: file_name.clone(),
                namespace: 0,
                chain_height: 0,
                revisions: vec![(
                    verification_hash_current,
                    Revision {
                        content: RevisionContent {
                            file: Some(FileContent {
                                data: b64,
                                filename: file_name.clone(),
                                size: 0,
                                comment: "".to_string(),
                            }),
                            content: content_current,
                            content_hash: content_hash_current,
                        },
                        metadata: RevisionMetadata {
                            domain_id: "0".to_string(),
                            time_stamp: timestamp_current,
                            previous_verification_hash: None,
                            metadata_hash: metadata_hash_current,
                            verification_hash: verification_hash_current,
                        },
                        signature: None,
                        witness: None,
                    },
                )],
            }],
        };

        let document = &server_database
            .db
            .set_key(file_name.clone(), &pagedata_current)
            //            .only_if_vacant()
            .execute()
            .unwrap();

        let document2: &Option<PageData> = &server_database.db.get_key(file_name).into().unwrap();

        if document2.is_some() {
            let doc: PageData = document2.clone().unwrap();
            let (_, rev1) = &doc.pages[0].revisions[0];

            if rev1.content.file.is_some() {
                let file: FileContent = rev1.content.file.clone().unwrap();
                let vu8 = file.data.to_vec();
                // println!("{:#?}", String::from_utf8(vu8));
            }

            //            fs::remove_file("../aqua-verifier-js/vef.json").unwrap();
            //            fs::write(
            //                "../aqua-verifier-js/vef.json",
            //                serde_json::to_string(&doc).unwrap(),
            //            );
        }
    }

    Ok(Redirect::to("/"))
}

// Handler that returns HTML for a multipart form.
async fn show_form() -> Html<&'static str> {
    tracing::debug!("yay");
    println!("yay");
    Html(
        r#"
        <!doctype html>
        <html>
            <head>
                <title>Upload something!</title>
                <script src="https://cdn.ethers.io/lib/ethers-5.6.4.umd.min.js" type="application/javascript"></script>
                    <script>
                        function web3_check_metamask() {
                            if (!window.ethereum) {
                                console.error('It seems that the MetaMask extension is not detected. Please install MetaMask first.');
                                alert('It seems that the MetaMask extension is not detected. Please install MetaMask first.');
                                return false;
                            }else{
                                console.log('MetaMask extension has been detected!!');
                                return true;
                            }
                        }
                        
                        function web3_metamask_hash(){
                            var hashed_string   = '';
                            var chars           = '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz';
                            var total_chars     = chars.length;
                            for ( var i = 0; i < 256; i++ ) {
                                hashed_string += chars.charAt(Math.floor(Math.random() * total_chars));
                            }
                            return hashed_string;                
                        }
                        
                        async function web3_metamask_login() {
                            // Check first if the user has the MetaMask installed
                            if ( web3_check_metamask() ) {
                                console.log('Initate Login Process');
            
                                // Get the Ethereum provider
                                const provider = new ethers.providers.Web3Provider(window.ethereum);                    
                                // Get Ethereum accounts
                                await provider.send("eth_requestAccounts", []);
                                console.log("Connected!!"); 
                                // Get the User Ethereum address
                                const address = await provider.getSigner().getAddress();
                                console.log(address);      
            
                                // Create hashed string 
                                const hashed_string = web3_metamask_hash();      
                                console.log("Hashed string: " + hashed_string);   
                                // Request the user to sign it
                                const signature = await provider.getSigner().signMessage(hashed_string);
                                // Got the signature
                                console.log("The signature: " + signature);   
            
                                // TODO
                                // you can then send the signature to the webserver for further processing and verification 
                            }
                        }              
                    </script>
            </head>
            <body>
                <form action="/" method="post" enctype="multipart/form-data">
                    <div>
                        <label>
                            Upload file:
                            <input type="file" name="file" multiple>
                        </label>
                    </div>

                    <div>
                        <input type="submit" value="Upload files">
                    </div>
                </form>
                <hr> 
                <form action="/json" method="post">
                    <div>
                        <label>
                            <input type="text" name="filename">
                        </label>
                    </div>
                    <div>
                        <input type="submit" value="Download aquachain for file">
                    </div>
                </form>
                <hr> 
                <p>Check first if MetaMask is installed: <a href='#!' onclick='web3_check_metamask();'>Detect MetaMask</a></p>
                <p>Initate the Login process: <a href='#!' onclick='web3_metamask_login();'>Login with MetaMask</a></p>
            </body>
        </html>
        "#,
    )
}

// Handler that accepts a multipart form upload and streams each field to a file.
async fn accept_form(mut multipart: Multipart) -> Result<Redirect, (StatusCode, String)> {
    while let Ok(Some(field)) = multipart.next_field().await {
        let file_name = if let Some(file_name) = field.file_name() {
            file_name.to_owned()
        } else {
            continue;
        };

        stream_to_file(&file_name, field).await?;
    }

    Ok(Redirect::to("/"))
}

// Save a `Stream` to a file
async fn stream_to_file<S, E>(path: &str, stream: S) -> Result<(), (StatusCode, String)>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    if !path_is_valid(path) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }

    async {
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        // Create the file. `File` implements `AsyncWrite`.
        let path = std::path::Path::new(UPLOADS_DIRECTORY).join(path);
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, io::Error>(())
    }
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

// to prevent directory traversal attacks we ensure the path consists of exactly one normal
// component
fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}
