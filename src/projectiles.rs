use crate::prelude::*;

pub fn spawn_projectiles(world: &mut World, commands: &mut CommandBuffer) {
    for (id, (state, physics, character)) in
        world.query_mut::<(&mut StateMachine, &Physics, &Character)>()
    {
        if state.context.spawn.is_some()
            && state.context.elapsed == state.context.spawn.as_ref().unwrap().timing
        {
            if let Some(mut projectile) = state.context.spawn.take() {
                println!("owner {:?}", id);
                if let Some(action) = find_action(character, &projectile.name) {
                    let mut animator = Animator::new(character.data.origin, 3, physics.facing_left);
                    animator.keyframes.clone_from(&action.timeline);
                    let state = StateMachine {
                        processor: StateProcessor::new(Box::new(Fireball)),
                        context: Context {
                            elapsed: 1,
                            duration: projectile.duration,
                            ..Default::default()
                        },
                    };

                    projectile.owner.replace(id);
                    commands.spawn((
                        projectile.physics,
                        animator,
                        state,
                        action.clone(),
                        projectile.clone(),
                    ));
                    println!("Spawning {}", projectile.name);
                }
            }
        }
    }

    commands.run_on(world);
}

pub struct Fireball;
impl State for Fireball {
    fn name(&self) -> String {
        "Obj Fireball".into()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Obj Fireball on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        if context.ctx.reaction.has_hit {
            println!("Obj Fireball hit");
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Obj Fireball on_exit");
    }
}
