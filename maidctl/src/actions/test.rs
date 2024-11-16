use clio::Input;

pub trait TestPayload {}

pub struct NoTestPayload();

impl TestPayload for NoTestPayload {}

pub struct StringTestPayload(pub String);

impl TestPayload for StringTestPayload {}

pub struct FileTestPayload(pub Input);

impl TestPayload for FileTestPayload {}

pub fn test<P: TestPayload>(_name_or_url: String, _payload: P) {}
