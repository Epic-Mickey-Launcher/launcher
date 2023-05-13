export const serverLink = 'https://api.memerdev.com/';
export const staticAssetsLink = 'https://api.memerdev.com/';
export let loggedin = false;
import { WriteToken, ReadToken } from "./configfiles.js";
import { Subscribe, Invoke } from "./callback.js";

let accountinfo;

export async function SignIn(userinfo){
 await Login(userinfo)
}

export async function SetLoggedIn(l)
{
  loggedin = l
}

export async function Register(userinfo){
  let info = await POST("register", {username: userinfo.username, password:userinfo.password})

  if(info.error == 1){
    //account with same username already exists
    return
  }

  Login({token: info.token})
}

export async function UploadMod(modfile, cb, r){
  let info = await GetUserInfo()
  let moduploadresult = await MultipartPOST("modupload", {token: info.token, modfile:modfile, replacing:r})
  cb()
}

export async function GetToken()
{
  return accountinfo.token;
}
export async function GetId(){
  return accountinfo.id;
}

export async function OnSignedIn(cb){

        if(loggedin)
        {
          cb(await GetUserInfo())
        }
        else{
            Subscribe("SignedIn", cb, true)
        }
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

   if(finalinfo != null)
   {
    accountinfo = finalinfo
    await WriteToken(finalinfo.token)
    loggedin = true;
    Invoke("SignedIn", finalinfo)
   }
   else{
    loggedin = false
    Invoke("SignedIn", {error:1})
   }

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

export async function MultipartPOST(route, data)
{
  const formData = new FormData();
  for (const name in data)
  {
    formData.append(name, data[name])
  }
  const res = await fetch(serverLink + route, {
    method: 'POST',
  
    body: formData
  });
  return await res;
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


