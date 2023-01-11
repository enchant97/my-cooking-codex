use yew::{function_component, html, ChildrenWithProps, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct StatProps {
    pub title: String,
    pub value: String,
    #[prop_or_default]
    pub description: Option<String>,
}

#[function_component(Stat)]
pub fn stat(props: &StatProps) -> Html {
    html! {
        <div class="stat place-items-center">
            <div class="stat-title">{props.title.clone()}</div>
            <div class="stat-value">{props.value.clone()}</div>
            if !props.description.is_none() {
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
