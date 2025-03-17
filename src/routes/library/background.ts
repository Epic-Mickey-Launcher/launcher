const background_modmarket = [
    "back1.webp",
    "back2.webp",
    "back3.webp",
    "back5.webp",
    "back6.webp",
    "back7.webp",
    "back8.webp",
    "back9.webp",
    'bgtoon.png',
    "bgNut.png",
]

const background_creds_modmarket = [
    "Background from Disney Epic Mickey",
    "Background from Disney Epic Mickey",
    "Background from Disney Epic Mickey",

    "Background from Disney Epic Mickey Rebrushed",
    "Background from Disney Epic Mickey Rebrushed",
    "Background from Disney Epic Mickey Rebrushed",
    "Background from Disney Epic Mickey Rebrushed",
    "Background from Disney Epic Mickey Rebrushed",

    "Background from @toonblot on Discord",
    "Background from @altruisticnut on Discord"
];

const background_login = [
    "back1.webp",
    "back2.webp",
    "back3.webp",
    "back5.webp",
    "back6.webp",
    "back7.webp",
    "back8.webp",
    "back9.webp",
]

const background_creds_login = [
    "Background from Disney Epic Mickey",
    "Background from Disney Epic Mickey",
    "Background from Disney Epic Mickey",
    "Background from Disney Epic Mickey Rebrushed",
    "Background from Disney Epic Mickey Rebrushed",
    "Background from Disney Epic Mickey Rebrushed",
    "Background from Disney Epic Mickey Rebrushed",
    "Background from Disney Epic Mickey Rebrushed",
];

export interface BackgroundInfo {
    path: string,
    credits: string
}

export function GetBackgroundLogin(): BackgroundInfo {
    let random = Math.floor(Math.random() * background_login.length);
    return {
        path: "img/backgrounds/" + background_login[random], credits: background_creds_login[random]
    };
}

export function GetBackgroundModMarket() {
    let random = Math.floor(Math.random() * background_modmarket.length);
    let res: BackgroundInfo = {
        path: "img/backgrounds/" + background_modmarket[random], credits: background_creds_modmarket[random]
    };
    return res;
}
