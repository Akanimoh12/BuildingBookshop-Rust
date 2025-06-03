use std::{fmt::format, string};

#[derive(Clone, Copy, Debug)]
 enum Status {
        Available,
        Checked,
        UnderRepair
    }

    enum Genre {
        Fiction,
        NonFiction,
        Mystery,
        ScienceFiction
    }

    struct Book {
        title:String,
        author:String,
        pages:u64,
        status:Status,
        genre:Genre,
    }

    impl Book {

        fn new(title:&str, author:String, pages:u64, status:Status, genre:Genre )->Self {
            Self{ title:title.to_string(), author, pages, status, genre }
        }

        fn checked(&mut self){
            self.status=Status::Checked
        }

        fn book(&self)->&Self{
            self
        }

        fn underRepair(&mut self){
            self.status=Status::UnderRepair
        }

        fn bookInfo(&self)->String{
            let title = self.title.clone();
            let author=self.author.clone();
            let pages = self.pages.clone();
            let status=self.status;
            let genre=self.genre;
            
            let fnt=format!("hell the book name is {} by {} it has {} pages, is {}", )
        }

        fn can_be_borrowed(&self){
            if
        }

    }

fn main() {
    
    let book1 = Book::new("fitch", "fitect".to_string(), 40, Genre::Fiction);
    let book2 = Book::new("fitect admin", "sir george".to_string(), 12, Genre::Mystery);
    let book3 = Book::new(
        "fitect student",
        "Mr obed".to_string(),
        540,
        Genre::ScienceFiction,
    );

    let book4 = Book::new(
        "rick dad poor dad",
        "kelvin".to_string(),
        40,
        Genre::Fiction,
    );
    let mut books = vec![book1, book2, book3, book4];

    books[2].checked();
    books[3].checked();
    books[2].underRepair();
    let bookinfo = books[1].bookInfo();
    books[3].can_be_borrowed();
    books[3].genre();
    // println!("book 1 {:#?}",bookinfo);

}