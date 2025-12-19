use gpui::*;
use gpui_component::{
    Collapsible, Icon, IconName,
    sidebar::{Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem},
};
use smol::block_on;

use crate::{
    app::{
        screens::{document_screen::DocumentScreen, home_screen::HomeScreen},
        states::{
            app_state::AppState, document_state::DocumentState, repository_state::RepositoryState,
        },
    },
    domain::database::document::Document,
};

pub struct AppSidebar {
    documents: Option<Vec<Document>>,
    app_state: Entity<AppState>,
}

impl AppSidebar {
    pub fn new(app_state: Entity<AppState>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let mut this = Self {
                documents: None,
                app_state,
            };

            this.fetch_documents(cx);
            this
        })
    }

    fn fetch_documents(&mut self, cx: &mut App) {
        let document_repository =
            cx.read_global::<RepositoryState, _>(|repositories, _| repositories.documents.clone());

        let documents = block_on(async move { document_repository.get_documents().await });
        if let Ok(documents) = documents {
            self.documents = Some(documents);
        }
    }
}

impl Render for AppSidebar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let links = vec![
            SidebarMenuItem::new("Chercher").icon(IconName::Search),
            SidebarMenuItem::new("Accueil")
                .icon(Icon::default().path("icons/house.svg"))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.app_state.update(cx, |app_state, cx| {
                        let home_screen = HomeScreen::new(cx.weak_entity());
                        app_state.navigator.push(home_screen, cx);
                    });
                })),
            SidebarMenuItem::new("Boite de réception").icon(IconName::Inbox),
        ];

        let documents = if let Some(documents) = &self.documents {
            documents
                .into_iter()
                .map(|document| {
                    let document_id = document.id.to_string();
                    let document_content = document.content.as_array().unwrap().clone();

                    SidebarMenuItem::new(document.title.clone())
                        .icon(IconName::File)
                        .on_click(cx.listener(move |this, _, window, cx| {
                            cx.update_global::<DocumentState, _>(|state, cx| {
                                state.add_document(
                                    document_id.clone(),
                                    document_content.clone(),
                                    window,
                                    cx,
                                );
                            });

                            this.app_state.update(cx, |app_state, cx| {
                                let document_screen = DocumentScreen::new(cx.weak_entity());
                                app_state.navigator.push(document_screen, cx);
                            });
                        }))
                        .active(true)
                })
                .collect()
        } else {
            vec![]
        };

        Sidebar::left()
            .w(Pixels::from(240.0))
            .header(SidebarHeader::new())
            .child(
                SidebarGroup::new("Label")
                    .child(SidebarMenu::new().children(links))
                    .collapsed(true),
            )
            .child(
                SidebarGroup::new("Documents")
                    .child(SidebarMenu::new().children(documents).collapsed(false)),
            )
            .footer(SidebarFooter::new().child("Footer"))
    }
}
