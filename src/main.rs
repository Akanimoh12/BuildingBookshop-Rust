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
            
            let fnt=format!("hello the book name is {} by {} it has {} pages", )
        }

        fn can_be_borrowed(&self){
            if
        }

    }

fn main() {
   

}