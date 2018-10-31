#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize)]
pub struct Status {
	pub date: i64,
	pub update_date: Option<i64>,
	pub eta: Option<i64>,
	pub close_date: Option<i64>,
	pub title: String,
	pub message: String,
}

impl Status {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
		self.title = title.into();
		self
	}

	pub fn with_message<S: Into<String>>(mut self, message: S) -> Self {
		self.message = message.into();
		self
	}

	pub fn set_title<S: Into<String>>(&mut self, title: S) {
		self.title = title.into();
	}

	pub fn set_message<S: Into<String>>(&mut self, message: S) {
		self.message = message.into();
	}
}
