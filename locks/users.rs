//^
//^ HEAD
//^

//> HEAD -> CORE
use core::sync::atomic::{
    Atomic,
    Ordering
};

//> HEAD -> SUPER
use super::{
    writehandle::WriteHandle,
    readhandle::ReadHandle
};


//^
//^ USERS
//^

//> USERS -> STRUCT
pub struct Users {
    pub users: Atomic<usize> = Atomic::<usize>::new(1)
}

//> USERS -> IMPLEMENTATION
impl Users {
    pub fn write<'valid>(&'valid self) -> Option<WriteHandle<'valid>> {
        self.users.compare_exchange(
            1, 
            0, 
            Ordering::AcqRel, 
            Ordering::Acquire
        ).ok()?;
        return Some(WriteHandle {
            users: self
        });
    }
    pub fn read<'valid>(&'valid self) -> Option<ReadHandle<'valid>> {
        let now = self.users.load(Ordering::SeqCst);
        return if now != 0 {
            match self.users.compare_exchange(
                now, 
                now + 1, 
                Ordering::AcqRel, 
                Ordering::Acquire
            ) {
                Ok(_) => Some(ReadHandle {
                    users: self
                }),
                Err(_) => None
            }
        } else {None}
    }
}