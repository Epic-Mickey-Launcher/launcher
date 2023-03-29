export const serverLink = 'http://localhost:3002/eml/';
export const staticAssetsLink = 'http://localhost:3002/';
export let loggedin = false;
import { WriteToken, ReadToken } from "./configfiles.js";
import { Subscribe, Invoke } from "./callback.js";

let accountinfo;

export async function SignIn(userinfo){
 await Login(userinfo)
}

export async function Register(userinfo){
  let info = await POST("register", {username: userinfo.username, password:userinfo.password})

  if(info.error == 1){
    //account with same username already exists
    return
  }

  Login({token: info.token})
}

export async function UploadMod(modfile){
  let info = await GetUserInfo()
  let moduploadresult = await POST("modupload", {token: info.token, modfile:modfile})
}

export async function Login(userinfo){
  loggedin = false;
   let finalinfo;

   if(userinfo.token != null){
     finalinfo = await POST("signintoken", {token: userinfo.token})
   }
   else {
    finalinfo = await POST("signin", {
      username: userinfo.username,
       password:userinfo.password
      })
   }
   accountinfo = finalinfo
   console.log(finalinfo)
 await WriteToken(finalinfo.token)
 loggedin = true;
 Invoke("SignedIn", finalinfo)
}

export async function GetUserInfo(){
  if(accountinfo == null){
    console.log("User is not logged in!")
    return {error:0}
  }
  else{
    return accountinfo
  }
}

export async function POST(route, data)
{
     const res = await fetch(serverLink + route, {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(data)
  });
  const content = await res.json();

  return content;
}
export async function GET(route)
{
  const res = await fetch(serverLink + route)
  const content = await res.json();
  return content;
}


