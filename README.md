# collections
https://doc.rust-lang.org/stable/book/ch08-01-vectors.html

The deal with vectors is that you can grow and shrink them (they're on the heap), but all elements in the vector must be the same type (and this is declared either implicityly or explicitly when you create the vector).

The point is: You can have as many immutable borrows as you want in some scope. But if you have an immutable borrow, you can't do a mutable
borrow. And you can't do two mutable borrows either. However, if you have an immutable and add a mutable, the compiler won't 
complain until you try to use the immutable after the mutable (ex. in a println!). Not sure why but this seems to be the thing.
