
fn main() {
    let mut _v1 = "AAAABBBCCDAABBB".chars();
    let mut _v2 = "ABBCcAD".chars();
    let mut _v3 = vec![1,2,2,3,3];

    assert_eq!(unique_in_order(_v1), vec!['A','B','C','D','A','B']);
    assert_eq!(unique_in_order(_v2), vec!['A','B','C','c','A','D']);
    assert_eq!(unique_in_order(_v3), vec![1,2,3])

}

#[allow(dead_code)]
fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}

#[allow(dead_code)]
fn unique_in_order<T>(list: T ) -> Vec<T::Item> 
where
    T:std::iter::IntoIterator ,
    T::Item:std::cmp::PartialEq + std::fmt::Debug
{
    let mut result = Vec::new();
    
    for (idx,value) in list.into_iter().enumerate() {
       match idx {
           0 => result.push(value),
           _ => if result.last().unwrap() != &value { result.push(value) }
       };
    };
    result
}
   