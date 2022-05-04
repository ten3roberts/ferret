import createAuth0Client from "@auth0/auth0-spa-js";
import { user, isAuthenticated, popupOpen } from "../stores";
import config from "./config";

let _client = null;
let _user = null;
let _authenticated = null;


async function createClient() {
  if (_client !== null) {
    return { client: _client, authenticated: _authenticated, user: _user }
  }

  console.log(config);
  _client = await createAuth0Client({
    domain: config.domain,
    client_id: config.clientId,
    audience: config.audience,
    useRefreshTokens: true,
  });

  _authenticated = await _client.isAuthenticated()
  isAuthenticated.set(_authenticated);

  _user = await _client.getUser()

  user.set(_user);
  console.log("Auth: " + config);

  return { client: _client, authenticated: _authenticated, user: _user };
}

async function getToken() {
  let { client } = await createClient()
  const token = await client.getTokenSilently();
  return token
}

async function login(options) {
  let { client } = await createClient()

  console.log(`Client: ${client}`)
  popupOpen.set(true);

  try {
    await client.loginWithPopup(options);

    user.set(await client.getUser());
    isAuthenticated.set(true);
  } catch (e) {
    // eslint-disable-next-line
    console.error(e);
  } finally {
    popupOpen.set(false);
  }
}

async function logout() {
  let { client } = await createClient()
  console.log(`Client: ${client}`)
  return client.logout();
}

const auth = {
  createClient,
  login,
  logout,
  getToken
};

export default auth;
