use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    db::{DataNovel, DataNovelChapter, DbClient},
    oss::OssClient,
};

pub enum NovelCategory {
    Fantasy(i32),
    MartialArts(i32),
    Urban(i32),
    Historical(i32),
    WebGame(i32),
    SciFi(i32),
    Female(i32),
    Completed(i32),
    None,
}
impl From<&str> for NovelCategory {
    fn from(s: &str) -> Self {
        match s {
            "玄幻" => NovelCategory::Fantasy(1),
            "武侠" => NovelCategory::MartialArts(2),
            "都市" => NovelCategory::Urban(3),
            "历史" => NovelCategory::Historical(4),
            "网游" => NovelCategory::WebGame(5),
            "科幻" => NovelCategory::SciFi(6),
            "女生" => NovelCategory::Female(7),
            "完结" => NovelCategory::Completed(0),
            _ => NovelCategory::None,
        }
    }
}
impl ToString for NovelCategory {
    fn to_string(&self) -> String {
        match self {
            NovelCategory::Fantasy(s) => s.to_string(),
            NovelCategory::MartialArts(s) => s.to_string(),
            NovelCategory::Urban(s) => s.to_string(),
            NovelCategory::Historical(s) => s.to_string(),
            NovelCategory::WebGame(s) => s.to_string(),
            NovelCategory::SciFi(s) => s.to_string(),
            NovelCategory::Female(s) => s.to_string(),
            NovelCategory::Completed(s) => s.to_string(),
            NovelCategory::None => "-1".to_string(),
        }
    }
}
pub struct NovelChapter {}
impl NovelChapter {
    fn new() -> Self {
        Self {}
    }
    pub fn url(&self, origin: &String, path: &String) -> String {
        format!("{}{}", origin, path)
    }
    pub fn content(&self, element: &ElementRef) -> Result<String, String> {
        let content_selector = Selector::parse("div#chaptercontent")
            .map_err(|err| format!("选择器错误：{:?}", err))?;
        let content_ref = element
            .select(&content_selector)
            .next()
            .ok_or("元素选择错误")?;
        let content = content_ref.text().collect::<Vec<_>>().join("");
        Ok(content.replace(
            "请收藏本站：https://www.3bqg.cc。笔趣阁手机版：https://m.3bqg.cc",
            "",
        ))
    }
}
pub struct NovelTable {}
impl NovelTable {
    fn new() -> Self {
        Self {}
    }
    pub fn url(&self, origin: &String, path: &String) -> String {
        format!("{}{}", origin, path)
    }
    pub fn table(&self, element: &ElementRef) -> Result<(String, Vec<(String, String)>), String> {
        let state_selector =
            Selector::parse("body > div.book > div.info div.small > span:nth-child(2)")
                .map_err(|err| format!("选择器错误：{:?}", err))?;
        let state_ref = element
            .select(&state_selector)
            .next()
            .ok_or("元素选择错误")?;
        let state = state_ref.text().collect::<Vec<_>>().join("");
        let mut chapter_list = Vec::<(String, String)>::new();
        let table_seelctor = Selector::parse("body > div.listmain > dl dd:not(.more.pc_none) > a")
            .map_err(|err| format!("选择器错误：{:?}", err))?;
        for table_ref in element.select(&table_seelctor) {
            let title = table_ref.text().collect::<Vec<_>>().join("");
            let path = table_ref.attr("href").ok_or("元素选择错误")?;
            chapter_list.push((String::from(path), title));
        }
        Ok((state, chapter_list))
    }
}
pub struct NovelList {
    category: String,
}
impl NovelList {
    pub fn new(category: String) -> Self {
        Self { category }
    }
    pub fn path(&self, page: &String) -> String {
        format!(
            "sortid={}&page={}",
            NovelCategory::from(self.category.as_str()).to_string(),
            page
        )
    }
    pub fn url(&self, origin: &String, page: &String) -> String {
        format!("{}/json?{}", origin, self.path(page))
    }
    pub async fn novel_detail(
        &self,
        url: &String,
    ) -> Result<Vec<(String, String, String, String, String)>, String> {
        let response = Client::new()
            .get(url)
            .send()
            .await
            .map_err(|err| format!("连接\"{}\"错误：{:?}", url, err))?;
        let json = response
            .json::<Value>()
            .await
            .map_err(|err| format!("解析\"{}\"错误：{:?}", url, err))?;
        let novel_info_list = json.as_array().ok_or("数据格式错误".to_string())?;
        let mut novels = Vec::<(String, String, String, String, String)>::new();
        for novel_info in novel_info_list {
            let title = novel_info["articlename"]
                .as_str()
                .ok_or("数据格式错误".to_string())?;
            let author = novel_info["author"]
                .as_str()
                .ok_or("数据格式错误".to_string())?;
            let path = novel_info["url_list"]
                .as_str()
                .ok_or("数据格式错误".to_string())?;
            let brief = novel_info["intro"]
                .as_str()
                .ok_or("数据格式错误".to_string())?;
            let cover = novel_info["url_img"]
                .as_str()
                .ok_or("数据格式错误".to_string())?;
            novels.push((
                String::from(title),
                String::from(author),
                String::from(path),
                String::from(brief),
                String::from(cover),
            ));
        }
        Ok(novels)
    }
}
pub struct NovelSpider {
    origin: String,
}
impl NovelSpider {
    pub fn new(origin: String) -> Self {
        Self { origin }
    }
    pub fn novel_list(&self, category: String) -> NovelList {
        NovelList::new(category)
    }
    pub async fn run(&self, db: &DbClient, oss: &OssClient, origin: &String) -> Result<(), String> {
        let mut novels_list = Vec::<NovelList>::new();
        novels_list.push(self.novel_list(String::from("玄幻")));
        novels_list.push(self.novel_list(String::from("武侠")));
        novels_list.push(self.novel_list(String::from("都市")));
        novels_list.push(self.novel_list(String::from("历史")));
        novels_list.push(self.novel_list(String::from("网游")));
        novels_list.push(self.novel_list(String::from("科幻")));
        novels_list.push(self.novel_list(String::from("女生")));
        novels_list.push(self.novel_list(String::from("完结")));
        for novels in novels_list {
            let mut page = 1;
            loop {
                let url = novels.url(&self.origin, &page.to_string());
                match novels.novel_detail(&url).await {
                    Ok(novel_info_list) => {
                        for (title, author, path, brief, cover) in novel_info_list {
                            let novel_id = Uuid::new_v4().to_string();
                            let novel_table = NovelTable::new();
                            let table_url = novel_table.url(&self.origin, &path);
                            let table_response = Client::new()
                                .get(table_url)
                                .send()
                                .await
                                .map_err(|err| format!("连接\"{}\"错误：{:?}", url, err))?;
                            let table_text = table_response
                                .text()
                                .await
                                .map_err(|err| format!("解析\"{}\"错误：{:?}", url, err))?;
                            let table_document = Html::parse_document(&table_text);
                            let (state, chapter_list) =
                                novel_table.table(&table_document.root_element())?;
                            let parts = &cover
                                .split(".")
                                .map(|s| String::from(s))
                                .collect::<Vec<_>>();
                            let extension = parts.last().ok_or(String::from("不是图片"))?;
                            let object_name = format!("novel/cover/{}.{}", novel_id, extension);
                            oss.post_from_url(&cover, &object_name).await?;
                            let data_novel = DataNovel::new(
                                novel_id.clone(),
                                title,
                                author,
                                brief,
                                format!("http://{}/{}", origin, object_name),
                                state,
                                novels.category.clone(),
                            );
                            let mut data_novel_chapters = Vec::<DataNovelChapter>::new();
                            for (path, title) in chapter_list {
                                let novel_chapter = NovelChapter::new();
                                let chapter_url = novel_chapter.url(&self.origin, &path);
                                let chapter_response = Client::new()
                                    .get(chapter_url)
                                    .send()
                                    .await
                                    .map_err(|err| format!("连接\"{}\"错误：{:?}", url, err))?;
                                let chapter_text = chapter_response
                                    .text()
                                    .await
                                    .map_err(|err| format!("解析\"{}\"错误：{:?}", url, err))?;
                                let chapter_document = Html::parse_document(&chapter_text);
                                let content =
                                    novel_chapter.content(&chapter_document.root_element())?;
                                let chapter_id = Uuid::new_v4().to_string();
                                data_novel_chapters.push(DataNovelChapter::new(
                                    chapter_id,
                                    title,
                                    content,
                                    novel_id.clone(),
                                ));
                            }
                            db.insert_novel(data_novel, data_novel_chapters).await?;
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                        break;
                    }
                }
                page += 1;
                break;
            }
            break;
        }
        Ok(())
    }
}
