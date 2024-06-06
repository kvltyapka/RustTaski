use crate::Blog;
use crate::Post;
use chrono::Utc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_post() {
        let mut blog = Blog::new();
        let post = Post {
            id: 1,
            title: "Test Title".to_string(),
            author: "Test Author".to_string(),
            date: Utc::now(),
            content: "Test Content".to_string(),
        };
        blog.posts.insert(1, post);

        assert_eq!(blog.posts.len(), 1);
        assert_eq!(blog.posts.get(&1).unwrap().title, "Test Title");
    }

    #[test]
    fn test_get_post_detail() {
        let mut blog = Blog::new();
        let post = Post {
            id: 1,
            title: "Test Title".to_string(),
            author: "Test Author".to_string(),
            date: Utc::now(),
            content: "Test Content".to_string(),
        };
        blog.posts.insert(1, post);

        let post_detail = blog.posts.get(&1).unwrap();
        assert_eq!(post_detail.title, "Test Title");
        assert_eq!(post_detail.author, "Test Author");
        assert_eq!(post_detail.content, "Test Content");
    }

    #[test]
    fn test_get_non_existent_post() {
        let blog = Blog::new();

        let post_detail = blog.posts.get(&1);
        assert!(post_detail.is_none());
    }

    #[test]
    fn test_view_all_posts() {
        let mut blog = Blog::new();
        let post1 = Post {
            id: 1,
            title: "Test Title 1".to_string(),
            author: "Test Author 1".to_string(),
            date: Utc::now(),
            content: "Test Content 1".to_string(),
        };
        let post2 = Post {
            id: 2,
            title: "Test Title 2".to_string(),
            author: "Test Author 2".to_string(),
            date: Utc::now(),
            content: "Test Content 2".to_string(),
        };
        blog.posts.insert(1, post1);
        blog.posts.insert(2, post2);

        assert_eq!(blog.posts.len(), 2);
    }
}
