use std::sync::Arc;

use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    query, MySqlPool,
};

pub struct DataNovel {
    id: String,
    title: String,
    author: String,
    brief: String,
    cover: String,
    state: String,
    category: String,
}
impl DataNovel {
    pub fn new(
        id: String,
        title: String,
        author: String,
        brief: String,
        cover: String,
        state: String,
        category: String,
    ) -> Self {
        Self {
            id,
            title,
            author,
            brief,
            cover,
            state,
            category,
        }
    }
}
pub struct DataNovelChapter {
    id: String,
    title: String,
    content: String,
    novel_id: String,
}
impl DataNovelChapter {
    pub fn new(id: String, title: String, content: String, novel_id: String) -> Self {
        Self {
            id,
            title,
            content,
            novel_id,
        }
    }
}
pub struct DbClient {
    pool: Arc<MySqlPool>,
}
impl DbClient {
    pub async fn from(
        host: &String,
        username: &String,
        password: &String,
        database: &String,
    ) -> Result<Self, String> {
        let connection_options = MySqlConnectOptions::new()
            .host(host)
            .port(3306)
            .username(username)
            .password(password)
            .database(database);
        let pool = MySqlPoolOptions::new()
            .connect_with(connection_options)
            .await
            .map_err(|err| format!("连接数据库失败：{:?}", err))?;
        Ok(Self::new(Arc::new(pool)))
    }
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }
    pub async fn insert_novel(
        &self,
        novel: DataNovel,
        chapters: Vec<DataNovelChapter>,
    ) -> Result<(), String> {
        query("INSERT INTO `tb_novel`(`id`,`title`,`author`,`brief`,`cover`,`category`) VALUE(?,?,?,?,?,?);")
        .bind(novel.id)
        .bind(novel.title)
        .bind(novel.author)
        .bind(novel.brief)
        .bind(novel.cover)
        .bind(novel.category)
      .execute(&*self.pool).await.map_err(|err| format!("{:?}",err))?;
        let sql = format!(
            "INSERT INTO `tb_novel_chapter`(`id`,`title`,`content`,`novel_id`) VALUES{};",
            &chapters
                .iter()
                .map(|_| "(?,?,?,?)")
                .collect::<Vec<_>>()
                .join(",")
        );
        let mut batch = query(&sql);
        for chapter in chapters {
            batch = batch
                .bind(chapter.id)
                .bind(chapter.title)
                .bind(chapter.content)
                .bind(chapter.novel_id);
        }
        batch
            .execute(&*self.pool)
            .await
            .map_err(|err| format!("{:?}", err))?;
        Ok(())
    }
}
