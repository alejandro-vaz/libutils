//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::users::Users;

//> HEAD -> CORE
use core::sync::atomic::Ordering;


//^
//^ READHANDLE
//^

//> READHANDLE -> STRUCT
pub struct ReadHandle<'valid> {
    pub users: &'valid Users
}

//> READHANDLE -> DROP
impl<'valid> Drop for ReadHandle<'valid> {
    fn drop(&mut self) {
        self.users.users.fetch_sub(1, Ordering::AcqRel);
    }
}