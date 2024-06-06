mod models;
use chrono::Utc;
use models::{Blog, Post};
use std::io;

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    input.trim().to_string()
}

fn new_post(blog: &mut Blog) {
    print!("{}[2J", 27 as char);

    let title = read_input("Enter the title of the post:");
    let author = read_input("Enter the author of the post:");
    let content = read_input("Enter the content of the post:");

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

    read_input("Press Enter to continue...");
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
    print!("{}[2J", 27 as char);

    for (_, post) in &blog.posts {
        print_post(post);
    }

    read_input("Press Enter to continue...");
}

fn detail_post(blog: &Blog) {
    print!("{}[2J", 27 as char);

    let id: u32 = read_input("Enter the ID of the post:")
        .parse()
        .expect("Please type a number!");

    if let Some(post) = blog.posts.get(&id) {
        print_post(post);
    } else {
        println!("No post found with ID {}", id);
    }

    read_input("Press Enter to continue...");
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
            1 => new_post(&mut blog),
            2 => all_posts(&blog),
            3 => detail_post(&blog),
            4 => break,
            _ => println!("Invalid option"),
        }
    }
}
