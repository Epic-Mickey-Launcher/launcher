export const serverLink = 'http://localhost:8574/';
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

export function GetModIconPath(id)
{
  return serverLink + "img/modicon?id=" + id + "&t=" + Date.now()
}

export function GetPfpPath(id)
{
  return serverLink + "img/userpfp?id=" + id + "&t=" + Date.now()
}

export function SetOutdated()
{
  outdated = true
}

export async function SignIn(userinfo) {
  await Login(userinfo)
}

export async function SetLoggedIn(l) {
  loggedin = l
}

export async function Register(userinfo) {
  let response = await POST("user/register<", {
    username: userinfo.username,
    password: userinfo.password
  })

  if (response.error) {
    return
  }

  Login({
    token: response.body
  })
}

function isNullOrWhitespace( input ) {
  console.log(input)
  return input == input.trim();
}

export function UploadMod(modfile, cb, r, e, checked) {

    if (!loggedin) return

    MultipartPOST("mod/publish", {
      token: token,
      modfile: modfile,
      extension: e,
      replacing: r,
      automaticPublish: checked
    }).then((response) => {
      JSON.parse(response.body)
      .then((res) => {
        Invoke("onModUpload", res.id)
      });
    })
  
}

export async function GetToken() {
  if(token === ""){
    token = await ReadToken()
  }

  return token;
}
export async function GetId() {
  await GetToken()
  if (token === "") return null
  if (id != "") return id
  let response = await POST("user/idfromtoken", {
    token: token
  }, false)
  if (response.error) return null
  id = response.body
  return response.body;
}

export async function OnSignedIn(cb) {

  if (loggedin) {
    cb()
  } else {
    Subscribe("SignedIn", cb, true)
  }
}

export async function Login(userinfo) {
  loggedin = false;
  let response;

  if(userinfo.token === "")
  {
    if(isNullOrWhitespace(userinfo.username) || isNullOrWhitespace(userinfo.password)) {
      await alert("You forgot to enter your username or password, doofus.")
      return
    }
  }

  let tokenLogin = false

  if (userinfo.token != null) {
    response = await POST("user/login", {
      token: userinfo.token
    }, false)

    tokenLogin = true
  } else {
    response = await POST("user/login", {
      username: userinfo.username,
      password: userinfo.password
    }, false)
  }

  if (!response.error) {
    if (!tokenLogin) await WriteToken(response.body)
    console.log(response)
    loggedin = true;
    Invoke("SignedIn", {
      error:0
    }) 
  } else {
    loggedin = false

    Invoke("SignedIn", {
      error: 1
    })
  }

}

export async function MultipartPOST(route, data) {
  
  const formData = new FormData();
  for (const name in data) {
    console.log(name)
    console.log("trolol")
    formData.append(name, data[name])
  }

  const res = await fetch(serverLink + route, {
    method: 'POST',
    body: formData,
  });
  
  if(res.status != 200)
  {
    await alert(serverLink + route + "\nMultipart Request Failed: \"" + await res.text() + "\"")
  }

  return {body: await res.text(), error: res.status != 200};
}

export async function POST(route, data, toJson = true) {
  const res = await fetch(serverLink + route, {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },

    body: JSON.stringify(data)
  });

  if(res.status != 200)
  {
    console.log("prollem")
    await alert(serverLink + route + "\nRequest Failed: \"" + await res.text() + "\"")
  }
  
  let content;
  content = toJson ? await res.json() : await res.text();
  return {body:content, error: res.status != 200};
}
export async function GET(route) {
  const res = await fetch(serverLink + route)
  const content = await res.json();
  return content;
}

export async function GETEXT(route) {
  const res = await fetch(route)
  const content = await res.json();
  return content;
}
