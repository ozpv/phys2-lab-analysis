use leptos::{html, leptos_dom::helpers, prelude::*};
use leptos_icons::Icon;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, MediaStreamConstraints, MouseEvent};

#[component]
pub fn CameraModal() -> impl IntoView {
    let video = NodeRef::<html::Video>::new();
    let canvas = NodeRef::<html::Canvas>::new();
    let image = NodeRef::<html::Img>::new();

    Effect::new(move || {
        let constraints = MediaStreamConstraints::new();
        constraints.set_video(&JsValue::TRUE);
        constraints.set_audio(&JsValue::FALSE);

        let stream_callback = Closure::new(move |stream: JsValue| {
            let stream = stream.into();
            video.get_untracked().unwrap().set_src_object(Some(&stream));
        });

        let _ = helpers::window()
            .navigator()
            .media_devices()
            .unwrap()
            .get_user_media_with_constraints(&constraints)
            .unwrap()
            .then(&stream_callback);

        // don't drop our closure
        stream_callback.forget();
    });

    let capture = move |ev: MouseEvent| {
        ev.prevent_default();

        let video = video.get().unwrap();

        let (w, h) = (
            f64::from(video.video_width()),
            f64::from(video.video_height()),
        );

        let () = canvas
            .get()
            .unwrap()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .unchecked_into::<CanvasRenderingContext2d>()
            .draw_image_with_html_video_element_and_dw_and_dh(&video, 0.0, 0.0, w, h)
            .unwrap();

        Dom::set_attribute(
            &image.get().unwrap(),
            "src",
            &canvas
                .get()
                .unwrap()
                .to_data_url_with_type("image/png")
                .unwrap(),
        );
    };

    view! {
        <div class="bg-base p-6 rounded-lg shadow-lg">
            <div class="flex items-center justify-center relative">
                <video class="bg-base rounded-lg" autoplay node_ref=video />
                <button class="bg-base text-text p-3 rounded-full shadow transition duration-200 hover:bg-mantle"
                    on:click=capture
                >
                    <Icon icon={icondata::LuCamera} width="36" height="36"/>
                </button>
            </div>
        </div>
        <canvas class="hidden" width="768" height="768" node_ref=canvas />
        <img alt="captured photo will appear here" node_ref=image />
    }
}
