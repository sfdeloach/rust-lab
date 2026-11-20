// used to demonstrate moving ownership
fn consume_1(s: String) -> usize {
    println!("Dropped at the end of this function, and its heap memory freed: {s}");
    s.len()
}

// used to demonstrate returning ownership
fn consume_2a(s: String) -> String {
    println!("Ownership moved to s, then returned: {s}");
    s
}

// used to demonstrate borrowing ownership
fn consume_2b(s: &str) {
    println!("Ownership not moved, s is a non-owning pointer: {s}");
}

// used to demonstrate cloning (Note: cloning performed outside of this function)
fn consume_2c(s: String) {
    println!("Dropped at the end of this function, and its heap memory freed: {s}");
}

fn main() {
    /*
     * 1. Write fn consume(s: String) -> usize and call it twice with the same variable. Read the full
     *    error (E0382, use of moved value). Write, as a comment above the call, what moved, where it
     *    moved to, and who tried to use it afterwards.
     */

    let str_1 = String::from("hello 1st objective!");
    consume_1(str_1);

    // A `String` is created and assigned to `str_1`. In this example, a Rust `String` is a struct that lives
    // on the stack and one of its fields is a pointer to heap memory that contains the actual string data.
    // When the `consume_1` function is called, a copy of the struct is passed to the calling function. Since
    // the struct involves allocated memory on the heap, ownership of the `String` is moved to the function's
    // `s` parameter. While `str_1` still contains valid references to heap memory, it is no longer the owner.
    // When `consume_1` returns, since `s` had ownership and it went out of scope, any heap memory held by
    // `String` is freed. Strictly enforcing single ownership and freeing memory once it goes out of scope
    // prevents a common bug (aka undefined behavior) of a "double free" -- attempting to free an area of heap
    // memory that has already been freed. `str_1` now references an area of heap memory that is no longer
    // guaranteed to point to it's original target and the Rust compiler will not allow `str_1` to be used
    // again. When `consume_1` is called a second time, an error occurs since it's argument is considered
    // no longer valid.

    // consume_1(str_1); // <<< commented out after first objective met

    println!("------------");

    /*
     * 2. Fix it three different ways: (a) have consume return the String back; (b) change it to borrow
     *    (&str); (c) .clone() at the call site. Write one sentence on the cost of each.
     */

    // 2a. Return the String Back
    // Cost: Similar to what was described earlier in part 1, the big difference here is the function
    // returns, or transfers back, the ownership of the `String`. But ownership must be caught upon return
    // or it is lost. This approach also comes at the cost of writing inelegant/confusing code as the
    // primary purpose of calling a function is usually to do some useful work and return a needed value.
    // This approach's return signature then would return at least two values: values that are the primary
    // reason the function was called in the first place, then the housekeeping value of maintaining ownership
    // of some area of heap memory. Another way to think about it: the return value(s) are a mix of data
    // and metadata.

    let str_2a = String::from("hello 2nd objective, part a!");
    let str_2a = consume_2a(str_2a);

    // here, the returned String is a temporary that nobody binds, so it's dropped at the end of the
    // statement and its heap memory is freed here
    consume_2a(str_2a);

    // consume_2a(str_2a); // <<< can't do this, causes the same E0382 error as step 1

    println!("------------");

    // 2b. Borrowing
    // Cost: Copying a pointer. No allocation and no change of ownership. The constraint paid instead is
    // the caller keeps ownership. The function can only read the value (it would need &mut to change it),
    // and it can't keep it beyond the call.

    let str_2b = String::from("hello 2nd objective, part b!");
    consume_2b(&str_2b);
    consume_2b(&str_2b);

    println!("------------");

    // 2c. Cloning
    // Cost: Cloning creates a full copy, allocating the same amount of memory on the heap as the
    //       current length of the String. The clone method is called to essentially create a throw
    //       away copy, keeping the original string untouched and still accessible.

    let str_2c = String::from("hello 2nd objective, part c!");
    consume_2c(str_2c.clone());
    println!("str_2c still allocated: {}", str_2c);

    /*
     * 3. Repeat with a Vec<i32>, then with a plain i32 and explain why the i32 version compiles
     *    unchanged.
     */

    /*
     *4. Define struct Noisy(&'static str) with an impl Drop that prints its name. Create several in
     *    nested scopes, move one into a function, and predict the print order before running.
     */
}
