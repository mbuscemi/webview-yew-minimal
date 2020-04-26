#[macro_use]
extern crate stdweb;
extern crate serde;
extern crate serde_json;
extern crate serde_derive;

use serde::Deserialize;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use stdweb::{ serde::Serde, unstable::TryInto, Value };

pub enum Msg {
    SendToWebview,
    ReceiveFromWebview(String),
}

pub struct Model {
    link: ComponentLink<Self>,
    times_called_to_webview: usize,
    communiques: Vec<String>,
    js_value: Value,
}

#[derive(Deserialize)]
pub struct Detail {
    pub communique: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let yew_callback = link.callback(|detail: Detail| Msg::ReceiveFromWebview(detail.communique) );

        let js_callback = move |value: Value| {
            let structure: Serde<Detail> = value.try_into().expect("unable to parse payload from event");
            let detail: Detail = structure.0;
            yew_callback.emit(detail)
        };

        let js_value =
            js! {
                var callback = @{js_callback};
                var listener = event => callback(event.detail);
                document.addEventListener("send_back", listener);
                return {
                    name: "send_back",
                    callback: callback,
                    listener: listener
                };
            };

        Model {
            link: link,
            times_called_to_webview: 0,
            communiques: Vec::new(),
            js_value: js_value,
        }
    }

    fn destroy(&mut self) {
        js! {
            var value = @{&self.js_value};
            document.removeEventListener(value.name, value.listener);
            value.callback.drop();
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendToWebview => {
                self.times_called_to_webview = self.times_called_to_webview + 1;
                js! { external.invoke("receive"); }
            },
            Msg::ReceiveFromWebview(communique) => {
                self.communiques.push(communique);
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <button onclick=self.link.callback(|_| Msg::SendToWebview)>{"To Webview and Back"}</button>
                <h1>{"Times Called to Webview"}</h1>
                <div>{self.times_called_to_webview}</div>
                <h1>{"Messages from Webview"}</h1>
                <ol>
                    { for self.communiques.iter().map(|c| html! { <li>{c}</li> }) }
                </ol>
            </>
        }
    }
}