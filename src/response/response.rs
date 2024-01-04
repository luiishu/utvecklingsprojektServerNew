pub trait HttpResponseCodes {
    const OK: usize = 200;
    const CREATED: usize = 201;
    const BAD_REQUEST: usize = 400;
    const NOT_FOUND: usize = 404;
    const CONFLICT: usize = 409;
    const INTERNAL_SERVER_ERROR: usize = 500;
    const VERSION_NOT_SUPPORTED: usize = 505;
}

pub trait HttpResponseMessages {
    const OK: &'static str = "OK";                                          // 200
    const CREATED: &'static str = "CREATED";                                // 201
    const BAD_REQUEST: &'static str = "BAD REQUEST";                        // 400
    const NOT_FOUND: &'static str = "NOT FOUND";                            // 404
    const CONFLICT: &'static str = "CONFLICT";                              // 409
    const INTERNAL_SERVER_ERROR: &'static str = "INTERNAL SERVER ERROR";    // 500
    const VERSION_NOT_SUPPORTED: &'static str = "VERSION NOT SUPPORTED";    // 505
}

pub struct ResponseLine;
pub struct HttpResponseCode;
pub struct HttpResponseMessage;
impl HttpResponseCodes for HttpResponseCode{}
impl HttpResponseMessages for HttpResponseMessage{}

impl ResponseLine {
    pub fn get_response_line(status_code: usize) -> String {
        let status_message = match status_code {
            HttpResponseCode::OK => HttpResponseMessage::OK,
            HttpResponseCode::CREATED => HttpResponseMessage::CREATED,
            HttpResponseCode::BAD_REQUEST => HttpResponseMessage::BAD_REQUEST,
            HttpResponseCode::NOT_FOUND => HttpResponseMessage::NOT_FOUND,
            HttpResponseCode::CONFLICT => HttpResponseMessage::CONFLICT,
            HttpResponseCode::INTERNAL_SERVER_ERROR => HttpResponseMessage::INTERNAL_SERVER_ERROR,
            HttpResponseCode::VERSION_NOT_SUPPORTED => HttpResponseMessage::VERSION_NOT_SUPPORTED,
            _ => HttpResponseMessage::INTERNAL_SERVER_ERROR
        };

        format!("HTTP/1.1 {status_code} {status_message}")
    }
}