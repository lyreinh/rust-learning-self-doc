fn main()                                                                      {
    let mut book = Vec::new();
    {
        let r = &book;
        book.push(1);
        let x = r.len();

        println!("Length is {} now", x);
    }
                                                                               }
