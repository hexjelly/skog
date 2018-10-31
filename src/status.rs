#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize)]
pub struct Status {
	pub id: Option<i64>,
	pub date: Option<i64>,
	pub update_date: Option<i64>,
	pub eta: Option<i64>,
	pub close_date: Option<i64>,
	pub title: Option<String>,
	pub message: Option<String>,
}
