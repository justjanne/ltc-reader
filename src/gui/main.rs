use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use gtk::traits::{BoxExt, GtkWindowExt};
use relm::{channel, ComponentParts, ComponentSender, Receiver, RelmApp, RelmWidgetExt, SimpleComponent};

use ltc_reader::{connect_stream, LtcFrame};

#[derive(Debug)]
enum AppInput {
    Update(LtcFrame, u128),
    Error(cpal::StreamError),
}

struct AppModel {
    timecode: Option<LtcFrame>,
    fps: Option<u128>,
}

struct AppWidgets {
    label_timecode: gtk::Label,
    label_fps: gtk::Label,
}

impl SimpleComponent for AppModel {
    /// The type of the messages that this component can receive.
    type Input = AppInput;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = Receiver<AppInput>;
    /// The root GTK widget that this component will create.
    type Root = gtk::Window;
    /// A data structure that contains the widgets that you will need to update.
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("Simple app")
            .default_width(300)
            .default_height(100)
            .build()
    }

    /// Initialize the UI and model.
    fn init(
        receiver: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel { timecode: None, fps: None };

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();

        let label_fps = gtk::Label::new(
            Some(
                model.fps.map(|fps| format!("{} fps", fps))
                    .unwrap_or("?? fps".to_string())
                    .as_str()
            )
        );
        label_fps.set_margin_all(5);
        let label_timecode = gtk::Label::new(
            Some(
                model.timecode.clone().map(|frame| frame.to_string())
                    .unwrap_or("00:00:00:00".to_string())
                    .as_str()
            )
        );
        label_timecode.set_margin_all(5);

        window.set_child(Some(&vbox));
        vbox.set_margin_all(5);
        vbox.append(&label_fps);
        vbox.append(&label_timecode);

        sender.clone().spawn_oneshot_command(move || {
            while let Some(event) = receiver.recv_sync() {
                let _ = sender.input_sender().send(event);
            }
        });

        let widgets = AppWidgets { label_fps, label_timecode };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::Update(timecode, fps) => {
                self.timecode = Some(timecode);
                self.fps = Some(fps);
            }
            AppInput::Error(_) => {}
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        widgets.label_fps.set_label(
            self.fps.map(|fps| format!("{} fps", fps))
                .unwrap_or("?? fps".to_string())
                .as_str()
        );
        widgets.label_timecode.set_label(
            self.timecode.clone().map(|frame| frame.to_string())
                .unwrap_or("00:00:00:00".to_string())
                .as_str()
        );
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple_manual");
    let (audio_sender, audio_receiver) = channel::<AppInput>();
    let sender_input = audio_sender.clone();
    let sender_error = audio_sender.clone();

    let host = cpal::default_host();
    let device = host.default_input_device().expect("no input device available");
    let default_config = device.default_input_config().expect("no supported config");
    let stream = connect_stream(
        &device,
        &default_config.config(),
        move |(frame, fps)| {
            let _ = sender_input.send(AppInput::Update(frame, fps));
        },
        move |err| {
            let _ = sender_error.send(AppInput::Error(err));
        },
    ).expect("could not connect to audio stream");
    stream.play().expect("could not start stream");

    app.run::<AppModel>(audio_receiver);
}
