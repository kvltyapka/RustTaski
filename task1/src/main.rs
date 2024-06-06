use std::io;

mod models;
use models::{Blog, Post};

fn new_post(blog: &mut Blog) {
    let mut title = String::new();
    let mut author = String::new();
    let mut content = String::new();

    println!("Enter the title of the post:");
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    println!("Enter the author of the post:");
    io::stdin()
        .read_line(&mut author)
        .expect("Failed to read line");

    println!("Enter the content of the post:");
    io::stdin()
        .read_line(&mut content)
        .expect("Failed to read line");

    let post = Post {
        id: blog.next_id,
        title: title.trim().to_string(),
        author: author.trim().to_string(),
        date: Utc::now(),
        content: content.trim().to_string(),
    };

    blog.posts.insert(blog.next_id, post);
    blog.next_id += 1;

    println!("Post added successfully!");
    print_menu();
}

fn all_posts() {}

fn detail_post() {}

fn print_menu() {
    println!("1. Add new post");
    println!("2. All posts");
    println!("3. Detail post");

    // обработка
    let mut menu = String::new();

    io::stdin()
        .read_line(&mut menu)
        .expect("Failed to read line");

    let menu: u8 = menu.trim().parse().expect("Please type a number!");
    match menu {
        1 => {
            let mut blog = Blog::new();
            new_post(&mut blog);
        }
        2 => println!("all posts"),
        3 => println!("detail post"),
        _ => println!("Invalid option"),
    }
}

fn main() {
    print_menu();
}
