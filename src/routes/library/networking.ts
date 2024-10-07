export let serverLink = 'http://localhost:8574/' //'https://emlapi.kalsvik.no/';
export const statusMessageLink = 'https://raw.githubusercontent.com/Epic-Mickey-Launcher/status/main/emlclientstatus'
export let loggedin = false;
export let outdated = false
import {
  WriteToken,
  ReadToken
} from "./configfiles.js";
import {
  Subscribe,
  Invoke
} from "./callback.js";

let token = ""
let id = ""

export interface CommentData {
  accountid: string;
  commentid: string;
  pageid: string;
}
export interface UserInfo {
  username?: string,
  password?: string,
  email?: string,
  token?: string
}

export interface Response {
  error: boolean
  body: any
}

export enum ImageType {
  User,
  Mod
}

/**Get Image from ID.
* @param id - The ID of the Image 
* @param type - What image server to pull from
* @param ignoreCaching - Will ignore local cache and redownload image from server. 'true' by default.
*/
export function GetImagePath(id: string, type: ImageType, ignoreCaching: boolean = true): string {
  let typeString = type == ImageType.User ? "userpfp" : "modicon";
  let query = serverLink + "img/" + typeString + "?id=" + id
  if (ignoreCaching) {
    query += "&t=" + Date.now()
  }
  return query
}

export function SetOutdated() {
  outdated = true
}

export async function SignIn(userinfo: UserInfo) {
  await Login(userinfo)
}

export async function ClearInMemoryToken() {
  token = ""
}

export async function SetLoggedIn(value: boolean) {
  loggedin = value
}

export async function Register(userinfo: UserInfo) {
  let response = await POST("user/register", {
    username: userinfo.username,
    password: userinfo.password
  }, false)
  if (response.error) {
    return
  }
  userinfo.token = response.body
  Login(userinfo)
}

function isNullOrWhitespace(input: string): boolean {
  return input == input.trim();
}

export function UploadMod(modfile: File, replacing: string, extension: string, autoPublish: boolean) {

  if (!loggedin) return

  MultipartPOST("mod/publish", {
    token: token,
    modfile: modfile,
    extension: extension,
    replacing: replacing,
    automaticPublish: autoPublish
  }).then((response) => {
    JSON.parse(response.body)
      .then((res: any) => {
        Invoke("onModUpload", res.id)
      });
  })

}

export async function GetToken(): Promise<string> {
  if (token === "") {
    token = await ReadToken()
  }
  return token;
}

export async function GetId(): Promise<string> {
  await GetToken()
  if (token === "") return ""
  if (id != "") return id
  let response = await POST("user/idfromtoken", {
    token: token
  }, false)
  if (response.error) return ""
  id = response.body
  return response.body;
}

export async function OnSignedIn(callback: any) {

  if (loggedin) {
    callback()
  } else {
    Subscribe("SignedIn", callback, true)
  }
}

export async function Login(userinfo: UserInfo) {
  loggedin = false;
  let response: Response;
  if (userinfo.token === "") {
    if (isNullOrWhitespace(userinfo.username!) || isNullOrWhitespace(userinfo.password!)) {
      await alert("You forgot to enter your username or password, doofus.")
      return
    }
  }

  let tokenLogin = false
  if (userinfo.token != null) {
    response = await POST("user/login", {
      token: userinfo.token
    }, false)
    await WriteToken(userinfo.token)
    token = userinfo.token
    tokenLogin = true
    id = ""

    loggedin = true
    Invoke("SignedIn", {
      error: 0
    })

    return
  } else {
    response = await POST("user/login", {
      username: userinfo.username,
      password: userinfo.password
    }, false)
  }

  if (!response.error) {
    if (!tokenLogin) {
      await WriteToken(response.body)
      token = response.body
    }
    loggedin = true;
    Invoke("SignedIn", {
      error: 0
    })
  } else {
    loggedin = false

    Invoke("SignedIn", {
      error: 1
    })
  }

}

export async function SetServerURL(url: string) {
  serverLink = url;
}

export async function MultipartPOST(route: string, data: any): Promise<Response> {
  const formData = new FormData();
  for (const name in data) {
    formData.append(name, data[name])
  }

  const res = await fetch(serverLink + route, {
    method: 'POST',
    body: formData,
  });

  if (res.status != 200) {
    await alert(serverLink + route + "\nMultipart Request Failed: \"" + await res.text() + "\"")
  }

  const response: Response = {
    body: await res.text(),
    error: res.status != 200
  };

  return response
}

export async function POST(route: string, data: any, toJson = true, suppressError = false, external = false): Promise<Response> {
  const res = await fetch(external ? "" : serverLink + route, {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },

    body: JSON.stringify(data)
  });

  if (res.status != 200 && !suppressError) {
    await alert(serverLink + route + "\nRequest Failed: \"" + await res.text() + "\"")
  }

  let content: any = toJson ? await res.json() : await res.text();

  let response: Response = {
    error: res.status != 200,
    body: content
  }

  return response
}
export async function GET(route: string): Promise<any> {
  const res = await fetch(serverLink + route)
  const content = await res.json();
  return content;
}

export async function GETEXT(route: string): Promise<any> {
  const res = await fetch(route)
  const content = await res.json();
  return content;
}

