const serverLink = 'http://localhost:3002/eml/';
let token = ""
// @ts-ignore
window.executeCallbacks = function (callbacks, data){
    callbacks.forEach(cb => {
         cb(data)
    })
}

// @ts-ignore
window.onSignIn = []

export async function Register(userinfo){
  let info = await POST("register", {username: userinfo.username, password:userinfo.password})

  if(info.error == 1){
    //account with same username already exists
    return
  }

  Login({token: info.token})
}

export async function UploadMod(modfile){
  let moduploadresult = await POST("modupload", {token: token, modfile:modfile})
}

export async function Login(userinfo){

   let finalinfo;

   if(userinfo.token != null){
     finalinfo = await POST("signintoken", {token: userinfo.token})
   }
   else {
    console.log("fartoluci")
    finalinfo = await POST("signin", {
      username: userinfo.username,
       password:userinfo.password
      })
   }

// @ts-ignore
   window.executeCallbacks(window.onSignIn, finalinfo)


   if(finalinfo.error == 0){
        token = finalinfo.token;
   }
   else{
   
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
