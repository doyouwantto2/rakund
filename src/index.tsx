import { render } from "solid-js/web";
import { Router, Route } from "@solidjs/router";
import Home from "./pages/home/home";
import '@/assets/styles/global.css'

render(
  () => (
    <Router>
      <Route path="/" component={Home} />
    </Router>
  ),
  document.getElementById("root") as HTMLElement
);

