//! Security helpers live in their own module so authentication and policy code
//! do not get scattered across handlers. A production version would validate
//! JWTs, map mesh workload identity to service principals, and expose a small
//! `RequestIdentity` type to the domain layer.
