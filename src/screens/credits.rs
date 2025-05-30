//! A credits screen that can be accessed from the title screen.

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::Music,
    screens::Screen,
    theme::{interaction::OnPress, palette::SCREEN_BACKGROUND, prelude::*},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), spawn_credits_screen);

    app.register_type::<CreditsMusic>();
    app.load_resource::<CreditsMusic>();
    app.add_systems(OnEnter(Screen::Credits), start_credits_music);
    app.add_systems(OnExit(Screen::Credits), stop_credits_music);
}

fn spawn_credits_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert((StateScoped(Screen::Credits), BackgroundColor(SCREEN_BACKGROUND)))
        .with_children(|parent| {
            let font = TextFont::from_font_size(12.0);
            parent.header("Made by");
            parent.label("Joe Shmoe - Implemented aligator wrestling AI").insert(font.clone());
            parent.label("Jane Doe - Made the music for the alien invasion").insert(font.clone());

            parent.header("Assets");
            parent.label("Bevy logo - All rights reserved by the Bevy Foundation. Permission granted for splash screen use when unmodified.").insert(font.clone());
            parent.label("Button SFX - CC0 by Jaszunio15").insert(font.clone());
            parent.label("Music - CC BY 3.0 by Kevin MacLeod").insert(font.clone());
            parent.label("Ambient music and Footstep SFX - CC0 by NOX SOUND").insert(font.clone());
            parent.label("Throw SFX - FilmCow Royalty Free SFX Library License Agreement by Jason Steele").insert(font.clone());
            parent.label("Fox model - CC0 1.0 Universal by PixelMannen (model), CC BY 4.0 International by tomkranis (Rigging & Animation), CC BY 4.0 International by AsoboStudio and scurest (Conversion to glTF)").insert(font.clone());
            parent.label("Player model - \"You can use it commercially without the need to credit me\" by Drillimpact").insert(font.clone());
            parent.label("Vocals - CC BY 4.0 by Dillon Becker").insert(font.clone());
            parent.label("Rest of the assets - CC BY-NC-SA 3.0 by The Dark Mod Team. Converted to Bevy-friendly assets by Jan Hohenheim.").insert(font.clone());

            parent.button("Back").observe(enter_title_screen);
        });
}

fn enter_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct CreditsMusic {
    #[dependency]
    music: Handle<AudioSource>,
    entity: Option<Entity>,
}

impl FromWorld for CreditsMusic {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Monkeys Spinning Monkeys.ogg"),
            entity: None,
        }
    }
}

fn start_credits_music(mut commands: Commands, mut music: ResMut<CreditsMusic>) {
    music.entity = Some(
        commands
            .spawn((
                AudioPlayer(music.music.clone()),
                PlaybackSettings::LOOP,
                Music,
            ))
            .id(),
    );
}

fn stop_credits_music(mut commands: Commands, mut music: ResMut<CreditsMusic>) {
    if let Some(entity) = music.entity.take() {
        commands.entity(entity).despawn();
    }
}
