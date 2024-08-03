use crate::message::poll::answer::PollAnswer;
use crate::message::poll::layout_type::PollLayoutType;
use crate::message::poll::media::PollMedia;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollCreate {
    /// The question of the poll. Only text is supported.
    question: PollMedia,
    /// Each of the answers available in the poll, up to 10
    answers: Vec<PollAnswer>,
    /// Number of hours the poll should be open for, up to 32 days. Defaults to 24
    duration: Option<u32>,
    /// Whether a user can select multiple answers. Defaults to false
    allow_multiselect: Option<bool>,
    /// The layout type of the poll. Defaults to... DEFAULT!
    layout_type: Option<PollLayoutType>,
}

impl PollCreate {
    pub fn new(question: PollMedia) -> PollCreate {
        Self {
            question,
            answers: vec![],
            duration: None,
            allow_multiselect: None,
            layout_type: None,
        }
    }

    pub fn answer<Func>(mut self, function: Func) -> Self
    where
        Func: FnOnce(PollMedia) -> PollMedia,
    {
        self.answers
            .push(PollAnswer::new(function(PollMedia::new())));
        self
    }
}
