use chrono::Utc;
use jieba_rs::Jieba;
use salvo::logging::Logger;
use salvo::prelude::*;
use salvo::IntoVecString;
use serde::Serialize;

#[handler]
async fn hello(_req: &mut Request, _depot: &mut Depot, res: &mut Response, _ctrl: &mut FlowCtrl) {
    let query = _req.query::<String>("q");
    println!("{:?}", query);
    let mut user = Words {
        code: 503,
        message: "failed params is empty".to_string(),
        time: Utc::now().timestamp_micros().to_string(),
        word: Vec::new(),
        trace_id: "1234567890".to_string(),
    };
    if query.is_none() {
        res.render(Json(user));
        return;
    }
    let jieba = Jieba::new();
    let words = jieba.cut(query.as_deref().unwrap_or(""), false);
    user.code = 200;
    user.message = "success".to_string();
    user.time = Utc::now().timestamp_micros().to_string();
    user.word = words.into_vec_string();
    user.trace_id = Utc::now().timestamp_nanos_opt().unwrap().to_string();
    res.render(Json(user));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().hoop(Logger::new()).get(hello);
    let acceptor = TcpListener::new("127.0.0.1:5810").bind().await;
    Server::new(acceptor).serve(router).await;
}

#[derive(Serialize, Debug)]
struct Words {
    code: i32,
    message: String,
    time: String,
    word: Vec<String>,
    trace_id: String,
}
