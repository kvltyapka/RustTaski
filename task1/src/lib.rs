use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// `Post` - структура, представляющая собой пост для блога.
///
/// # Поля
///
/// * `id` - Уникальный идентификатор поста.
/// * `title` - Заголовок поста.
/// * `author` - Автор поста.
/// * `date` - Дата и время создания поста.
/// * `content` - Содержание поста
pub struct Post {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub date: DateTime<Utc>,
    pub content: String,
}

/// `Blog` - структура, представляющая собой блог.
///
/// # Поля
///
/// * `posts` - Хэш-карта, где ключ - это уникальный идентификатор поста, а значение - сам пост.
/// * `next_id` - Идентификатор, который будет присвоен следующему созданному посту.
pub struct Blog {
    pub posts: HashMap<u32, Post>,
    pub next_id: u32,
}

impl Blog {
    /// Создает новый экземпляр `Blog` с пустой хэш-картой для постов и начальным значением `next_id` равным 1.
    ///
    /// # Returns
    ///
    /// * `Self` - Новый экземпляр `Blog`.
    pub fn new() -> Self {
        Blog {
            posts: HashMap::new(),
            next_id: 1,
        }
    }
}
