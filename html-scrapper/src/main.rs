use std::{
    alloc::{alloc, Layout},
    ops::Deref,
};

use reqwest::{self};
use scraper::{html::Select, Html, Selector};

struct Data {
    data: Vec<String>,
}

impl Data {
    fn iter(&self) -> DataItertor {
        DataItertor {
            data: self,
            index: 0,
        }
    }
}

struct DataItertor<'a> {
    data: &'a Data,
    index: usize,
}

fn bzero(ptr: *mut u8, size: usize) -> *mut u8 {
    unsafe {
        let mut i = 0;
        while i < size {
            *ptr.offset(i as isize) = 1;
            i += 1;
        }
    }
    ptr
}

fn main_fn() {
    unsafe {
        let layout = Layout::from_size_align_unchecked(30, 4);

        let ptr = alloc(layout) as *mut u8;

        bzero(ptr, 30);
        let mut i = 0;
        while i < 30 {
            println!("index: {} => {:?}", i, ptr.offset(i as isize));
            i += 1;
        }
    }
}

impl<'a> Iterator for DataItertor<'a> {
    type Item = &'a String;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.data.data.get(self.index);
        self.index += 1;
        item
    }
}

#[tokio::main]
async fn main() {
    // let base_url = "https://normoyun.com/manga?page=";
    // let page = 1;
    // let url = format!("{}{}", base_url, page);
    // let response = reqwest::get(&url).await.unwrap();
    // if response.status().is_success() {
    //     let body = response.text().await.unwrap();
    //     let doc = Html::parse_document(&body);
    //     let s = Selector::parse("div.bsx > a").unwrap();
    //     for element in doc.select(&s) {
    //         println!("Item: {}", element.value().attr("href").unwrap());
    //     }
    // }

    main_fn();
}
