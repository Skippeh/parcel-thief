use std::{path::Path, time::Duration};

use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, PollWatcher, RecursiveMode, Watcher};

pub struct FileWatcher {
    watcher: PollWatcher,
    rx: Receiver<Result<Event, notify::Error>>,
}

impl FileWatcher {
    pub fn new() -> Result<Self, notify::Error> {
        let (mut tx, rx) = channel(1);
        let watcher = PollWatcher::new(
            move |res| {
                futures::executor::block_on(async {
                    tx.send(res).await.unwrap();
                })
            },
            Config::default().with_poll_interval(Duration::from_millis(500)),
        )?;

        Ok(Self { watcher, rx })
    }

    pub fn watch(
        &mut self,
        path: impl AsRef<Path>,
        recursive_mode: RecursiveMode,
    ) -> notify::Result<()> {
        self.watcher.watch(path.as_ref(), recursive_mode)
    }

    pub async fn next(&mut self) -> notify::Result<Event> {
        self.rx.next().await.unwrap()
    }
}
