
use super::{Crabby, MageFriend, do_attack};

pub fn app() {
    let crabby = Crabby;
    let mage_friend = MageFriend;

    do_attack(crabby);
    do_attack(mage_friend);
}