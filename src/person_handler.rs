use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};
use std::convert::Infallible;

pub async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let person = get_person();
    let response_text = format!("{}\n{}\n{}", person.name, person.age, person.description);
    Ok(Response::new(Full::new(Bytes::from(response_text))))
}

// Person構造体
pub struct Person {
    pub name: String,
    pub age: i32,
    pub description: String,
}

// Personを生成する関数
fn get_person() -> Person {
    Person {
        name: "WadaTakamichi".to_string(),
        age: 26,
        description: concat!(
        "ぎおんしょうじゃのかねのこえ、しょぎょうむじょうのひびきあり。 ",
        "さらそうじゅのはなのいろ、 じょうしゃひっすいのことわりをあらわす。 ",
        "おごれるひともひさしからず、 ただはるのよのゆめのごとし。 ",
        "たけきものもついにはほろびぬ、 ひとえにかぜのまえのちりにおなじ。",
        "とおくいちょうをとぶらえば、 しんのちょうこう、かんのおうもう、りょうのしゅうい、とうのろくさん、 ",
        "これらはみな、きゅうしゅせんこうのまつりごとにもしたがわず、 たのしみをきわめ、いさめをもおもいいれず、 ",
        "てんかのみだれんことをさとらずして、 みんかんのうれうるところをしらざつしかば、 ひさしからずしてぼうじにしものどもなり。",
        "ちかくほんちょうをうかがうに、 じょうへいのまさかど、てんぎょうのすみとも、こうわのぎしん、へいじののぶより、 ",
        "これらはおごれるこころも、たけきこともみなとりどりにこそありしかども、まぢかくは、 ",
        "ろくはらのにゅうどう、さきのだいじょうだいじん、たいらのあそんきよもりこうともうししひとのありさま、 ",
        "つたえうけたまわるこそ、こころもことばもおよばれね。",
        "そのせんぞをたずぬればかんむてんのうだいごのおうじ、 いっぽんしきぶきょうかずらはらしんのうくだいのこういん、さぬきのかみまさもりがまご、 ",
        "ぎょうぶきょうただもりのあそんのちゃくなんなり。 かのしんのうのみこ、たかみのおう、むかんむいにしてうせたまいぬ。 ",
        "そのみこ、たかもちのおうのとき、はじめてへいのしょうをたまわって、 かずさのすけになりたまいしより、たちまちにおうしをいでてじんしんにつらなる。 ",
        "そのこちんじゅふのしょうぐんよしもち、のちにはくにかとあらたむ。 くにかよりまさもりにいたるろくだいは、しょこくのずりょうたりしかども、 ",
        "てんじょうのせんせきをばいまだゆるされず。" ).to_string(),
    }
}
