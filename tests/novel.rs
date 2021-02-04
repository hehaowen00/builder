use builder::Builder;

#[derive(Builder)]
struct Novel {
    title: String,
    author: String,
    isbn: Option<String>,
    #[builder(each = "chapter")]
    chapters: Vec<String>,
}

fn main() {
    let novel = Novel::builder()
        .title("Title")
        .author("Author")
        .chapter("Chapter 1")
        .chapter("Chapter 2")
        .build()
        .unwrap();

    assert!(novel.isbn.is_none());
    assert_eq!(novel.chapters, vec!["Chapter 1", "Chapter 2"]);

    let novel = Novel::builder()
        .title("Title")
        .author("Author")
        .isbn(String::new())
        .build()
        .unwrap();

    assert!(novel.isbn.is_some());
    assert!(novel.chapters.is_empty());
}
