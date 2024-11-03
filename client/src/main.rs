use std::time::Duration;
use state::ClientToServerMsg;
use tokio::net::TcpStream;
mod state;
use state::AppMode;
use state::AppState;
fn panic_handler(args: &std::panic::PanicHookInfo) {
    crossterm::terminal::disable_raw_mode().expect("Failed to enter raw terminal mode");
    if let Some(s) = args.payload().downcast_ref::<&str>() {
        println!("{:?}: {s}", args.location());
    } else if let Some(s) = args.payload().downcast_ref::<String>() {
        println!("{:?}: {s}", args.location());
    }
}
#[tokio::main]
async fn main() {
    std::panic::set_hook(Box::new(&panic_handler));
    crossterm::terminal::enable_raw_mode().expect("Failed to enter raw terminal mode");

    let mut state = AppState::default();
    loop {
        if crossterm::event::poll(Duration::from_millis(100)).unwrap() {
            let event = crossterm::event::read().expect("Couldnt find event");
            match match state.mode {
                AppMode::Visual => visual(event, &mut state).await,
                AppMode::Type(_) => text(event, &mut state).await,
                AppMode::Connect(_) => connect(event, &mut state).await,
            } {
                Response::None => {}
                Response::Exit => break,
            }
        }
        println!("Current appmode: {state:?}\r");
    }
    crossterm::terminal::disable_raw_mode().expect("Couldnt exit raw terminal mode");
}
enum Response {
    None,
    Exit,
}
async fn timeout(duration: Duration) {
    tokio::time::sleep(duration).await;
}
async fn connect(event: crossterm::event::Event, state: &mut AppState) -> Response {
    match event {
        crossterm::event::Event::Key(key_event) => match key_event.code {
            crossterm::event::KeyCode::Backspace => {
                state.mode.force_connect().unwrap().pop();
                Response::None
            }
            crossterm::event::KeyCode::Enter => {
                let mut url = state.mode.force_connect().unwrap().as_str().to_string();
                let timeout_duration = Duration::from_secs(1);
                let mut stream = tokio::select! {
                    _ = timeout(timeout_duration) => {
                        None
                    }
                    stream = TcpStream::connect(&url) => {
                        stream.ok()
                    }
                };
                if stream.is_none() {
                    url.push_str(":65432");
                    tokio::select! {
                        _ = timeout(timeout_duration) => {}
                        new_stream = TcpStream::connect(&url) => {
                            stream = new_stream.ok();
                        }
                    };
                }
                match stream {
                    Some(stream) => {
                        state.connection = Some(net_message::asymmetric::AsymmetricTcpStream::new(stream.into_std().expect("Failed to convert to std TcpStream"), Duration::from_secs(1)).unwrap());
                        state.mode = AppMode::Visual;
                        Response::None
                    }
                    None => {
                        println!("Failed to connect, try adding the port (\":65432\") to the end of the URL i.e. \"website.com:65432\"");
                        return Response::None;
                    }
                }
            }
            crossterm::event::KeyCode::Left => todo!(),
            crossterm::event::KeyCode::Right => todo!(),
            crossterm::event::KeyCode::Up => todo!(),
            crossterm::event::KeyCode::Down => todo!(),
            crossterm::event::KeyCode::Home => todo!(),
            crossterm::event::KeyCode::End => todo!(),
            crossterm::event::KeyCode::PageUp => todo!(),
            crossterm::event::KeyCode::PageDown => todo!(),
            crossterm::event::KeyCode::Tab => todo!(),
            crossterm::event::KeyCode::BackTab => todo!(),
            crossterm::event::KeyCode::Delete => todo!(),
            crossterm::event::KeyCode::Insert => todo!(),
            crossterm::event::KeyCode::F(_) => todo!(),
            crossterm::event::KeyCode::Char(chr) => {
                state.mode.force_connect().unwrap().push(chr);
                Response::None
            }
            crossterm::event::KeyCode::Null => todo!(),
            crossterm::event::KeyCode::Esc => {
                state.mode = AppMode::Visual;
                Response::None
            }
            crossterm::event::KeyCode::CapsLock => todo!(),
            crossterm::event::KeyCode::ScrollLock => todo!(),
            crossterm::event::KeyCode::NumLock => todo!(),
            crossterm::event::KeyCode::PrintScreen => todo!(),
            crossterm::event::KeyCode::Pause => todo!(),
            crossterm::event::KeyCode::Menu => todo!(),
            crossterm::event::KeyCode::KeypadBegin => todo!(),
            crossterm::event::KeyCode::Media(media_key_code) => todo!(),
            crossterm::event::KeyCode::Modifier(modifier_key_code) => todo!(),
        },
        crossterm::event::Event::Mouse(mouse_event) => {
            println!("{mouse_event:#?}");
            Response::None
        }
        crossterm::event::Event::Paste(data) => Response::None,
        _ => Response::None,
    }
}
async fn text(event: crossterm::event::Event, state: &mut AppState) -> Response {
    match event {
        crossterm::event::Event::Key(key_event) => match key_event.code {
            crossterm::event::KeyCode::Backspace => {
                state.mode.force_type().unwrap().pop();
                Response::None
            }
            crossterm::event::KeyCode::Enter => {
                let stream = state.connection.as_mut().expect("Not connected");
                stream.send(ClientToServerMsg::Message(state.mode.force_type().unwrap().to_owned())).unwrap();
                state.mode = AppMode::Visual;
                Response::None
            },
            crossterm::event::KeyCode::Left => todo!(),
            crossterm::event::KeyCode::Right => todo!(),
            crossterm::event::KeyCode::Up => todo!(),
            crossterm::event::KeyCode::Down => todo!(),
            crossterm::event::KeyCode::Home => todo!(),
            crossterm::event::KeyCode::End => todo!(),
            crossterm::event::KeyCode::PageUp => todo!(),
            crossterm::event::KeyCode::PageDown => todo!(),
            crossterm::event::KeyCode::Tab => todo!(),
            crossterm::event::KeyCode::BackTab => todo!(),
            crossterm::event::KeyCode::Delete => todo!(),
            crossterm::event::KeyCode::Insert => todo!(),
            crossterm::event::KeyCode::F(_) => todo!(),
            crossterm::event::KeyCode::Char(chr) => {
                state.mode.force_type().unwrap().push(chr);
                Response::None
            }
            crossterm::event::KeyCode::Null => todo!(),
            crossterm::event::KeyCode::Esc => {
                state.mode = AppMode::Visual;
                Response::None
            }
            crossterm::event::KeyCode::CapsLock => todo!(),
            crossterm::event::KeyCode::ScrollLock => todo!(),
            crossterm::event::KeyCode::NumLock => todo!(),
            crossterm::event::KeyCode::PrintScreen => todo!(),
            crossterm::event::KeyCode::Pause => todo!(),
            crossterm::event::KeyCode::Menu => todo!(),
            crossterm::event::KeyCode::KeypadBegin => todo!(),
            crossterm::event::KeyCode::Media(media_key_code) => todo!(),
            crossterm::event::KeyCode::Modifier(modifier_key_code) => todo!(),
        },
        crossterm::event::Event::Mouse(mouse_event) => {
            println!("{mouse_event:#?}");
            Response::None
        }
        crossterm::event::Event::Paste(data) => Response::None,
        _ => Response::None,
    }
}
async fn visual(event: crossterm::event::Event, state: &mut AppState) -> Response {
    match event {
        crossterm::event::Event::Key(key_event) => match key_event.code {
            crossterm::event::KeyCode::Backspace => todo!(),
            crossterm::event::KeyCode::Enter => todo!(),
            crossterm::event::KeyCode::Left => todo!(),
            crossterm::event::KeyCode::Right => todo!(),
            crossterm::event::KeyCode::Up => todo!(),
            crossterm::event::KeyCode::Down => todo!(),
            crossterm::event::KeyCode::Home => todo!(),
            crossterm::event::KeyCode::End => todo!(),
            crossterm::event::KeyCode::PageUp => todo!(),
            crossterm::event::KeyCode::PageDown => todo!(),
            crossterm::event::KeyCode::Tab => todo!(),
            crossterm::event::KeyCode::BackTab => todo!(),
            crossterm::event::KeyCode::Delete => todo!(),
            crossterm::event::KeyCode::Insert => todo!(),
            crossterm::event::KeyCode::F(_) => todo!(),
            crossterm::event::KeyCode::Char(chr) => match chr {
                'q' => Response::Exit,
                'i' => {
                    state.mode = AppMode::Type("".to_string());
                    Response::None
                }
                'c' => {
                    state.mode = AppMode::Connect("".to_string());
                    Response::None
                }
                _ => Response::None,
            },
            crossterm::event::KeyCode::Null => todo!(),
            crossterm::event::KeyCode::Esc => Response::None,
            crossterm::event::KeyCode::CapsLock => todo!(),
            crossterm::event::KeyCode::ScrollLock => todo!(),
            crossterm::event::KeyCode::NumLock => todo!(),
            crossterm::event::KeyCode::PrintScreen => todo!(),
            crossterm::event::KeyCode::Pause => todo!(),
            crossterm::event::KeyCode::Menu => todo!(),
            crossterm::event::KeyCode::KeypadBegin => todo!(),
            crossterm::event::KeyCode::Media(media_key_code) => todo!(),
            crossterm::event::KeyCode::Modifier(modifier_key_code) => todo!(),
        },
        crossterm::event::Event::Mouse(mouse_event) => {
            println!("{mouse_event:#?}");
            Response::None
        }
        crossterm::event::Event::Paste(data) => Response::None,
        _ => Response::None,
    }
}
