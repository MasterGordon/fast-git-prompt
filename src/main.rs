use git2::Repository;

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
    let head = repo.head().unwrap();
    let name = head.name().unwrap();
    println!("Hello, world! {}", name);
}
