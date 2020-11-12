pub fn flattern<O>(outer: O) -> Flattern<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    Flattern::new(outer)
}

pub struct Flattern<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flattern<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(outer: O) -> Flattern<O> {
        Flattern {
            outer: outer,
            inner: None,
        }
    }
}

impl<O> Iterator for Flattern<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        loop {
            if let Some(ref mut inner_itr) = self.inner {
                if let Some(v) = inner_itr.next() {
                    return Some(v);
                }
            }
            if let Some(outer_itr) = self.outer.next() {
                self.inner = Some(outer_itr.into_iter());
            } else {
                return None;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn empty() {
    println!("{:?}", Vec::<()>::new().into_iter().count());
    assert_eq!(flattern(Vec::<Vec<()>>::new().into_iter()).count(), 0);
}

#[test]
fn empty_many() {
    assert_eq!(
        flattern(vec![Vec::<()>::new(), vec![], vec![]].into_iter()).count(),
        0
    );
}

#[test]
fn once() {
    assert_eq!(flattern(vec![vec!["a"]].into_iter()).count(), 1);
}

#[test]
fn more() {
    assert_eq!(
        flattern(vec![vec!["a"], vec!["b"], vec!["d"]].into_iter()).count(),
        3
    );
}
