import { c as create_ssr_component, a as each, b as add_attribute, e as escape, v as validate_component } from "../../chunks/index-7340f048.js";
const Header = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  const routes = ["About", "Home"];
  return `<nav class="${"shadow"}"><ol class="${"flex justify-start items-center"}"><h1 class="${"font-bold px-5"}">Ferret</h1>
    ${each(routes, (route) => {
    return `<li class="${"py-2 px-3 hover:bg-emerald-400"}"><a${add_attribute("href", "/" + route.toLowerCase(), 0)}>${escape(route)}</a>
      </li>`;
  })}</ol></nav>`;
});
const Routes = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  return `${validate_component(Header, "Header").$$render($$result, {}, {}, {})}`;
});
export { Routes as default };
