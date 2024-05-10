export function getJwtToken() {
  return sessionStorage.getItem("session");
}

export function setJwtToken(token: string) {
  sessionStorage.setItem("session", token);
}
