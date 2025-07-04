// use std::env;
// use std::process;

// use minigrep::Config; // from lib.rs
// use minigrep::run;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let config = Config::build(&args).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {}", err);
//         std::process::exit(1);
//     });

//     if let Err(e) = run(config) {
//         eprintln!("Application error: {e}");
//         process::exit(1);
//     }

// 13.1 Capturing the environment with Closures

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColour {
        Red,
        Blue,
    }

    struct Inventory {
        shirts: Vec<ShirtColour>,
    }

    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColour>) -> ShirtColour {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }

        fn most_stocked(&self) -> ShirtColour {
            let mut num_red = 0;
            let mut num_blue = 0;

            for colour in &self.shirts {
                match colour {
                    ShirtColour::Red => num_red += 1,
                    ShirtColour::Blue => num_blue += 1,
                }
            }
            if num_red > num_blue {
                ShirtColour::Red
            } else {
                ShirtColour::Blue
            }
        }
    }
//}

fn main() {
    // let store = Inventory {
    //     shirts: vec![ShirtColour::Blue, ShirtColour::Red, ShirtColour::Blue],

    // };
    
    // let user_pref1 = Some(ShirtColour::Red);
    // let giveaway1 = store.giveaway(user_pref1);
    // println!(
    //     "The user with preference {:?} gets {:?}",
    //     user_pref1, giveaway1
    // );

    // let user_pref2 = None;
    // let giveaway2 = store.giveaway(user_pref2);
    // println!(
    //     "The user with preference {:?} gets {:?}",
    //     user_pref2, giveaway2
    // );
    // Shirt company giveaway situation
    // The closure captures an immutable reference to the 'self' "Inventory" instance and passes it with passes it with the code we specify to the 
    // with the cpde we specify to the 'unwrap_or_else' method. Functions, on the other hand, are not able to capture their environment in this way.

    // CAPTURING References OR Moving Ownership:

    // Closures can capture values from their environment in 3 ways (1 borrowing immutably, 2 borrowing mutable, 3 and taking ownership)
    // The closure will decide which of these to use based on what the body of the function does with the captured values.
    
   // fn main() {
        // let list = vec![1, 2, 3];
        // println!("Before defning closure: {list:?}");

        // let only_borrows = || println! ("From closure: {list:?}");

        // println!("Before calling closure: {list:?}");
        // only_borrows();
        // println!("After calling closure: {list:?}");
   //} 
   // Because we can have multiple immutable references to 'list' at the same time, 'list' is still accessible from the code before the closure definition, after the closure definintion but before tyhe closure is called, and after the closure is called. code compiles and runs. 


   // Next, we change the closure body so that it adds an element to the 'list' vector. The closure now captures a mutable reference:

   // fn main() {
        // let mut list = vec![1, 2, 3];
        // println!("Before defining closure: {list:?} ");

        // let mut borrows_mutably = || list.push(7);

        // borrows_mutably();
        // println!("After calling closure: {list:?}");
   //}

   // Note; there's no longer a 'println!' between the definintion and the call of the 'borrows_mutably' closure: when 'borrows_mutably' 
   // is defined, it captures a mutable reference to list. We don't use a closure again after the closure is called, so the mutable borrow ends. Between the closure and the closure call, an immutable borrow to print isn't allowed because no other borrows are allowed when there's mutable borrow. 
   // If you want to force the closure to take ownership of the values it uses in the environment even though the body of the closure doesn’t strictly need ownership, you can use the move keyword before the parameter list.
    // If you want to force the closure to take ownership of the values it uses in the environment even though the body of the closure doesn’t strictly need ownership, you can use the move keyword before the parameter list.
    // This technique is mostly useful when passing a closure to a new thread to move the data so that it’s owned by the new thread.


    // // BIG IDEA: comparing Closure with Rc<RefCell<Vec<i32>>> :

    // use std::rc::Rc;
    // use std::cell::RefCell;

    // // fn main() {
    //     let list = Rc::new(RefCell::(vec![1, 2, 3]));
    //     println!("Before defining closure: {:?}", list.borrow());

    //     let list_clone = Rc::clone(&list); // CREATING a second SHARED owner, now BOTH main and closure own list.
    //     let modifies_list = move || {
    //         list_clone.borrow_mut().push(4);
    //         println!("From closure (after mutation): {:?}", list_clone.borrow());
    //     };

    //     println!("Before calling closure: {:?}", list.borrow());
    //     modifies_list(); // calling the closure, pushes 4 into the vector
    //     println!("After calling closure: {:?}", list.borrow());
    // //}

    // Rc<T> : allows MULTIPLE OWNERS (reference counted)
    // RefCell<T> : Allows MUTABLE BORROWS AT RUNTIME, even when the outer structure is immutable
    // This setup enables cloning and MUTATING the vector inside the closure.

    // The CLOSURE:

    // let modifies_list = move || {
    //    list_clone.borrow_mut().push(4);
    //    println!("From closure (after mutation): {:?}", list_clone.borrow());
    

    // - move : Needed to transfer ownership of 'list_clone' into the closure.
    // - borrow_mut() : gives a mutable reference to the vector (checked at runtime).
    // - Closure modifies the vector --- something the original code couldn't do

    // MOVING CAPTURED values out of Closures and the Fn Traits:
    // - FnOnce => applies to closures that can be called once - only closures that move captured values out of its body.
    // - FnMut => applies to closures that don't move captured values out of their body, but might mutate the captured value. These can be called more than once.
    // - Fn => applies to closure that dont move values out of their body, that don't mutate their values as well as closure that captures nothing from their environment.Can be called more than once without mutating their environment...imprtant for concurrent multiple calls!

    
    // EXAMPLE:

    // impl<T> Option<T> {
    //     pub fn unwrap_or_else<F>(self, f: F) -> T
    //     where
    //     F: FnOnce() -> T 
    //     {
    //         match self {
    //             Some self {
    //                 Some(x) => x,
    //                 None => f(),
    //             }
    //         }
    //     }
    // }

    // T is a generic type representing the type of the value in the 'Some' variant of an 'Option'. That type T is also the return type of the 'unwrap_or_else' function: code that calls 'unwrap_or_else' on an 'Option<String>', for example, will get a String.
   // Next, notice the 'unwrap_or_else' function has the additional generic type parameter F. The F type is the type of the parameter named 'f', which is the closure we provide when calling 'unwrap_or_else'.
   // The trait bound specified on the generic type F is 'FnOnce() -> T', which means F must be able to be called once, take no arguments, and return a T. Using 'FnOnce' in the trait bound expresses the constraint that 'unwrap_or_else'
   // is going to is only going to call f at most one time.
   // In the body of 'unwrap_or_else', we can see that if the 'Option' is 'Some', f won't be called.
   // If the Option in None, f will be called once. Because all closures implement 'FnOnce', 'unwrap_or_else' accepts all three kinds of closures and is as flexible as it can be.   
    

   // Looking at the std lib method 'sort_by_key' defeined on slices, see how that differs from 'unwrap_or_else' and why 'sort_by_key' use FnMut instead of FnOnce for the trait bound.

   #[derive(Debug)]
   struct Rectangle {
    width: u32,
    height: u32,
   }

   //fn main() {
        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5},
            Rectangle { width: 7, height: 3},
            Rectangle { width: 13, height: 8},

    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
   //}

   // The reason 'sort_by_key' is define to take an FnMut closure is that it calls the closure multiple times: once for each item in the slice. 
   // The closure |r| r.width doesn't capture. mutate, or move anything out of its environment, so it meet the trait bound requirements.

   // THE EXAMPLE below, will error: this is because the closure that implements just the FnOnce trait, because it moves a value out of the environment.
   // The compiler won't let us use this closure with 'sort_by_key':

   #[derive(Debug)]
   struct Rectangle {
    width: u32,
    height: u32,
   }

   let mut sort_operations = vec![];
   let value = String::from("closure called");

   list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
   });
   println!("{list:#?}"); // this code will cause the compiler to complain

// Fixing the code above:

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1},
        Rectangle { width: 3, height: 6},
        Rectangle { width: 10, height: 13},
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println("{list:#?}, sorted in {num_sort_operations} operations");
} // Using an FnMut closure with sort_by_key is allowed

// THE ITERATOR trait and the next Method

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided

    // the traint 'Iterator' from the std lib 
    // it also has a type 'Item' and 'Self::Item' which define an ASSOCIATED TYPE.
    // the Iterator trait requires to be implemented by one method: the 'next' method
    // the method returns one item of the iterator at a time wrapped in 'Some' and, when the iteration is over, it returns None.
}

// calling the next method on iterators directly;

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // we needed to make v1_iter mutable: calling the next method on an iterator changes the internal state
    // that the iterator uses to keep tract of where it is in the sequence. IN other words, this code cconsumes
    // or uses up the iterator.Each call to next eats up an item from the iterator.
    // We did not need to make v1_iter mutable when we used a 'for' loop because the loop takes ownership of of v1_iter and made it mutable behind he scenes
    // Also, the values we get from the calls to next are immutable references to the values in the vector.
    // THe iter method produces an iterator over immutable references. If 
    //  If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter. 
    // Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter.

}

// Methods that consume the iterator:
// they are called - consuming adapters - e.g the sum method

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

// Using Closures that Capture Their Environment

// - Many iterator adapters take closures as arguments, these will be closure that capture their environment
// - below, we use a filter method that takes a closure. The closure gets an item from the iterator and retursn a 'bool' . If the closure
// - returns a true, the value will be includedin the iteration produced by the filter
// - If the closure retuens a false, the value won't be included

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,    
}

fn shoes_in_size(shoes: Vec<Shoes>, shoes_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoes_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec! [
            Shoe {
                size: 10,
                style: String::from("amariFu"),
            },
            Shoe {
                size: 13,
                style: String::from("Imbadada"),
            },
            Shoe {
                size: 11,
                style: String::from("sleepers"),
            },
        ];

            let in_my_size = shoes_in_size(shoes, 10);

            assert_eq!(
                in_my_size,
                vec![
                    Shoe {
                        size: 10,
                        style: String::from("amariFu")
                    },

                    Shoe {
                        size: 11,
                        style: String::from("sleepers")
                    },
                ]
            );
        }
    }

}