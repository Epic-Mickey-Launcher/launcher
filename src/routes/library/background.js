const background = [
    "back1.webp",
    "back2.webp",
    "back3.webp",
]

export function GetBackground()
{
   let random = Math.floor(Math.random() * background.length);
   return "img/backgrounds/" + background[random];
}