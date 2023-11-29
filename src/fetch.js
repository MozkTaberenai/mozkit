export function fetch_get(url) {
  return fetch(url);
}
export function fetch_post(url, body) {
  return fetch(url, { method: "POST", body: body });
}
