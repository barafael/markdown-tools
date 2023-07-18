#[derive(Debug)]
pub struct Thing<'a> {
    text: &'a str,
}

impl<'a> Thing<'a> {
    pub fn mutate(&mut self) -> Option<&str> {
        todo!()
    }
}

#[derive(Debug)]
pub struct ThingIterator<'a, I> {
    state: Thing<'a>,
    iter: I,
}

impl<'a, I> Iterator for ThingIterator<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.state.mutate()
    }
}
