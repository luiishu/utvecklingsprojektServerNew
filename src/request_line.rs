use server::get_request_line;

pub struct RequestLine {
    pub request_line: String,
    pub method: String,
    pub uri: String,
    pub uri_file_type: String,
}

impl RequestLine {
    pub fn new(request: &str) -> RequestLine {
        let request_line = get_request_line(request);
        let method = RequestLine::get_request_method(&request_line);
        let uri = RequestLine::get_request_uri(&request_line);
        let uri_file_type = RequestLine::get_request_uri_file_type(&uri);    
        RequestLine {request_line, method, uri, uri_file_type}
    }

    pub fn get_request_method(request_line: &str) -> String {
        request_line.split(" ").next().unwrap().to_string()
    }

    pub fn get_request_uri(request_line: &str) -> String {
        println!("Request line from get_request_uri: {}", request_line);
        let request_line_vector: Vec<_> = request_line.split(" ").collect();
        request_line_vector[1].to_string() // this will panic when an empty request is received
    }

    pub fn get_request_uri_without_first_slash(&self) -> String {
        //let request_line_vector: Vec<_> = request_line.split(" ").collect();
        let mut uri_without_slash = String::from(&self.uri);
        uri_without_slash.remove(0);
        uri_without_slash
    }

    pub fn get_request_uri_file_type(request_uri: &str) -> String {
        request_uri.split(".").last().unwrap().to_string()
    }
}

impl ToString for RequestLine {
    fn to_string(&self) -> String {
        (&self.request_line).to_string()
    }
}