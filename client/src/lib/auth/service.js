import createAuth0Client from "@auth0/auth0-spa-js";
import { user, isAuthenticated, popupOpen } from "../stores";
import config from "./config";

let _client = null;


async function createClient() {
  if (_client !== null) { return _client }

  _client = await createAuth0Client({
    domain: config.domain,
    client_id: config.clientId
  });

  isAuthenticated.set(await _client.isAuthenticated());
  user.set(await _client.getUser());
  console.log("Auth: " + config);

  return _client;
}

async function getToken() {
  let client = await createClient()
  return await client.getTokenSilently();
}

async function login(options) {
  let client = await createClient()
  popupOpen.set(true);

  try {
    await client.loginWithPopup(options);

    user.set(await client.getUser());
    console.log("Logged in as " + await client.getUser())
    isAuthenticated.set(true);
  } catch (e) {
    // eslint-disable-next-line
    console.error(e);
  } finally {
    popupOpen.set(false);
  }
}

async function logout() {
  let client = await createClient()
  return client.logout();
}

const auth = {
  createClient,
  login,
  logout,
  getToken
};

export default auth;
