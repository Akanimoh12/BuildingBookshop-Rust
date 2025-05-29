use std::string;

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

        fn constructor(title:&str, author:String, pages:u64, status:Status, genre:Genre )->Book {
            Book { title:title.to_string(), author, pages, status, genre }
        }

        fn checked(){

        }

    }

fn main() {
   

}