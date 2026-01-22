/*[profile.release]
 * panic = 'abort'
 * Add the above to the cargo.toml to see more verbose errors
 * Adding this avoids rust's cleanup of the stack
*/

// lets induce a panic
fn main() {
    panic!("crash and burn");
}

// example of a panic call due to a bug in our code logic
fn panic_vec() {
    let v = vec![1, 2, 3];

    v[99];
}