use minigene_core::*;

fn main() {
    let mut world = World::default();
    let dispatcher = DispatcherBuilder::default()
        .add(|| {println!("Running!"); Ok(())})
        .build(&mut world);
    mini_loop_simple(world, dispatcher);
}
