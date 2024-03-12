const background_modmarket = [
    "back1.webp",
    "back2.webp",
    "back3.webp",
    'bgtoon.png',
    "bgNut.png",
]

const background_creds_modmarket = [
    "Background from Disney Epic Mickey",
    "Background from Disney Epic Mickey",
    "Background from Disney Epic Mickey",
    "Background from @toonblot on Discord",
    "Background from @altruisticnut on Discord"
];

const background_login = [
    "back1.webp",
    "back2.webp",
    "back3.webp",
    "bgNut.png",
]

const background_creds_login = [
    "Background from Disney Epic Mickey",
    "Background from Disney Epic Mickey",
    "Background from Disney Epic Mickey",
    "Background from @altruisticnut on Discord"
];

export function GetBackgroundLogin()
{
   let random = Math.floor(Math.random() * background_login.length);
   return {path: "img/backgrounds/" + background_login[random], credits: background_creds_login[random]};
}
export function GetBackgroundModMarket()
{
   let random = Math.floor(Math.random() * background_modmarket.length);
   return {path: "img/backgrounds/" + background_modmarket[random], credits: background_creds_modmarket[random]};
}