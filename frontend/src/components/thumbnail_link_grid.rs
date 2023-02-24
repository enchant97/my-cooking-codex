use yew::{function_component, html, AttrValue, Children, Html, Properties};
use yew_router::prelude::*;

use crate::Route;

#[derive(Properties, PartialEq)]
pub struct GridItemProps {
    pub navigate_to: Route,
    pub title: AttrValue,
    #[prop_or_default]
    pub image_src: Option<AttrValue>,
}

#[function_component(GridItem)]
pub fn grid_item(props: &GridItemProps) -> Html {
    html! {
        <Link<Route> to={props.navigate_to.clone()} classes="bg-base-100 shadow-xl aspect-square w-full relative rounded-md">
            if props.image_src.is_some() {
                <img class="object-cover w-full h-full rounded-lg hover:brightness-50 hover:duration-200" src={props.image_src.clone()} />
            } else {
                <div class="w-full h-full rounded-md hover:bg-neutral-focus hover:duration-200"></div>
            }
            <span
                class="absolute bottom-0 left-0 p-1 w-full bg-[#000000cc] rounded-b-md whitespace-nowrap overflow-hidden text-ellipsis text-lg font-bold">{props.title.clone()}</span>
        </Link<Route>>
    }
}

#[derive(Properties, PartialEq)]
pub struct GridProps {
    #[prop_or_default]
    // HACK use enum to accept iterator?
    // pub children: ChildrenWithProps<GridItem>,
    pub children: Children,
}

#[function_component(Grid)]
pub fn grid(props: &GridProps) -> Html {
    html! {
        <div class="grid gap-4 grid-cols-1 md:grid-cols-3 lg:grid-cols-5 rounded p-3 bg-base-200">
            { for props.children.iter() }
        </div>
    }
}
