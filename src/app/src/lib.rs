use leptos::prelude::*;

pub fn shell(options: LeptosOptions, google_api_key: String) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
            </head>
            <body>
                <App />
                <script
                    async
                    src=format!(
                        "https://maps.googleapis.com/maps/api/js?key={}&libraries=maps&v=weekly&loading=async",
                        google_api_key,
                    )
                ></script>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div style="padding: 20px;">
            <h1>"RentMap"</h1>
            <gmp-map
                center="25.0330,121.5654"
                zoom="13"
                style="height: 400px; width: 100%; border: 1px solid #ccc;"
            ></gmp-map>
        </div>
    }
}
