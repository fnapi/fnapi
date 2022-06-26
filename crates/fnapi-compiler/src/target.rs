/// The target of **server**.
pub trait Target {}

pub struct NextJs {}

impl Target for NextJs {}

pub struct AwsLambda {}

impl Target for AwsLambda {}
