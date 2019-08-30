use simple_server::{Request, ResponseBuilder, ResponseResult, StatusCode};
use libfonthelper::Fonts;

pub fn handler(request: Request<Vec<u8>>, mut response: ResponseBuilder) ->  ResponseResult {
    let dirs = vec![
        String::from("/usr/share/fonts"),
        String::from("/home/ruut/.local/share/fonts"),
    ];

    match Fonts::new(&dirs) {
        Err(err) => {
            eprintln!("{:?}", err);
            response.status(StatusCode::INTERNAL_SERVER_ERROR);
            Ok(response.body("Failed get fonts".as_bytes().to_vec())?)
        },
        Ok(fonts) => {
            let mut json = "{\"version\": 4,\"fontFiles\":".to_string();
            json.push_str(&fonts.to_json());
            json.push_str("}");
            Ok(response
                .header("Access-Control-Allow-Origin", "https://www.figma.com")
                .header("Content-Type", "application/json")
                .header("Content-Length", json.bytes().len())
                .body(json.as_bytes().to_vec())?)
        }
    }
}