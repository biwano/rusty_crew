use crate::collision::Collidable;
use crate::ship::Ship;
use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthBarFill;

#[derive(Resource)]
pub struct PlayerScore {
    pub score: u32,
}

impl Default for PlayerScore {
    fn default() -> Self {
        Self { score: 0 }
    }
}

pub fn setup_hud(mut commands: Commands) {
    // Text used to show controls
    commands.spawn((
        Text::new("CONTROLS:\nZQSD - Move\nQ/E - Rotate\n1/2 - Switch Weapons\nSPACE - Fire"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        },
    ));

    // Score display
    commands.spawn((
        ScoreDisplay,
        Text::new("Score: 0"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            right: px(12),
            ..default()
        },
    ));

    // Health bar container
    commands
        .spawn((
            HealthBar,
            Node {
                position_type: PositionType::Absolute,
                bottom: px(20),
                left: Val::Percent(50.0),
                width: px(200),
                height: px(20),
                margin: UiRect::left(Val::Px(-100.0)), // Center the bar
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)), // Dark gray background
        ))
        .with_children(|parent| {
            // Health bar fill (red)
            parent.spawn((
                HealthBarFill,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(1.0, 0.0, 0.0)), // Red fill
            ));
        });
}

pub fn update_score_display(
    player_score: Res<PlayerScore>,
    mut score_text_query: Query<&mut Text, With<ScoreDisplay>>,
) {
    for mut text in score_text_query.iter_mut() {
        *text = Text::new(format!("Score: {}", player_score.score));
    }
}

pub fn update_health_bar(
    ship_query: Query<&Collidable, With<Ship>>,
    mut health_bar_fill_query: Query<&mut Node, With<HealthBarFill>>,
) {
    if let Ok(collidable) = ship_query.single() {
        if let Ok(mut health_bar_fill_node) = health_bar_fill_query.single_mut() {
            // Calculate health percentage
            let health_percentage =
                (collidable.hit_points / collidable.max_hit_points).clamp(0.0, 1.0);

            // Update the width of the health bar fill
            health_bar_fill_node.width = Val::Percent(health_percentage * 100.0);
        }
    }
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerScore>()
            .add_systems(Startup, setup_hud)
            .add_systems(Update, (update_score_display, update_health_bar));
    }
}
