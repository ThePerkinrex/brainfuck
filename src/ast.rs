#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Code {
	INCR_POINTER(),
	DECR_POINTER(),

	INCR_BYTE(),
	DECR_BYTE(),

	READ(),
	WRITE(),

	LOOP(Vec<Self>),
}
