import { POST } from "./networking";
import { ReadToken, WriteToken } from "./configfiles";

export let loggedInAccount: Account = null;

export interface Account {
  username: string;
  id: string;
  token: string;
}

function isNullOrWhitespace(input: string): boolean {
  return input.trim() == "";
}

export async function Logout() {
  if (loggedInAccount == null) {
    console.log("attempting log out but not logged into an account...");
    return;
  }

  WriteToken("");
  loggedInAccount = null;
}

export async function LoginWithPassword(
  username: string,
  password: string,
): Promise<boolean> {
  if (isNullOrWhitespace(username) || isNullOrWhitespace(password)) {
    await alert("You forgot to enter your username or password, doofus.");
    return false;
  }

  let response = await POST(
    "user/login",
    {
      username: username,
      password: password,
    },
    false,
  );

  await WriteToken(response.body);

  if (!(await Login(response.body))) {
    console.log("Failed to retrieve data about logged-in account.");
    return false;
  }
  return true;
}

async function Login(token: string): Promise<boolean> {
  let response = await POST(
    "user/idfromtoken",
    {
      token: token,
    },
    false,
    true,
  );
  if (response.error) {
    console.log("failed to log in with token!");
    await alert("failed to log in with provided token.");
    return false;
  }
  let id = response.body;
  response = await POST("user/username", { id: id }, false, true);
  if (response.error) return false;
  let username = response.body;

  loggedInAccount = {
    username: username,
    id: id,
    token: token,
  };
  await WriteToken(token);

  return true;
}

export async function LoginWithSession(): Promise<boolean> {
  let token = await ReadToken();

  if (token == "" || token == null) {
    console.log("client does not have a login token");
    return false;
  }

  if (!(await Login(token))) {
    console.log("Failed to retrieve data about logged-in account.");
    await WriteToken("");
    return false;
  }

  return true;
}

export async function Register(
  username: string,
  password: string,
  email: string,
): Promise<boolean> {
  let response = await POST(
    "user/register",
    {
      username: username,
      password: password,
      email: email,
    },
    false,
  );
  if (response.error) {
    return;
  }
  Login(response.body);
}
