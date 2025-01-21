trait CustomIterator<'a> {
    fn has_next(&mut self) -> bool;
    fn next(&mut self) -> Option<&Book>;
}

/// このtraitを実装するオブジェクトのiteratorを払い出すためだけのtrait
trait CustomAggregate {
    // Box<dyn ...>ってどういう意味?
    fn iterator(&mut self) -> Box<dyn CustomIterator<'_> + '_>;

}
#[derive(Debug)]
struct Book {
    name: String
}

impl Book {
    fn get_name(&self) -> String {
      self.name.clone()
    }

    fn new(name: String) -> Self {
        Book { name: name }
    }
}
struct BookShelf {
    books: Vec<Book>
}

impl BookShelf {
    fn new(size: usize) -> Self {
        BookShelf { books: Vec::with_capacity(size) }
    }
    fn get_book_at(&mut self, i: usize) -> Option<&Book> {
        // Vecの中に入っているのはただのBookじゃなかったっけ？なんで&(参照)がつくの？
        self.books.get(i)
    }

    fn append_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn get_length(&mut self) -> usize {
        self.books.len()
    }
}
impl CustomAggregate for BookShelf {
    // &mut selfは可変参照のため所有権は残る
    // 'staticってなーに？
    /// これBookShelIteratorを返すように指定したほうがいいじゃないか？
    fn iterator(&mut self) -> Box<dyn CustomIterator + '_> {
        Box::new(BookShelfIterator::new(self))
    }
}

// BookShelfIteratorの実装
// なんだよ<'a>って...
struct BookShelfIterator<'a> {
    bookShelf: &'a mut BookShelf,
    index: usize
}
impl<'a> BookShelfIterator<'a> {
    fn new (bookshelf: &'a mut BookShelf) -> Self {
        BookShelfIterator { bookShelf: bookshelf, index: 0 }
    }
}
  
impl<'a> CustomIterator<'a> for BookShelfIterator<'a> {
    fn has_next(&mut self) -> bool {
        // もうちょっと書き方あるよなとか思ってる
        let result = self.bookShelf.get_book_at(self.index);
        match result {
            Some(_) => true,
            None => false
        }
    }
  
    fn next(&mut self) -> Option<&Book> {
      // Optionってなんだっけ？
      let book = self.bookShelf.books.get(self.index);
      self.index+=1;
      book
    }
}

fn main() {
    let mut book1 = Book::new("test".to_string());
    let mut book2 = Book::new("test2".to_string());
    let mut book3 = Book::new("Bible".to_string());
    let mut bookshelf = BookShelf::new(4);
    bookshelf.append_book(book1);
    bookshelf.append_book(book2);
    bookshelf.append_book(book3);
    let mut iter = bookshelf.iterator();
    while iter.has_next() {
        let book = iter.next().unwrap();
        println!("{}", book.get_name());
    }
}