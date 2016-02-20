#[macro_use]
extern crate rust_cql;

macro_rules! assert_response(
    ($resp:expr) => (
        if match $resp.opcode { rust_cql::OpcodeResponse::OpcodeError => true, _ => false } {
            panic!("Test failed at assertion: {}",
                match $resp.body { rust_cql::CqlResponseBody::ResponseError(_, message) => message, _ => Cow::Borrowed("Ooops!")});
        }
    );
);

macro_rules! try_test(
    ($call: expr, $msg: expr) => {
        match $call {
            Ok(val) => val,
            Err(ref err) => panic!("Test failed at library call: {}", err.description())
        };
    }
);

pub mod test_rust_cql;
