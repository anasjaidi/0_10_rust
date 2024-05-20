use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use reqwest::{self};
use scraper::{html::Select, Html, Selector};

trait ScrapperMessage {}

enum ChannelData {}

#[tokio::main]
async fn main() {
    let (fetch_sender, main_r) = mpsc::channel::<ChannelData>();
    let parse_sender = fetch_sender.clone();
    let store_sender = fetch_sender.clone();
    let (mfetch_sender, fetch_rec) = mpsc::channel::<ChannelData>();
    let (mparse_sender, parse_rec) = mpsc::channel::<ChannelData>();
    let (mstore_sender, store_rec) = mpsc::channel::<ChannelData>();
    let fetch_handler = thread::spawn(move || fetch_data(fetch_sender, fetch_rec));
    let parse_handler = thread::spawn(move || parse_date(parse_sender, parse_rec));
    let store_handler = thread::spawn(move || store_date(store_sender, store_rec));
}

fn fetch_data(ch_s: Sender<ChannelData>, ch_r: Receiver<ChannelData>) {}
fn parse_date(ch_s: Sender<ChannelData>, ch_r: Receiver<ChannelData>) {}
fn store_date(ch_s: Sender<ChannelData>, ch_r: Receiver<ChannelData>) {}
