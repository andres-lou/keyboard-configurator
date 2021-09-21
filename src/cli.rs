use backend::{Backend, Board};
use futures::channel::oneshot;
use gtk::{glib::clone, prelude::*};
use std::{cell::RefCell, io, process, rc::Rc};

async fn backend_boards() -> (Backend, Vec<Board>) {
    let backend = Backend::new().expect("Failed to create server");

    let boards = Rc::new(RefCell::new(Vec::new()));
    let id1 = backend.connect_board_added(clone!(@strong boards => move |board| {
        boards.borrow_mut().push(board.clone());
    }));

    let (sender, receiver) = oneshot::channel::<()>();
    let sender = RefCell::new(Some(sender));
    let id2 = backend.connect_board_loading_done(move || {
        if let Some(sender) = sender.borrow_mut().take() {
            sender.send(()).unwrap();
        }
    });
    backend.refresh();
    receiver.await.unwrap();

    backend.disconnect(id1);
    backend.disconnect(id2);

    (backend, boards.take())
}

pub async fn list_boards() {
    let (_backend, boards) = backend_boards().await;

    for board in boards {
        println!("{}", board.display_name());
    }
}

pub async fn board() -> Board {
    let (backend, boards) = backend_boards().await;

    if boards.is_empty() {
        error!("No boards detected");
        process::exit(1);
    } else if boards.len() == 1 {
        boards[0].clone()
    } else {
        eprintln!("Multiple boards detected");
        for board in boards {
            // Human readable name?
            eprintln!("{}", board.display_name());
        }
        todo!()
    }
}

// usb:  bool
pub async fn save(path: String) {
    let (backend, boards) = backend_boards().await;

    boards.len();

    for board in boards {}
}

pub async fn load(path: String) {}

pub async fn reset() {}
