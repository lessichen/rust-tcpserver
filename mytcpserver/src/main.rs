use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

// 借鉴代码，全部看懂进行抽象
fn main() {
    // 1 创建TCP连接监听本地8080端口,unwrap隐含panic处理
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // 2 监听TCP连接
    for stream in listener.incoming() {
        // 2.1 获取tcpStream ,unwrap隐含panic处理
        let mut stream = stream.unwrap();
        // 2.2 打印请求信息
        let req = print_request(&stream);
        // 2.3 服务端处理请求，返回一个Result类型数据
        let result = handle_http_method(req.as_str());
        // 2.4 使用上述Result进行match,如果返回的是<OK, "XXX">返回正确的响应结果，如果返回的是<Err, "XXX"> 返回错误的响应信息
        let resp = match result {
            Ok(m) => {
                print!("request method: {}", m.as_str());
                return_success_response()
            },
            Err(_) => return_error_response()
        };
        // 2.5 将响应信息写入到流中
        stream.write(resp.as_bytes()).unwrap();
        // 2.6 flush刷盘强制直接返回响应，把缓存中的内容直接写进网卡
        stream.flush().unwrap();
    }
}


fn print_request(mut stream: &TcpStream) -> String {
    // 在栈上声明一个 buffer 来存放读取到的数据，创建缓冲区的大小为1024字节
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // 将缓冲区的字节转换为字符串
    let content = String::from_utf8_lossy(&buffer);
    // 打印请求内容
    print!("{}", content);
    // 返回请求信息
    content.to_string()
}

fn return_success_response() -> String {
    // 读取正常响应文件html res_html 变量隐藏
    let res_html = fs::read_to_string("hello.html").unwrap();
    // 生成正常返回信息字符串
    let success_resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        res_html.len(),
        res_html
    );
    success_resp
}

fn return_error_response() -> String {
    // 读取异常响应文件html
    let res_html = fs::read_to_string("error.html").unwrap();
    // 生成异常响应信息字符串
    let err_resp = format!(
        "HTTP/1.1 404 NOTFOUND\r\nContent-Length: {}\r\n\r\n{}",
        res_html.len(),
        res_html
    );
    err_resp
}

fn handle_http_method (content: &str) -> Result<HttpMethod, &str> {
    // 根据请求内容判断HTTP请求method 正确则返回包含HTTPMethod的OK枚举,非GET POST则返回包含错误信息ERR枚举
    if content.starts_with("GET") {
        Result::Ok(HttpMethod::GET)
    } else if content.starts_with("POST") {
        Result::Ok(HttpMethod::POST)
    } else {
        Result::Err("http method not support")
    }
}

enum HttpMethod {
    GET,
    POST
}

impl HttpMethod {
    // 实现as_str方法便于打印
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST"
        }
    }
}