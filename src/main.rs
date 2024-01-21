// Run with "cargo run ."

use axum::{extract::Multipart, response::Html, routing::get, Router};
use tower_http::limit::RequestBodyLimitLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(show_form).post(accept_form))
        .layer(RequestBodyLimitLayer::new(1 * 1024 * 1024 /* 1 MB */));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head>
                <link href="https://releases.transloadit.com/uppy/v3.21.0/uppy.min.css" rel="stylesheet">
            </head>
            
            <body>
                <div id="uppy"></div>

                <br><br>

                <form action="/" method="post" enctype="multipart/form-data">
                    <label>
                        Upload file:
                        <input type="file" name="file" multiple>
                    </label>

                    <input type="submit" value="Upload files">
                </form>

                <script type="module">
                    import { Uppy, XHRUpload, Dashboard } from "https://releases.transloadit.com/uppy/v3.21.0/uppy.min.mjs"
                    
                    const uppy = new Uppy()
                        .use(XHRUpload, {
                            endpoint: "/",
                            bundle: true
                        })
                        .on('upload-success', (info) => {
                            console.log("upload-success, info:", info);
                        });

                    uppy.use(Dashboard, { target: '#uppy', inline: true })
                </script>
            </body>
        </html>
        "#,
    )
}

async fn accept_form(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes",
            data.len()
        );
    }
}
