const background = [
    "back1.webp",
    "back2.webp",
    "back3.webp",
]

const background_creds = [
    "Background from Disney Epic Mickey",
    "Background from Disney Epic Mickey",
    "Background from Disney Epic Mickey"
];

export function GetBackground()
{
   let random = Math.floor(Math.random() * background.length);
   return {path: "img/backgrounds/" + background[random], credits: background_creds[random]};
}