# rust-tcpserver
rust第一课，使用rust std标准库功能实现tcp server

使用了本地terminal的curl发起的请求，请求效果截图在screenshots文件夹中
客户端发起了三次请求，分别是GET、POST、DELETE请求，服务端打印三次请求信息并返回相应页面

1. cd mytcpserver
2. cargo build
3. ./target/mytcpserver