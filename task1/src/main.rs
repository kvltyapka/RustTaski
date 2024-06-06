mod models;
use chrono::Utc;
use models::{Blog, Post};
use std::io;

fn new_post(blog: &mut Blog) {
    print!("{}[2J", 27 as char);

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

    println!("Press Enter to continue...");
    let mut pause = String::new();
    io::stdin()
        .read_line(&mut pause)
        .expect("Failed to read line");
}

fn all_posts(blog: &Blog) {
    print!("{}[2J", 27 as char);

    for (id, post) in &blog.posts {
        println!("ID: {}", id);
        println!("Title: {}", post.title);
        println!("Author: {}", post.author);
        println!("Date: {}", post.date);
        println!("Content: {}", post.content);
        println!("---------------------------");
    }

    println!("Press Enter to continue...");
    let mut pause = String::new();
    io::stdin()
        .read_line(&mut pause)
        .expect("Failed to read line");
}

fn detail_post(blog: &Blog) {
    print!("{}[2J", 27 as char);
    println!("Enter the ID of the post:");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read line");
    let id: u32 = id.trim().parse().expect("Please type a number!");

    if let Some(post) = blog.posts.get(&id) {
        println!("ID: {}", post.id);
        println!("Title: {}", post.title);
        println!("Author: {}", post.author);
        println!("Date: {}", post.date);
        println!("Content: {}", post.content);
    } else {
        println!("No post found with ID {}", id);
    }

    println!("Press Enter to continue...");
    let mut pause = String::new();
    io::stdin()
        .read_line(&mut pause)
        .expect("Failed to read line");
}

fn print_menu() {
    print!("{}[2J", 27 as char);
    println!("\n1. Add new post");
    println!("2. All posts");
    println!("3. Detail post");
}

fn main() {
    let mut blog = Blog::new();

    loop {
        print_menu();
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => new_post(&mut blog),
            2 => all_posts(&blog),
            3 => detail_post(&blog),
            _ => println!("Invalid option"),
        }
    }
}
