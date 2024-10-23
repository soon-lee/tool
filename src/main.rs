use db::DbClient;
use oss::OssClient;
use spider::NovelSpider;

mod db;
mod oss;
mod spider;

#[tokio::main]
async fn main() {
    let db = DbClient::from(
        &String::from("diamater.mysql.rds.aliyuncs.com"),
        &String::from("spider_dml"),
        &String::from("S112358r@m"),
        &String::from("db_spider"),
    )
    .await
    .expect("");
    let oss = OssClient::from(
        &String::from(""),
        &String::from(""),
        &String::from("sl-img-service"),
    );
    let spider = NovelSpider::new(String::from("https://www.3bqg.cc"));
    spider
        .run(&db, &oss, &String::from("img.oss.qiniu.diamater.top"))
        .await
        .expect("");
}
