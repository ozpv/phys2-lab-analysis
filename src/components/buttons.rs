use leptos::prelude::*;
use leptos_icons::Icon;

pub enum ButtonSize {
    Small,
    Medium,
    Large,
    /// custom button size (x, y) in tailwind's units
    Custom(&'static str, &'static str),
}

impl ButtonSize {
    fn class(self) -> String {
        match self {
            Self::Small => "px-3 py-1".to_string(),
            Self::Medium => "px-5 py-2".to_string(),
            Self::Large => "px-7 py-4".to_string(),
            Self::Custom(x, y) => format!("px-{x} py-{y}"),
        }
    }
}

pub enum ButtonVariant {
    Primary,
    Secondary,
    Outlined,
}

impl ButtonVariant {
    fn class(self) -> String {
        match self {
            Self::Primary => "bg-sky hover:bg-blue text-base".to_string(),
            Self::Secondary => "bg-mantle hover:bg-crust text-text".to_string(),
            Self::Outlined => "border-mantle bg-base hover:bg-mantle text-text".to_string(),
        }
    }
}

#[derive(Clone)]
pub enum ButtonIcon {
    Left(icondata::Icon),
    Right(icondata::Icon),
    None,
}

impl ButtonIcon {
    // TODO: add icon view
    fn view_left(self) -> impl IntoView {
        if let Self::Left(icon) = self {
            view! { <Icon icon /> }.into_any()
        } else {
            ().into_any()
        }
    }

    fn view_right(self) -> impl IntoView {
        if let Self::Right(icon) = self {
            view! { <Icon icon /> }.into_any()
        } else {
            ().into_any()
        }
    }
}

pub trait IntoButtonIcon {
    fn into_left(self) -> ButtonIcon;
    fn into_right(self) -> ButtonIcon;
}

impl IntoButtonIcon for icondata::Icon {
    fn into_left(self) -> ButtonIcon {
        ButtonIcon::Left(self)
    }

    fn into_right(self) -> ButtonIcon {
        ButtonIcon::Left(self)
    }
}

#[component]
pub fn Button(
    #[prop(default = ButtonIcon::None)] icon: ButtonIcon,
    #[prop(default = ButtonSize::Small)] size: ButtonSize,
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class=format!("inline-flex justify-center items-center {} text-center rounded shadow {}", variant.class(), size.class())
        >
            {icon.clone().view_left()}
            {children()}
            {icon.view_right()}
        </button>
    }
}
