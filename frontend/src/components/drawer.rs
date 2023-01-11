use yew::prelude::*;
use yew_router::prelude::Link;

use crate::Route;

use super::Header;

#[derive(Properties, PartialEq)]
pub struct ContentProps {
    pub header: bool,
    pub children: Children,
}

#[function_component(DrawerContent)]
pub fn drawer_content(props: &ContentProps) -> Html {
    html! {
        <div class="drawer-content">
            if props.header {
                <Header/>
            }
            <div class="pt-3 px-3">
                { for props.children.iter() }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerLinkProps {
    pub to: Route,
    #[prop_or_default]
    pub active: bool,
    pub children: Children,
}

#[function_component(DrawerLink)]
pub fn drawer_link(props: &DrawerLinkProps) -> Html {
    html! {
        if props.active {
            <Link<Route> to={props.to.clone()} classes="btn btn-primary">
                { for props.children.iter() }
            </Link<Route>>
        } else {
            <Link<Route> to={props.to.clone()} classes="btn bg-base-100">
                { for props.children.iter() }
            </Link<Route>>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawProps {
    pub r#for: &'static str,
    pub children: Children,
}

#[function_component(DrawerDraw)]
pub fn drawer_draw(props: &DrawProps) -> Html {
    html! {
        <div class="drawer-side">
            <label for={props.r#for} class="drawer-overlay"></label>
            <ul class="menu gap-2 p-4 w-80 bg-base-200">
                { for props.children.iter().into_iter().map(|child| {
                    html!{<li>{child}</li>}
                }) }
            </ul>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerProps {
    pub r#for: &'static str,
    pub children: Children,
}

#[function_component(Drawer)]
pub fn drawer_content(props: &DrawerProps) -> Html {
    html! {
        <div class="drawer drawer-mobile">
            <input id={props.r#for} type="checkbox" class="drawer-toggle" />
            { for props.children.iter() }
        </div>
    }
}
