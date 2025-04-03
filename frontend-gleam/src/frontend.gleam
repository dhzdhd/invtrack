import gleam/uri.{type Uri}
import lustre
import lustre/effect.{type Effect}
import lustre/element.{text}
import lustre/element/html.{button, div, p}
import lustre/event.{on_click}
import modem

pub fn main() {
  let app = lustre.application(init, update, view)
  let assert Ok(_) = lustre.start(app, "#app", Nil)

  Nil
}

type Model {
  Model(current_route: Route)
}

type Route {
  Auth
  Home
}

type Msg {
  OnRouteChange(route: Route)
  Incr
  Decr
}

fn on_route_change(uri: Uri) -> Msg {
  case uri.path_segments(uri.path) {
    ["auth"] -> OnRouteChange(Auth)
    _ -> OnRouteChange(Home)
  }
}

fn init(_flags) -> #(Model, Effect(Msg)) {
  #(Model(Home), modem.init(on_route_change))
}

fn update(model, msg) {
  case msg {
    Incr -> #(model, effect.none())
    Decr -> #(model, effect.none())
    OnRouteChange(route) -> #(Model(current_route: route), effect.none())
  }
}

fn view(_model: Model) {
  div([], [
    button([on_click(Incr)], [text(" + ")]),
    p([], [text("Hello World!")]),
    button([on_click(Decr)], [text(" - ")]),
  ])
}
