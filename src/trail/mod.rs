use smash::{
    hash40,
    app::{lua_bind::*, *},
    lib::lua_const::*
};

#[skyline::hook( replace = FighterSpecializer_Trail::change_magic )]
pub unsafe fn trail_change_magic(fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    if !WorkModule::is_flag(module_accessor, 0x2100000c) {
        return;
    }
    let rand = sv_math::rand(hash40("fighter"), 3);
    WorkModule::set_int(module_accessor, rand, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND);
    original!()(
        fighter
    )
}

pub fn install() {
    // skyline::install_hooks!(
    //     trail_change_magic
    // );
    trail_change_magic::install();
}