#[derive(Debug)]
struct HttpRequest {
    method: Method,
    route: Route,
    version: Version,
    headers: HttpHeader
    request_body: String
}

#[derive(Debug)]
struct HttpHeader{
    headers: HashMap<String, String>
}

#[derive(Debug)]
struct Route {
    path: String
}

#[derive(Debug)]
enum Method {
    Get,
    Post,
    Uninitialized
}

#[derive(Debug)]
enum Version {
    V1_1,
    V2_0
}