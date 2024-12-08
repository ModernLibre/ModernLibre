// this file contains the environment literals

pub const REDIS_URL: &str = "REDIS_URL";                        // Must be set
pub const POSTGRES_URL: &str = "POSTGRES_URL";                  // Must be set
pub const JWT_PUBLIC_KEY_PATH: &str = "JWT_PUBLIC_KEY_PATH";    // Default: ./public.pem
pub const JWT_PRIVATE_KEY_PATH: &str = "JWT_PRIVATE_KEY_PATH";  // Default: ./private.pem
pub const JWT_ALGORITHM: &str = "JWT_ALGORITHM";                // Default: RS256