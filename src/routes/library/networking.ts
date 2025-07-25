export let serverLink = "http://127.0.0.1:8574/"; // "https://emlapi.kalsvik.no/";
export const statusMessageLink =
  "https://raw.githubusercontent.com/Epic-Mickey-Launcher/status/main/emlclientstatus";
export let outdated = false;
export let offlineMode = false;
import { loggedInAccount } from "./account.js";

export interface UserInfo {
  username?: string;
  password?: string;
  email?: string;
  token?: string;
}

export interface Response {
  error: boolean;
  body: any;
}

export enum ImageType {
  User,
  Mod,
}

/**Get Image from ID.
 * @param id - The ID of the Image
 * @param type - What image server to pull from
 * @param ignoreCaching - Will ignore local cache and redownload image from server. 'true' by default.
 */
export function GetImagePath(
  id: string,
  type: ImageType,
  ignoreCaching: boolean = true,
): string {
  let typeString = type == ImageType.User ? "userpfp" : "modicon";
  let query = serverLink + "img/" + typeString + "?id=" + id;
  if (ignoreCaching) {
    query += "&t=" + Date.now();
  }
  return query;
}

export function SetOutdated() {
  outdated = true;
}

export function SetOfflineMode() {
  offlineMode = true;
}

// DEPRECATED: use loggedInAccount.id instead.
export function GetId(): string {
  return loggedInAccount.id;
}

export async function SetServerURL(url: string) {
  serverLink = url;
}

export async function MultipartPOST(
  route: string,
  data: any,
): Promise<Response> {
  const formData = new FormData();
  for (const name in data) {
    formData.append(name, data[name]);
  }

  const res = await fetch(serverLink + route, {
    method: "POST",
    body: formData,
  });

  if (res.status != 200) {
    await alert(
      serverLink +
        route +
        '\nMultipart Request Failed: "' +
        (await res.text()) +
        '"',
    );
  }

  return {
    body: await res.text(),
    error: res.status != 200,
  };
}

export async function POST(
  route: string,
  data: any,
  toJson = true,
  suppressError = false,
  external = false,
  headers: Headers = new Headers(),
): Promise<Response> {
  headers.append("Accept", "application/json");
  headers.append("Content-Type", "application/json");

  const res = await fetch(external ? "" : serverLink + route, {
    method: "POST",
    headers: headers,
    body: JSON.stringify(data, null, 4),
  });

  let content: any = null;
  if (res.status != 200 && !suppressError) {
    await alert(
      serverLink + route + '\nRequest Failed: "' + (await res.text()) + '"',
    );
  }

  if (res.status == 200) {
    content = toJson ? await res.json() : await res.text();
  }

  return {
    error: res.status != 200,
    body: content,
  };
}

export async function GET(route: string): Promise<any> {
  const res = await fetch(serverLink + route);
  return await res.json();
}

export async function GETExternal(route: string): Promise<any> {
  const res = await fetch(route);
  return await res.json();
}
