//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::users::Users;

//> HEAD -> CORE
use core::sync::atomic::Ordering;


//^
//^ WRITEHANDLE
//^

//> WRITEHANDLE -> STRUCT
pub struct WriteHandle<'valid> {
    pub users: &'valid Users
}

//> WRITEHANDLE -> DROP
impl<'valid> Drop for WriteHandle<'valid> {
    fn drop(&mut self) {self.users.users.store(1, Ordering::Release)}
}