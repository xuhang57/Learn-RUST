fn main() {
    /*
     * This is another type of comment, a block comment. In general,
     * line comments are the recommended comment style. But
     * block comments are extremely useful for temporarily disabling
     * chunks of code. /* Block comments can be /* nested, */ */
     * so it takes only a few keystrokes to comment out everything
     * in this main() function. /*/*/* Try it yourself! */*/*/
     */

    /*
    Note: The previous column of `*` was entirely for style. There's
    no actual need for it.
    */

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x); // x = 10
}
