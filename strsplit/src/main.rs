struct strSplit<'a, 'b> {
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

fn first_word(haystack: &str) -> &str {
    strSplit::new(haystack, &String::from(" ")).next().unwrap()
}

fn main() {
    let haystack = String::from("anas jaidi wahed kouna");
    let needle = &String::from(" ");
    let mut str_split = strSplit::new(&haystack, needle);
    for item in str_split {
        println!("{}", item);
    }
}
