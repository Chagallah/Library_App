use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
enum BookCategories {
    Any,
    War,
    Fantasy,
}
#[derive(BorshDeserialize, BorshSerialize, Debug)]
enum LibraryPackages {
    None,
    Basic,
    Premium,
}
#[derive(BorshDeserialize, BorshSerialize, Debug)]
#[near_bindgen]
struct Location {
    drawer: u8,
    column: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
#[near_bindgen]
struct Books {
    id: u16,
    name: String,
    available: bool,
    copies: u16,
    category: BookCategories,
    location: Location,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
#[near_bindgen]
struct BooksBorrowed {
    book_id: u16,
    date_borrowed: String,
    date_due: String,
}
#[derive(BorshDeserialize, BorshSerialize, Debug)]
#[near_bindgen]
struct Users {
    name: String,
    id: u8,
    is_member: bool,
    member_package: LibraryPackages,
    books_taken: Vec<BooksBorrowed>,
}
#[near_bindgen]
impl Default for Users {
    fn default() -> Self {
        Users {
            name: "".to_string(),
            id: 0,
            is_member: false,
            member_package: LibraryPackages::None,
            books_taken: vec![],
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
#[near_bindgen]
struct Library {
    users: Vec<Users>,
    books: Vec<Books>,
}
#[near_bindgen]
impl Default for Library {
    fn default() -> Self {
        Library {
            users: vec![],//Vector::new(b"r".to_vec()),
            books: vec![]//::new(b"r".to_vec()),
        }
    }
}

impl Library {
    pub fn get_books_length(&self)->u8 {
   self.books.len() as u8
    }
    pub fn add_book(&mut self, name: String, category: String, drawer: u8, column: u8) {
        let bk = Books {
            id: self.books.len() as u16 + 1,
            name: name,
            available: true,
            copies: 1,
            category: match category.as_str() {
                "war" => BookCategories::War,
                _ => BookCategories::Any,
            },
            location: Location {
                drawer: drawer,
                column: column,
            },
        };
        self.books.push(bk)
    }

    pub fn  add_user(&mut self,name:String ){
        let tmp = self.books.len() as u8 + 1;
        let user = Users {
            name: name,
            id: tmp,
            is_member: false,
            member_package: LibraryPackages::None,
            books_taken: vec![],
        };
        self.users.push(user);
    }
    pub fn borrow_book(&mut self, book_name:String, user_name:String){
// check add user and borrow book function
        let mut bok:Option<&Books> = None;


        for elem in self.books.iter() {
            if elem.name == book_name{
                //elem.available = false;
                bok = Some (elem);
            }
        }
        match bok {
            Some(bk)=>{
                for elem in self.users.iter_mut() {
                    if elem.name == user_name{
    
                        let bk_borrowed = BooksBorrowed{
                            book_id: bk.id,
                            date_borrowed: "20/aug/2022".to_string(),
                            date_due: "27/aug/2022".to_string(),
                        }; 
        
                        elem.books_taken.push (bk_borrowed)
        
        
                    }
                }
        
               }
               None =>{
                panic!("Book not found ")
               }
           }
    
            
    
        }
    }
                
                    

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
 
 
    // TESTS HERE
    #[test]
    fn add_book() {
       
        let mut library = Library::default();
        library.add_book("IT Manual".to_string(), "manuals".to_string(), 8, 3);
        assert_eq!(library.get_books_length(), 1);
    }

    #[test]
    fn add_user() {
       
        let mut library = Library:: default();
        library.add_user("Frank".to_string());
        assert_eq!(library.users.len(), 1);
    }

    #[test]
 
    fn borrow_book() {
      
        let mut library = Library::default();
        library.add_book("IT Manual".to_string(), "manuals".to_string(),8, 3);
        library.add_user("Frank".to_string());
        library.borrow_book("IT Manual".to_string(), "Frank".to_string());
        
        let mut book_length =0;
        for elem in  library.users.iter() {
            if elem.name == "Frank".to_string(){
                book_length = elem.books_taken.len();
                break;
                        }
        }
        assert_eq!(book_length, 1)
    }
   
}

