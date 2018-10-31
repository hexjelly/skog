#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize)]
pub struct Status {
	pub id: Option<i64>,
	pub date: i64,
	pub update_date: Option<i64>,
	pub eta: Option<i64>,
	pub close_date: Option<i64>,
	pub title: String,
	pub message: String,
}
