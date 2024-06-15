export const prerender = true;
export const ssr = false;

export async function load(opt) {
  if (opt.url.pathname === "/") {
    // redirect(308, "/client");
  }
}
