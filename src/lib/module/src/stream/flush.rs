/// Advice system when to flush buffered output
#[derive(Debug, PartialEq)]
pub enum Flush {
	/// Let the system decide when to flush buffered output
	Auto,
	/// Force flush buffered output
	Force,
}
