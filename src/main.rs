use std::convert::Infallible;

use futures::TryStreamExt;
use warp::multipart::FormData;
use warp::reject::Rejection;
use warp::reply::Reply;
use warp::Filter;
use bytes::BufMut;
use warp::http::StatusCode;

mod pdf2text; 

#[tokio::main]
async fn main() {
    // Running curl -F file=@.gitignore 'localhost:3030/' should print [("file", ".gitignore", "\n/target\n**/*.rs.bk\nCargo.lock\n.idea/\nwarp.iml\n")]
    let upload_route = warp::path("upload")
    .and(warp::post())
    .and(warp::multipart::form().max_length(100_000_000))
    .and_then(upload);

    let router = upload_route.recover(handle_rejection);

    warp::serve(router).run(([127, 0, 0, 1], 3030)).await;
}

async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let field_names: Vec<_> = form
    .and_then(|mut field| async move {
        let mut bytes: Vec<u8> = Vec::new();

        // field.data() only returns a piece of the content, you should call over it until it replies None
        
        println!(" -- Doing stuff {} {} ", field.name(), field.filename().unwrap());
        if field.name().eq("pdf")
        {
            while let Some(content) = field.data().await {
                let content = content.unwrap();
                bytes.put(content);
            }
            
            match pdf2text::pdf2text(&bytes) {
                Ok((doc,lang)) => {
                    println!(" -- Done stuff {} {} {}", field.name(), field.filename().unwrap(), lang.three_letter_code());
                    Ok((
                        field.name().to_string(),
                        field.filename().unwrap().to_string(),
                        lang.three_letter_code(), 
                        doc
                    ))
                },
                Err(error_str) => {
                    Ok((
                        field.name().to_string(),
                        field.filename().unwrap().to_string(),
                        "None", 
                        String::from(error_str)
                    ))
                }
            }

        }
        else
        {
            Ok((
                field.name().to_string(),
                field.filename().unwrap().to_string(),
                "None", 
                String::from("Field: pdf, not found")
            ))
        }


    })
    .try_collect()
    .await
    .unwrap();

    Ok::<_, warp::Rejection>(format!("{:?}", field_names))

    // let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
    //     eprintln!("form error: {}", e);
    //     warp::reject::reject()
    // })?;
    // eprintln!("Parse parts");
    // for p in parts {
    //     if p.name() == "file" {
    //         let content_type = p.content_type();

    //         let file_ending;
    //         match content_type {
    //             Some(file_type) => match file_type {
    //                 "application/pdf" => {
    //                     file_ending = "pdf";
    //                 }
    //                 "image/png" => {
    //                     file_ending = "png";
    //                 }
    //                 v => {
    //                     eprintln!("invalid file type found: {}", v);
    //                     return Err(warp::reject::reject());
    //                 }
    //             },
    //             None => {
    //                 eprintln!("file type could not be determined");
    //                 return Err(warp::reject::reject());
    //             }
    //         }
    //         let value = p
    //             .stream()
    //             .try_fold(Vec::new(), |mut vec, data| {
    //                 vec.put(data);
    //                 async move { Ok(vec) }
    //             })
    //             .await
    //             .map_err(|e| {
    //                 eprintln!("reading file error: {}", e);
    //                 warp::reject::reject()
    //             })?;
    //             let file_name = format!("./files/{}.{}", Uuid::new_v4().to_string(), file_ending);
    //             tokio::fs::write(&file_name, value).await.map_err(|e| {
    //                 eprint!("error writing file: {}", e);
    //                 warp::reject::reject()
    //             })?;
    //             println!("created file: {}", file_name);
    //         }
    //     }
    //     Ok("success")
    }


async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}