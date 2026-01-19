use tokio::sync::oneshot;
use crate::domain::events::event::Event;

pub enum PipelineCommand {
    Append {
        stream_id: String,
        events: Vec<Event>,
        expected_version: i64,
        resp_tx: oneshot::Sender<Result<bool, String>>,
    },
}
