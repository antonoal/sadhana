use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    model::ConfirmationType,
    pages::{
        Charts, Confirmation, Input, SharedCharts,
        create_report::CreateReport,
        login::Login,
        practices::{
            Mode, edit_user_practice::EditUserPractice, edit_yatra_practice::EditYatraPractice,
            new_practice::NewPractice,
        },
        pwd_reset::PwdReset,
        register_with_id::RegisterWithId,
        settings::{
            Settings, edit_password::EditPassword, edit_user::EditUser, help::Help, import::Import,
            language::Language, support_form::SupportForm,
        },
        user_practices::UserPractices,
        yatras::{
            Yatras, admin_settings::AdminSettings, admin_settings_general::AdminSettingsGeneral,
            admin_settings_members::AdminSettingsMembers,
            admin_settings_practices::AdminSettingsPractices,
            admin_settings_stats::AdminSettingsStats, join::JoinYatra, settings::YatraSettings,
        },
    },
};

mod app_layout;
pub use app_layout::AppLayout;

#[derive(Clone, Routable, PartialEq)]
pub enum PublicRoute {
    #[at("/reset")]
    PasswordReset,
    #[at("/reset/:id")]
    PasswordResetWithConfirmationId { id: String },
    #[at("/register/:id")]
    RegisterWithConfirmationId { id: String },
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/shared/:id")]
    SharedCharts { id: String },
    #[at("/help")]
    Help,
    #[at("/")]
    Default,
    #[at("/*")]
    AppRoute,
}

/// Routes that depend on user context being loaded
#[derive(Clone, Debug, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Default,
    #[at("/settings")]
    Settings,
    #[at("/settings/edit-user")]
    EditUser,
    #[at("/settings/edit-password")]
    EditPassword,
    #[at("/help/support-form")]
    SupportForm,
    #[at("/settings/import")]
    Import,
    #[at("/settings/language")]
    Language,
    #[at("/settings/help")]
    Help,
    #[at("/settings/practices")]
    UserPractices,
    #[at("/settings/practice/new")]
    NewUserPractice,
    #[at("/settings/practice/:id/edit")]
    EditUserPractice { id: String },
    #[at("/settings/practice/new/:practice")]
    NewUserPracticeWithName { practice: String },
    #[at("/charts")]
    Charts,
    #[at("/charts/new")]
    NewReport,
    #[at("/yatras")]
    Yatras,
    #[at("/yatra/:id/join")]
    JoinYatra { id: String },
    #[at("/yatra/:id/settings")]
    YatraSettings { id: String },
    #[at("/yatra/:id/admin/settings")]
    YatraAdminSettings { id: String },
    #[at("/yatra/:id/admin/settings/general")]
    YatraAdminSettingsGeneral { id: String },
    #[at("/yatra/:id/admin/settings/practices")]
    YatraAdminSettingsPractices { id: String },
    #[at("/yatra/:id/admin/settings/members")]
    YatraAdminSettingsMembers { id: String },
    #[at("/yatra/:id/admin/settings/stats")]
    YatraAdminSettingsStats { id: String },
    #[at("/yatra/:id/practice/:practice_id/edit")]
    EditYatraPractice { id: String, practice_id: String },
    #[at("/yatra/:id/practice/new")]
    NewYatraPractice { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl AppRoute {
    pub fn is_child_of(&self, parent: &AppRoute) -> bool {
        *parent != AppRoute::Default && self.to_path().starts_with(&parent.to_path())
    }
}

fn app_switch(route: AppRoute, single_pane: bool) -> Html {
    match route {
        AppRoute::Default => html! {
            if single_pane {
                <Input with_single_pane_layout=true />
            } else {
                <Charts />
            }
        },
        AppRoute::Settings => html! { <Settings /> },
        AppRoute::EditUser => html! { <EditUser /> },
        AppRoute::EditPassword => html! { <EditPassword /> },
        AppRoute::SupportForm => html! { <SupportForm /> },
        AppRoute::Import => html! { <Import /> },
        AppRoute::Language => html! { <Language /> },
        AppRoute::Help => html! { <Help /> },
        AppRoute::UserPractices => html! { <UserPractices /> },
        AppRoute::NewUserPractice => {
            html! { <NewPractice mode={Mode::UserPractice} /> }
        }
        AppRoute::EditUserPractice { id } => {
            html! { <EditUserPractice {id} /> }
        }
        AppRoute::NewUserPracticeWithName { practice } => {
            html! { <NewPractice mode={Mode::UserPractice} practice={practice} /> }
        }
        AppRoute::Charts => html! { <Charts /> },
        AppRoute::NewReport => html! { <CreateReport /> },
        AppRoute::Yatras => html! { <Yatras /> },
        AppRoute::JoinYatra { id } => html! { <JoinYatra yatra_id={id} /> },
        AppRoute::YatraSettings { id } => html! { <YatraSettings yatra_id={id} /> },
        AppRoute::YatraAdminSettings { id } => html! { <AdminSettings yatra_id={id} /> },
        AppRoute::YatraAdminSettingsGeneral { id } => {
            html! { <AdminSettingsGeneral yatra_id={id} /> }
        }
        AppRoute::YatraAdminSettingsPractices { id } => {
            html! { <AdminSettingsPractices yatra_id={id} /> }
        }
        AppRoute::YatraAdminSettingsMembers { id } => {
            html! { <AdminSettingsMembers yatra_id={id} /> }
        }
        AppRoute::YatraAdminSettingsStats { id } => {
            html! { <AdminSettingsStats yatra_id={id} /> }
        }
        AppRoute::EditYatraPractice { id, practice_id } => {
            html! { <EditYatraPractice yatra_id={id} {practice_id} /> }
        }
        AppRoute::NewYatraPractice { id } => {
            html! { <NewPractice mode={Mode::YatraPractice { yatra_id: id }} /> }
        }
        AppRoute::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

pub fn root_switch(route: PublicRoute, single_pane: bool) -> Html {
    match route {
        PublicRoute::PasswordReset => {
            html! { <Confirmation confirmation_type={ConfirmationType::PasswordReset} /> }
        }
        PublicRoute::RegisterWithConfirmationId { id } => html! { <RegisterWithId id={id} /> },
        PublicRoute::Login => html! { <Login /> },
        PublicRoute::PasswordResetWithConfirmationId { id } => html! { <PwdReset id={id} /> },
        PublicRoute::Register => {
            html! { <Confirmation confirmation_type={ConfirmationType::Registration} /> }
        }
        PublicRoute::SharedCharts { id } => html! { <SharedCharts share_id={id} /> },
        PublicRoute::Help => html! { <Help /> },
        PublicRoute::Default | PublicRoute::AppRoute => {
            html! { <Switch<AppRoute> render={move |r| app_switch(r, single_pane)} /> }
        }
    }
}
