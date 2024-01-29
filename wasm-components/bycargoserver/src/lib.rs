mod bindings;

use bindings::{
    exports::wasi::http::incoming_handler::{Guest, IncomingRequest, ResponseOutparam},
    wasi::http::types::{Headers, OutgoingResponse},
};

struct Component;

impl Guest for Component {
    fn handle(request: IncomingRequest, output: ResponseOutparam) {
        let path = request.path_with_query().unwrap();
        let headers = Headers::new();
        let response: OutgoingResponse = OutgoingResponse::new(headers);
        if path.contains("bad") {
            OutgoingResponse::set_status_code(&response, 500).unwrap();
        } else {
            OutgoingResponse::set_status_code(&response, 204).unwrap();
        }
        ResponseOutparam::set(output, Ok(response));
    }
}
