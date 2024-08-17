use bevy::prelude::*;

use crate::prelude::*;
use crate::ai_framework::*;
use crate::ai::*;

pub struct AiSystemPlugin;

impl Plugin for AiSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_ai_timer);

        app.add_systems(PreUpdate, pre_update_ai);
    }
}

#[derive(Debug, Component)]
struct AiTimer(Timer);

fn startup_ai_timer(mut commands: Commands) {
    commands.spawn(
        AiTimer(Timer::from_seconds(1.0 / (GAME_TICK as f32), TimerMode::Repeating))
    );
}

fn pre_update_ai(
    game_resource: Res<GameResource>,
    time         : Res<Time>,

    mut time_q     : Query<&mut AiTimer>,
    mut transform_q: Query<&mut TransformComponent>,
) {
    // 限制Tick数
    // 因为需要播放动画等原因，所以不能直接不能暂停其他所有Systems
    time_q.single_mut().0.tick(time.delta());
    if !time_q.single_mut().0.just_finished() { // 每个Tick只执行一次
        return;
    }

    // println!("AiSystemPlugin::pre_update_ai: teams = {:#?}", game_resource.teams);

    // 遍历每个Team
    game_resource.teams.iter().for_each(|team| {

        // 1. 构造GameController
        let mut game_controller = GameController::new(team);

        // 2. 执行每个Team的AI
        match team.id {
            0 => ai_team_0(&mut game_controller),
            1 => ai_team_1(&mut game_controller),
            2 => ai_team_2(&mut game_controller),
            3 => ai_team_3(&mut game_controller),
            4 => ai_team_4(&mut game_controller),
            5 => ai_team_5(&mut game_controller),
            6 => ai_team_6(&mut game_controller),
            7 => ai_team_7(&mut game_controller),
            8 => ai_team_8(&mut game_controller),
            _ => panic!("Not support team_id >= 9"),
        }

        // 3. 根据control_events，执行对应的操作，控制角色
        // => 【待定：执行动作时，必须间接改变操作数据，不能直接改变数据。例如，必须通过修改速度，来改变位置】
        game_controller.control_events_retain(|control_event| { match control_event {
            ControlEvent::MoveSlime { slime, direction } => {
                let mut logic_transform = transform_q.get_mut(slime.0).unwrap();

                match direction {
                    MoveDirection::Up    => logic_transform.z += 1,
                    MoveDirection::Down  => logic_transform.z -= 1,
                    MoveDirection::Left  => logic_transform.x -= 1,
                    MoveDirection::Right => logic_transform.x += 1,
                }

                println!("AiSystemPlugin::pre_update_ai: MoveSlime: slime = {:#?}, direction = {:#?}", slime, direction);

                return false;
            },
        }});
    });


}