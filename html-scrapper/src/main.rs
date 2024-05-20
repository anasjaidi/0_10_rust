use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

enum ChannelData {}

#[tokio::main]
async fn main() {
    let (main_sender, main_receiver) = mpsc::channel::<ChannelData>();
    let mut handlers = vec![];

    let mut spawn_task = |i: usize, task: fn(Sender<ChannelData>, Receiver<ChannelData>) -> ()| {
        let (_, task_receiver) = mpsc::channel::<ChannelData>();
        let main_sender_clone = main_sender.clone();
        let handler = thread::spawn(move || task(main_sender_clone, task_receiver));
        handlers.push(handler);
    };
    for (i, &t) in [fetch_data, parse_date, store_date].iter().enumerate() {
        spawn_task(i, t);
    }

    while handlers.iter().all(|h| h.is_finished()) {
        match main_receiver.try_recv() {
            Ok(data) => {}
            Err(err) => {}
        }
    }
}

fn fetch_data(ch_s: Sender<ChannelData>, ch_r: Receiver<ChannelData>) {}
fn parse_date(ch_s: Sender<ChannelData>, ch_r: Receiver<ChannelData>) {}
fn store_date(ch_s: Sender<ChannelData>, ch_r: Receiver<ChannelData>) {}
