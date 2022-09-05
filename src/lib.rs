use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};

/// AppLibrary app
/// This is an AppLibrary app that
/// 1. tracks book in the app
/// 2. tracks users
/// 3. a list of books borrowed by whom
/// 
/// We use env to get current user and current timestamps for the books

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
enum BookCategories {
    Any,
    War,
    Fantasy,
}
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
enum AppLibraryPackages {
    None,
    Basic,
    Premium,
}
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
#[near_bindgen]
struct Location {
    drawer: u8,
    column: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
#[near_bindgen]
struct Books {
    id: u64,
    name: String,
    copies: u16,
    category: BookCategories,
    location: Location,
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
struct BooksBorrowed {
    book_id: u64,
    date_borrowed: u64,
    date_due: u64,
}
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
#[near_bindgen]
struct Users {
    account: AccountId,
    name: String,
    id: u64,
    is_member: bool,
    // member_package: AppLibraryPackages,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct AppLibrary {
    users: Vector<Users>,
    books: Vector<Books>,
    books_borrowed: UnorderedMap<AccountId, Vector<BooksBorrowed>>,
}


impl Default for AppLibrary {
    fn default() -> Self {
        AppLibrary {
            users: Vector::new(b"r".to_vec()),
            books: Vector::new(b"r".to_vec()),
            books_borrowed: UnorderedMap::new(b"p".to_vec()),
        }
    }
}

#[near_bindgen]
impl AppLibrary {
    pub fn get_users(&self) -> Vec<Users> {
        let mut item: Vec<Users> = vec! []; 
        for itm in self.users.iter() {
            item.push(itm)
        }
        item
    }
    pub fn get_books_borrowed(&self) -> Option<Vec<BooksBorrowed>> {
        let mut items: Vec<BooksBorrowed> = vec![];
        match self.books_borrowed.get(&env::current_account_id()) {
            Some(bks) => {
                for itm in bks.iter() {
                    items.push(itm)
                }
                Some(items)
            }
            None => {
                env::log_str("User has no books borrowed");
                None
            }
        }
    }
    pub fn add_book(&mut self, name: String, category: String, drawer: u8, column: u8) -> u64 {
        let id = env::block_timestamp();
        let bk = Books {
            id: id,
            name: name,
            copies: 1,
            category: match category.as_str() {
                "war" => BookCategories::War,
                _=> BookCategories::Any,
            },
            location: Location {
                drawer: drawer,
                column: column,
            },
        };
        self.books.push(&bk);

        return id;
    }

    pub fn add_user(&mut self, user_name: String) {
        let user = Users {
            account: env::current_account_id(),
            name: user_name,
            id: env::block_height(),
            is_member: false,
            // member_package: AppLibraryPackages::None,
        };
        self.users.push(&user);
    }

    pub fn borrow_book(&mut self, book_id: u64) -> String {
        let books_borrowed = self.books_borrowed.get(&env::current_account_id());
        let bk = BooksBorrowed {
            book_id: book_id,
            date_borrowed: env::block_timestamp(),
            date_due: env::block_timestamp() + (60 * 60 * 60 * 24), // book is borrowed for 24 hours 
        };
        
        match books_borrowed {
            Some(mut user_bks) => {
                user_bks.push(&bk);


                self.books_borrowed
                    .insert(&env::current_account_id(), &user_bks);
            }
            None => {
                let mut tmp = Vector::new(b"r".to_vec());

                tmp.push(&bk);
                self.books_borrowed.insert(&env::current_account_id(), &tmp);
            }
        }

        "okay".to_string()
    }
}


                
                    
// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};
 
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }
 
 
    // TESTS HERE
    #[test]
    fn add_book() {
        let user = AccountId::new_unchecked("chagalla.testnet".to_string());
        let mut context = get_context(user.clone());

        context.block_timestamp(9999);
        testing_env!(context.build());
       
        let mut app = AppLibrary::default();
        app.add_book("IT Manual".to_string(), "manuals".to_string(), 8, 3);
        assert_eq!(app.get_books_length(), 1);
    }

    #[test]
    fn add_user() {
        let user = AccountId::new_unchecked("chagalla.testnet".to_string());
        let context = get_context(user.clone());
        testing_env!(context.build());
        let mut app = AppLibrary:: default();
        app.add_user("Frank".to_string());
        assert_eq!(app.users.len(), 1);
    }

    #[test]
 
    fn borrow_book_item() {
        let user = AccountId::new_unchecked("chagalla.testnet".to_string());
        let mut context = get_context(user.clone());

        context.block_timestamp(9999);
        testing_env!(context.build());

        let mut app = AppLibrary::default();
        app.add_book("Sample".to_string(), "manuals".to_string(), 8, 3);
        assert_eq!(app.get_books().len(), 1);
        
        app.add_user("Chagalla".to_string());
        assert_eq!(app.users.len(), 1);

        app.borrow_book(9999);
        
        match app.get_books_borrowed() {
            Some(bkx) => {
                assert_eq!(bkx.len(), 1)
            }
            None => env::panic_str("book borrowed by user nt found"),
        }
    }
}
 


