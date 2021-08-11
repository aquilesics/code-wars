fn main() {
    let mut _v1 = vec![1,2,3];
    let mut _v2 = vec![3,4,5];

    print!("{:?}", array_diff(_v1, _v2));

}
fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}