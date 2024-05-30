pub struct strSplit<'a, 'b> {
    haystack: Option<&'a str>,
    needle: &'b str,
}

impl<'a, 'b> strSplit<'a, 'b> {
    fn new(haystack: &'a str, needle: &'b str) -> Self {
        Self {
            haystack: Some(haystack),
            needle,
        }
    }
}

fn execute_with_any_lifetime<F>(f: F)
where
    F: for<'a> Fn(&'a str, &'a str) -> &'a str,
{
    // Call the closure with different lifetimes
    let s1: &str = "hello";
    f(s1, s1); // Call the closure with a reference to `s1`
    let s2: String = "world".to_string();
    f(&s2, s1); // Call the closure with a reference to `s2`
}

impl<'a> Iterator for strSplit<'a, '_> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let rem = self.haystack.as_mut()?;
        if let Some(del) = rem.find(self.needle) {
            let str = &rem[..(del + self.needle.len())];
            *rem = &rem[(del + self.needle.len())..];
            Some(str)
        } else {
            self.haystack.take()
        }
    }
}

pub trait LastWord<'a> {
    fn last(&mut self) -> Option<&'a str>;
}

impl<'a> LastWord<'a> for strSplit<'a, '_> {
    fn last(&mut self) -> Option<&'a str> {
        let mut last = None;

        for l in self {
            last = Some(l);
        }
        last
    }
}

pub fn last_word<'a, T>(mut str: T) -> Option<&'a str>
where
    T: LastWord<'a>,
{
    str.last()
}

pub fn first_word(haystack: &str) -> &str {
    strSplit::new(haystack, &String::from(" ")).next().unwrap()
}

pub fn bigger_text<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str
where
    'b: 'a,
{
    if s1.len() > s1.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let haystack = String::from("anas jaidi wahed kouna");
    let needle = &String::from(" ");
    let str_split = strSplit::new(&haystack, needle);
    for item in str_split {
        println!("{}", item);
    }
}
