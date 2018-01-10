mod defs;


pub use self::defs::astero;
pub use self::defs::astero::mod_Server::OneOfMsg as AsteroServerMsg;
pub use self::defs::astero::mod_Client::OneOfMsg as AsteroClientMsg;
pub use self::defs::astero::mod_Create::OneOfEntity as AsteroCreateEntity;

pub use self::defs::mmob;
pub use self::mmob::mod_Client::OneOfMsg as MmobClientMsg;
pub use self::mmob::mod_Server::OneOfMsg as MmobServerMsg;


