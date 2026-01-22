#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // This method is a perfect example of combining iterators and closures.
    // The filter iterator adapter takes in a closure s and compares it.
    // All those who dont meet the 'filter' are not included in the collected iterator.
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    let v1 = vec![1,2,3];

    // Create an iterator, this by itself does nothing (its simply a type)
    let v1_iter = v1.iter();

    // We could then iterate the iterator like so...
    for val in v1_iter {
        println!("Got: {val}");
    }

    /*
    Standard library trait for Iterator:
        #![allow(unused)]
        fn main() {
            pub trait Iterator {
                type Item;

                fn next(&mut self) -> Option<Self::Item>;
            }
        }
    
    This says that Iterator requires a type Item be defined.
    This Item type is what is returned in the next method (aka returned by iterator).
    Iterator trait only requires implementors to define one method: the next method.
    Next returns one item of the iterator at a time, wrapped in Some, and, when iteration is over, returns None.
    We can call Next() on an iterator directly see test iterator_demonstration() below
    */

    // Iterator Adapter in use:
    let v1: Vec<i32> = vec![1, 2, 3];

    // Map is an iterator adapter, meaning it produces an iterator from another.
    // Here we pass a closure to x and the closure function increments x by 1.
    // Unfortunately the following will produce an error: unused `Map` that must be used
    // v1.iter().map(|x| x + 1);
    // This error is due to Iterators being lazy, they must be used...
    // A simply fix is to call .collect() which will consume the iterator and create a simple collection data type.
    let v2: Vec<i32>  = v1.iter().map(|x| x + 1).collect();

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        // Note: We must make it mutable as next() changes internal iterator state
        let mut v1_iter = v1.iter();

        // We can call next() directly 
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        // We aren’t allowed to use v1_iter after the call to sum, because sum takes ownership of the iterator we call it on.
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        // .sum() is a consuming adapter, it will use up the iterator.
        // .sum() iterates via repeated calls to .next(), adding each item as it goes.
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn filters_by_size() {
        // Lets practice combining closures with iterators.

        // Well begin by defining some shoes of differing styles and sizes.
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        // We then use our helper method to 'filter' the shoes
        let in_my_size = shoes_in_size(shoes, 10);

        // As a result we have only the matched shoes
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
