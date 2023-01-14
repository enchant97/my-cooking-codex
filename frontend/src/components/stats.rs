use yew::{function_component, html, ChildrenWithProps, Html, Properties, AttrValue};

#[derive(Properties, PartialEq)]
pub struct StatProps {
    pub title: AttrValue,
    pub value: AttrValue,
    #[prop_or_default]
    pub description: Option<AttrValue>,
}

#[function_component(Stat)]
pub fn stat(props: &StatProps) -> Html {
    html! {
        <div class="stat place-items-center">
            <div class="stat-title">{props.title.clone()}</div>
            <div class="stat-value">{props.value.clone()}</div>
            if props.description.is_some() {
                <div class="stat-desc">{props.description.clone().unwrap()}</div>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct StatsProps {
    pub children: ChildrenWithProps<Stat>,
}

#[function_component(Stats)]
pub fn stats(props: &StatsProps) -> Html {
    html! {
        <div class="stats shadow">
            { for props.children.iter() }
        </div>
    }
}
