use crate::example::*;
use patternfly_yew::*;
use yew::html::ChildrenRenderer;
use yew::prelude::*;

pub struct LoginPageExample {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for LoginPageExample {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let header = ChildrenRenderer::new(vec![html! {<> {"Header" }</>}]);
        let footer = ChildrenRenderer::new(vec![html! {<p>{"Some footer text"}</p>}]);
        html! {
            <>
                <Background/>
                <Login
                    header=header
                    footer=footer
                    >
                    <LoginMain>
                        <LoginMainHeader
                            title=html_nested!{<Title>{"Login to your account"}</Title>}
                            description="Enter the credentials to your account right here."
                            />
                        <LoginMainBody>
                        </LoginMainBody>
                    </LoginMain>
                </Login>
            </>
        }
    }
}

impl LoginPageExample {}