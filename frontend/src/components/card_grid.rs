use yew::{function_component, html, AttrValue, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct GridItemProps {
    #[prop_or_default]
    pub image_src: Option<AttrValue>,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(GridItem)]
pub fn grid_item(props: &GridItemProps) -> Html {
    html! {
        <div class="card card-compact bg-base-100 shadow-xl">
            if !props.image_src.is_none() {
                <figure class="h-3/5"><img class="object-cover w-full h-full" src={props.image_src.clone()} /></figure>
            } else {
                <div class="h-3/5"></div>
            }
            <div class="card-body">
                if !props.title.is_none() {
                    <h2 class="card-title">{props.title.clone()}</h2>
                }
                { for props.children.iter() }
            </div>
        </div>
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
        <div class="grid gap-4 grid-cols-1 md:grid-cols-2 lg:grid-cols-3 rounded p-3 bg-base-200">
            { for props.children.iter() }
        </div>
    }
}
