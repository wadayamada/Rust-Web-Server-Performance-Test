use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};
use std::convert::Infallible;

// pub async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
//     let person = get_person();
//     let response_text = format!("{}\n{}\n{}", person.name, person.age, person.description);
//     let response = Response::builder()
//         .header(hyper::header::CONTENT_TYPE, "text/plain; charset=UTF-8")
//         .body(Full::new(Bytes::from(response_text)))
//         .unwrap();
//     Ok(response)
// }

pub async fn parse_handler(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let body_text = build_large_response().await;
    println!("{}", body_text);
    let response_text = format!("aaa");
    let response = Response::builder()
        .header(hyper::header::CONTENT_TYPE, "text/plain; charset=UTF-8")
        .body(Full::new(Bytes::from(response_text)))
        .unwrap();
    Ok(response)
}

// Person構造体
pub struct Person {
    pub name: String,
    pub age: i32,
    pub description: String,
}

async fn build_large_response() -> String {
    let mut bufs: Vec<Vec<u8>> = Vec::with_capacity(50_000);

    for _ in 0..50_000 {
        bufs.push(vec![0u8; 1_024]);        // 5 万回 malloc
    }
    
    println!("perse");

    // 解析フェーズ（ダミー実装）
    parse(&bufs).await.expect("parse failed");

    // 実際は解析結果の文字列や JSON を返す想定
    "done".to_string()
}

/// ダミーの parse() ：本番では独自ロジックを実装
async fn parse(_bufs: &[Vec<u8>]) -> Result<(), ()> {
    // 何らかの非同期処理 …
    Ok(())
}

// Personを生成する関数
// fn get_person() -> Person {
//     Person {
//         name: "WadaTakamichi".to_string(),
//         age: 26,
//         description: concat!(
//         "ぎおんしょうじゃのかねのこえ、しょぎょうむじょうのひびきあり。 ",
//         "さらそうじゅのはなのいろ、 じょうしゃひっすいのことわりをあらわす。 ",
//         "おごれるひともひさしからず、 ただはるのよのゆめのごとし。 ",
//         "たけきものもついにはほろびぬ、 ひとえにかぜのまえのちりにおなじ。",
//         "とおくいちょうをとぶらえば、 しんのちょうこう、かんのおうもう、りょうのしゅうい、とうのろくさん、 ",
//         "これらはみな、きゅうしゅせんこうのまつりごとにもしたがわず、 たのしみをきわめ、いさめをもおもいいれず、 ",
//         "てんかのみだれんことをさとらずして、 みんかんのうれうるところをしらざつしかば、 ひさしからずしてぼうじにしものどもなり。",
//         "ちかくほんちょうをうかがうに、 じょうへいのまさかど、てんぎょうのすみとも、こうわのぎしん、へいじののぶより、 ",
//         "これらはおごれるこころも、たけきこともみなとりどりにこそありしかども、まぢかくは、 ",
//         "ろくはらのにゅうどう、さきのだいじょうだいじん、たいらのあそんきよもりこうともうししひとのありさま、 ",
//         "つたえうけたまわるこそ、こころもことばもおよばれね。",
//         "そのせんぞをたずぬればかんむてんのうだいごのおうじ、 いっぽんしきぶきょうかずらはらしんのうくだいのこういん、さぬきのかみまさもりがまご、 ",
//         "ぎょうぶきょうただもりのあそんのちゃくなんなり。 かのしんのうのみこ、たかみのおう、むかんむいにしてうせたまいぬ。 ",
//         "そのみこ、たかもちのおうのとき、はじめてへいのしょうをたまわって、 かずさのすけになりたまいしより、たちまちにおうしをいでてじんしんにつらなる。 ",
//         "そのこちんじゅふのしょうぐんよしもち、のちにはくにかとあらたむ。 くにかよりまさもりにいたるろくだいは、しょこくのずりょうたりしかども、 ",
//         "てんじょうのせんせきをばいまだゆるされず。" ).to_string(),
//     }
// }
