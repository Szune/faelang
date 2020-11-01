use std::collections::HashMap;

pub enum Value {
	/*I64(i64),
	U64(u64),
	U8(u8),
	F64(f64),
	F32(f32),
	 */
	Unit,
    Integer(i64),
	Float(f64),
	String(String),
	List(Vec<Value>),
	Map(HashMap<Value,Value>),
	Bool(bool),
}
