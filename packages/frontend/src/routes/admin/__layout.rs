use serde::Serialize;
use tuono_lib::{Props, Response, Type};

#[derive(Serialize, Type)]
struct AdminLayoutProps {
    nav: Vec<NavItem>,
}

#[derive(Serialize, Type)]
struct NavItem {
    label: &'static str,
    href: &'static str,
}

#[tuono_lib::handler]
async fn admin_layout() -> Response {
    Response::Props(Props::new(AdminLayoutProps {
        nav: vec![
            NavItem {
                label: "Dashboard",
                href: "/admin",
            },
            NavItem {
                label: "Users",
                href: "/admin/users",
            },
            NavItem {
                label: "Bans",
                href: "/admin/bans",
            },
            NavItem {
                label: "Posts",
                href: "/admin/posts",
            },
        ],
    }))
}
