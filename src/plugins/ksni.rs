use ksni::TrayMethods;
use crate::core::thread_messages::GUIMessage;
use gettextrs::gettext;

pub struct SystrayInterface {
    pub gui_tx: async_channel::Sender<GUIMessage>,
}

impl ksni::Tray for SystrayInterface {
    fn id(&self) -> String {
        "songrec".into()
    }
    fn icon_name(&self) -> String {
        "re.fossplant.songrec".into()
    }
    fn title(&self) -> String {
        "SongRec".into()
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            StandardItem {
                label: gettext("Open SongRec").into(),
                activate: Box::new(|tray: &mut Self| {
                    tray.gui_tx.try_send(GUIMessage::ShowWindow).unwrap();
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: gettext("Quit...").into(),
                activate: Box::new(|tray: &mut Self| {
                    tray.gui_tx.try_send(GUIMessage::QuitApplication).unwrap();
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}

impl SystrayInterface {
    pub async fn try_enable(
        gui_tx: async_channel::Sender<GUIMessage>,
    ) -> Result<ksni::Handle<Self>, ksni::Error> {
        Self {
            gui_tx,
        }
        .spawn()
        .await
    }

    pub async fn disable(handle: &ksni::Handle<Self>) {
        handle.shutdown().await;
    }
}
