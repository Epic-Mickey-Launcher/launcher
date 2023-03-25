const serverLink = 'http://localhost:3002/eml/';

// @ts-ignore
window.executeCallbacks = function (callbacks, data){
    callbacks.forEach(cb => {
         cb(data)
    })
}

// @ts-ignore
window.onSignIn = []

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
