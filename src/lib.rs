use anagma::metadata::stream::value::ValueStream;
use anagma::metadata::stream::Error as StreamError;
use anagma::metadata::value::Value;

pub enum UnaryOp {
    Count,
    First,
    Last,
}

impl UnaryOp {
    pub fn process(&self, value_stream: ValueStream) -> Result<Value, StreamError> {
        Ok(Value::Null)
    }
}

#[cfg(test)]
mod tests {
}
