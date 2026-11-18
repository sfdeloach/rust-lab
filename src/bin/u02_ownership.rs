fn consume_1(s: String) -> usize {
    println!("ownership moved to s: {s}");
    0
}

fn consume_2a(s: String) -> String {
    println!("ownership moved to s, then returned: {s}");
    s
}

fn consume_2b(s: &String) {
    println!("ownership not moved, s is a non-owning pointer: {s}");
}

fn main() {
    /*
     * 1. Write fn consume(s: String) -> usize and call it twice with the same variable. Read the full
     *    error (E0382, use of moved value). Write, as a comment above the call, what moved, where it
     *    moved to, and who tried to use it afterwards.
     */

    let str_1 = String::from("hello first objective!");
    consume_1(str_1);

    // A second call will not compile >>> error[E0382]: use of moved value: `str_1`
    // 18 |     let str_1 = String::from("hello first objective!");
    //    |         --- move occurs because `str_1` has type `String`, which does not implement the `Copy` trait
    // 19 |
    // 20 |     consume(str_1);
    //    |             --- value moved here
    // ...
    // 30 |     consume(str_1);
    //    |             ^^^^^ value used here after move
    //    |
    // consume_1(str_1); // <<< commented out after first objective met

    println!("------------");

    /*
     * 2. Fix it three different ways: (a) have consume return the String back; (b) change it to borrow
     *    (&str); (c) .clone() at the call site. Write one sentence on the cost of each.
     */

    // 2a. Cost: Does not make a copy of the string in the heap, ownership of the string is reassigned to
    //           the variable in the stack; would suggest shadowing the same variable in this strategy,
    //           otherwise, a new variable would need to be created for every function call

    let str_2a = String::from("hello second objective, part a!");
    let str_2a = consume_2a(str_2a);

    // second use allowed, but since its return value is not captured, ownership of str_2a is lost
    consume_2a(str_2a);

    // consume_2a(str_2a); // <<< can't do this, causes the same E0382 error as step 1

    println!("------------");

    // 2b. Cost: Does not make a copy of the string in the heap like 2a, but there is no need to reassign
    //           the address of the string to a variable on the stack. The `consume` function can be
    //           called multiple times.

    let str_2b = String::from("hello second objective, part b!");
    consume_2b(&str_2b);
    consume_2b(&str_2b);

    println!("------------");

    /*
     * 3. Repeat with a Vec<i32>, then with a plain i32 and explain why the i32 version compiles
     *    unchanged.
     */

    /*
     *4. Define struct Noisy(&'static str) with an impl Drop that prints its name. Create several in
     *    nested scopes, move one into a function, and predict the print order before running.
     */
}
