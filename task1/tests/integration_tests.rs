// models.rs

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::io;

#[derive(Debug, PartialEq, Clone)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub date: DateTime<Utc>,
    pub content: String,
}

pub struct Blog {
    pub next_id: u32,
    pub posts: HashMap<u32, Post>,
}

impl Blog {
    pub fn new() -> Blog {
        Blog {
            next_id: 1,
            posts: HashMap::new(),
        }
    }
}

// main.rs

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    input.trim().to_string()
}

fn new_post(blog: &mut Blog, title: String, author: String, content: String) {
    let post = Post {
        id: blog.next_id,
        title,
        author,
        date: Utc::now(),
        content,
    };

    blog.posts.insert(blog.next_id, post);
    blog.next_id += 1;

    println!("Post added successfully!");
}

fn print_post(post: &Post) {
    println!("ID: {}", post.id);
    println!("Title: {}", post.title);
    println!("Author: {}", post.author);
    println!("Date: {}", post.date);
    println!("Content: {}", post.content);
    println!("---------------------------");
}

fn all_posts(blog: &Blog) {
    for (_, post) in &blog.posts {
        print_post(post);
    }
}

fn detail_post(blog: &Blog, id: u32) -> Option<&Post> {
    blog.posts.get(&id)
}

fn print_menu() {
    print!("{}[2J", 27 as char);
    println!("\n1. Add new post");
    println!("2. All posts");
    println!("3. Detail post");
    println!("4. Exit\n");
}

fn main() {
    let mut blog = Blog::new();

    loop {
        print_menu();
        let choice: u8 = read_input("Enter your choice:").parse().unwrap_or(0);

        match choice {
            1 => {
                let title = read_input("Enter the title of the post:");
                let author = read_input("Enter the author of the post:");
                let content = read_input("Enter the content of the post:");
                new_post(&mut blog, title, author, content);
            }
            2 => all_posts(&blog),
            3 => {
                let id: u32 = read_input("Enter the ID of the post:")
                    .parse()
                    .expect("Please type a number!");
                if let Some(post) = detail_post(&blog, id) {
                    print_post(post);
                } else {
                    println!("No post found with ID {}", id);
                }
            }
            4 => break,
            _ => println!("Invalid option"),
        }
    }
}

// ТЕСТЫ

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_post() {
        let mut blog = Blog::new();
        let title = String::from("Test Post");
        let author = String::from("Test Author");
        let content = String::from("This is a test post.");

        new_post(&mut blog, title.clone(), author.clone(), content.clone());

        assert!(blog.posts.contains_key(&1));
        let post = blog.posts.get(&1).unwrap();
        assert_eq!(post.title, title);
        assert_eq!(post.author, author);
        assert_eq!(post.content, content);
    }

    #[test]
    fn test_get_post_details() {
        let mut blog = Blog::new();
        let title = String::from("Test Post");
        let author = String::from("Test Author");
        let content = String::from("This is a test post.");

        new_post(&mut blog, title.clone(), author.clone(), content.clone());

        let post = detail_post(&blog, 1).expect("Post should exist");
        assert_eq!(post.title, title);
        assert_eq!(post.author, author);
        assert_eq!(post.content, content);
    }

    #[test]
    fn test_handle_nonexistent_post() {
        let blog = Blog::new();

        assert!(detail_post(&blog, 1).is_none());
    }

    #[test]
    fn test_view_all_posts() {
        let mut blog = Blog::new();
        let title1 = String::from("Test Post 1");
        let author1 = String::from("Test Author 1");
        let content1 = String::from("This is a test post 1.");

        new_post(&mut blog, title1.clone(), author1.clone(), content1.clone());

        let title2 = String::from("Test Post 2");
        let author2 = String::from("Test Author 2");
        let content2 = String::from("This is a test post 2.");

        new_post(&mut blog, title2.clone(), author2.clone(), content2.clone());

        let posts = blog.posts.values().collect::<Vec<&Post>>();
        assert_eq!(posts.len(), 2);
        assert!(posts.iter().any(|p| p.title == title1));
        assert!(posts.iter().any(|p| p.title == title2));
    }
}
