#![allow(improper_ctypes_definitions)]
mod local_ecs;
mod entities;
mod systems;

extern "C" {
    // Main function of the dynamically linked library / Toxoid App.
    pub fn app_main();
}

fn main() {
    // Initialize Toxoid ECS initializers + default components + default sy stems.
    toxoid_ffi::init();

    // Initialize all engine default entities. Such as rendering, input, etc.
    entities::init();
    
    // TODO: Move to dynamically linked lib
    systems::init();
}

#[no_mangle]
pub unsafe extern "C" fn app_init() {
    // Main function of the dynamically linked library / Toxoid App.
    app_main();
    
    // Start update loop / game loop / render loop.
    toxoid_ffi::start();
}
