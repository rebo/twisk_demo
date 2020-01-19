use crate::{generated::css_classes::C, twisk::*, Model, Msg, Player};
use comp_state::{
    list::{use_list, List, ListKey},
    topo, use_state, StateAccess,
};
use enclose::enclose as e;
use seed::{prelude::*, *};
use seed_comp_helpers::on_click;
use slotmap::DefaultKey;

#[topo::nested]
pub fn view(model: &Model) -> Node<Msg> {
    div![
        menus::pink_menu(
            "Football Manager",
            &[("Home", "/home"), ("About", "/about"),]
        ),
        h1!["Team one:"],
        team_arranger(model),
        h1!["Team Two:"],
        team_arranger(model),
        h1!["Team Three:"],
        team_arranger(model),
    ]
}

#[topo::nested]
fn team_arranger(model: &Model) -> Node<Msg> {
    let (players, list_control) =
        use_list(|| model.players.values().cloned().collect::<Vec<Player>>());
    div![div![
        class![C.ml_2 C.mr_2 C.flex C.flex_row],
        div![
            class![C.ml_4 C.w_1of2],
            player_selection_pane(&players, list_control)
        ],
        div![class![C.mr_4 C.w_1of2], field_pane(&players)],
    ]]
}

#[topo::nested]
fn player_selection_pane(
    players: &List<Player>,
    list_control: comp_state::list::ListControl<Player>,
) -> Node<Msg> {
    // clone the players into a list managed by this component

    div![
        class![C.flex C.flex_row],
        div![
            class![C.w_full C.flex C.flex_col],
            div!["Football Players Available:"],
            players
                .items_map.0.iter().map(|(key, player)| button![
                    player.name,
                    on_click(
                        e!((list_control) move |_| list_control.select_only_by_key(key))
                    ),
                    if players.selected_keys.contains(&key){
                            class![C.focus__outline_none C.rounded C.text_white C.bg_pink_600]
                    } else {
                        class![C.outline_none]
                    }
                ])
                .collect::<Vec<Node<Msg>>>(),
            div![tabs::tabs_3(player_bio(players.selected().next()), player_stats(players.selected().next()), player_photo(players.selected().next()))],
        ],
    ]
}

fn player_bio(player: Option<&Player>) -> (String, Node<Msg>) {
    let content = if let Some(player) = player {
        div![
            h1![player.name],
            dl![
                dt!["Age:"],
                dd![player.age],
                dt!["D.O.B.:"],
                dd![player.dob],
                dt!["Nationality:"],
                dd![player.nationality],
            ]
        ]
    } else {
        div!["No Player Selected"]
    };
    ("Biography".to_string(), content)
}
fn player_stats(player: Option<&Player>) -> (String, Node<Msg>) {
    let content = if let Some(player) = player {
        div![
            h1![player.name],
            dl![
                dt!["Shooting:"],
                dd![format!("{}", player.shooting)],
                dt!["Tackling:"],
                dd![format!("{}", player.tackling)],
                dt!["Passing:"],
                dd![format!("{}", player.passing)],
                dt!["Speed:"],
                dd![format!("{}", player.speed)],
            ]
        ]
    } else {
        div!["No Player Selected"]
    };
    ("Stats".to_string(), content)
}

fn player_photo(player: Option<&Player>) -> (String, Node<Msg>) {
    let content = if let Some(player) = player {
        div![
            h1![player.name],
            images::simple_raised_image(
                player.image_url.clone(),
                "Player photo".to_string()
            )
        ]
    } else {
        div!["No Player Selected"]
    };
    ("Photo".to_string(), content)
}
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Team {
    positions: HashMap<String, DefaultKey>,
}

impl Default for Team {
    fn default() -> Self {
        Self {
            positions: HashMap::new(),
        }
    }
}

#[topo::nested]
fn field_pane(players: &List<Player>) -> Node<Msg> {
    let (team, team_access) = use_state(Team::default);
    let (show_alert, show_alert_access) = use_state(|| false);

    div![
        if show_alert {
            alerts::pink_alert("Team Saved!", show_alert_access.clone())
        } else {
            empty![]
        },
        h1!["Playing Positions:"],
        div![
            class![C.flex C.justify_end],
            dropdowns::small_action_dropdown(
                "Actions...",
                &["Save Team", "Reset Team"],
                e!((team, show_alert_access)move |option| {
                    match option {
                        "Save Team" => {log!(team);show_alert_access.set(true)},
                        "Reset Team" => log!("Resetting Team"),
                        _ => log!("oh dear"),
                    }
                })
            )
        ],
        class![C.flex C.flex_col],
        div![
            class![C.w_full C.flex C.flex_row C.justify_center C.my_4 C.h_20],
            player_position("Goalkeeper", players, team_access.clone())
        ], // goalkeepiner
        div![
            class![C.w_full C.flex C.flex_row C.justify_center C.my_4 C.h_20],
            player_position("Right Back", players, team_access.clone()),
            player_position("Centre Back 1", players, team_access.clone()),
            player_position("Centre Back 2", players, team_access.clone()),
            player_position("Left Back", players, team_access.clone()),
        ], // backrow
        div![
            class![C.w_full C.flex C.flex_row C.justify_center C.my_4 C.h_20],
            player_position("Right MF", players, team_access.clone()),
            player_position("Centre MF 1", players, team_access.clone()),
            player_position("Centre MF 2", players, team_access.clone()),
            player_position("Left MF", players, team_access.clone()),
        ], // backrow
        div![
            class![C.w_full C.flex C.flex_row C.justify_center C.my_4 C.h_20],
            player_position("Forward 1", players, team_access.clone()),
            player_position("Forward 2", players, team_access),
        ]
    ] // attack
}

#[topo::nested]
fn player_position<T: Into<String>>(
    position: T,
    players: &List<Player>,
    team_access: StateAccess<Team>,
) -> Node<Msg> {
    // let (position, position_access) = use_state(|| position);
    let position = position.into();
    let (filled_with_player_key, fip_access) =
        use_state::<Option<ListKey>, _>(|| None);

    div![
        if let Some(Some(player)) =
            filled_with_player_key.map(|key| players.items_map.0.get(key))
        {
            div![
                class![C.flex C.flex_col],
                images::logo_raised_image(
                    player.image_url.clone(),
                    player.name.clone(),
                ),
                span![class![C.text_xs], position]
            ]
        } else {
            div![
                class![C.flex C.flex_col],
                images::logo_raised_image(
                    "static/images/cross.svg",
                    "player not positioned",
                ),
                span![class![C.text_xs], position]
            ]
        },
        on_click(e!((players) move |_| {
            if let Some(key) = players.selected_keys.iter().cloned().next(){
                if let Some(player) =  players.items_map.0.get(key){
                team_access.update(|team| {team.positions.insert(position.clone(), player.key );});
                fip_access.set(Some(key))
                }
            }
        })),
    ]
}
