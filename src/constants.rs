use bevy::math::Vec3;
use starknet_crypto::Felt;

// ALL LOCAL CONSTANTS

// Player accounts
pub const P1_ADDRESS: &str = "0xb3ff441a68610b30fd5e2abbf3a1548eb6ba6f3559f2862bf2dc757e5828ca";
pub const P1_PK: &str = "0x2bbf4f9fd0bbb2e60b0316c1fe0b76cf7a4d0198bd493ced9b8df2a3a24d68a";

pub const P2_ADDRESS: &str = "0xe29882a1fcba1e7e10cad46212257fea5c752a4f9b1b1ec683c503a2cf5c8a";
pub const P2_PK: &str = "0x14d6672dcb4b77ca36a887e9a11cd9d637d5012468175829e9c6e770c61642";

pub const P3_ADDRESS: &str = "0x29873c310fbefde666dc32a1554fea6bb45eecc84f680f8a2b0a8fbb8cb89af";
pub const P3_PK: &str = "0xc5b2fcab997346f3ea1c00b002ecf6f382c5f9c9659a3894eb783c5320f912";

pub const P4_ADDRESS: &str = "0x2d71e9c974539bb3ffb4b115e66a23d0f62a641ea66c4016e903454c8753bbc";
pub const P4_PK: &str = "0x33003003001800009900180300d206308b0070db00121318d17b5e6262150b";

pub const GAME_SYSTEM_CONTRACT_ADDRESS: &str =
    "0x34a3bf116ba899adcc24e885548dcd981aa96c0aeac9ccf551429fd0c6f91cf";

// Starknet.rs contstants for sending transactions
pub const STARKNET_RS_JSONRPC_URL: &str = "http://localhost:5050/";

// Torii contstants for receiving entity updates
pub const TORII_URL: &str = "http://localhost:8080";
pub const TORII_RPC_URL: &str = "http://localhost:5050";
pub const TORII_RELAY_URL: &str = "/ip4/10.175.151.36/tcp/9090";
pub const TORII_WORLD_CONTRACT: &str =
    "0x5d97c46d046f442f125b6cc83057e97ee6e848c4921126acd8ae9d17b55b369";

pub const EXISTING_ENTITY_QUERY_LIMIT: u32 = 500;