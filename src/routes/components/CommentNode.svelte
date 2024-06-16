<svelte:options accessors />

<script lang="ts">
  import { POST, GetId, CommentData } from "../library/networking";
  import User from "./User.svelte";
  let date: string;
  let comment: string;
  let isCommentAuthor: boolean;
  let data: CommentData;

  export let onDelete: any;

  export function InitCommentNode(_data: CommentData) {
    data = _data;
    let timestamp = parseInt(data.commentid);
    let d = new Date(timestamp);
    date = d.toLocaleString();
    CompareLocalID();
  }

  async function CompareLocalID() {
    let localID = await GetId();

    if (localID === data.accountid) {
      isCommentAuthor = true;
    }

    isCommentAuthor = true;
  }

  async function Delete() {
    await POST("deletecomment", {
      CommentID: data.commentid,
      PageID: data.pageid,
    });
    onDelete();
  }
</script>

<div
  style="width:50%;height:100px;background-color:#2e2e2e;border-radius:10px;display:flex;padding:3px;filter: drop-shadow(1px 1px 3px black);margin:auto;"
>
  <span
    style="margin-top:auto;margin-bottom:auto;text-align:center;margin-left:10px;"
  >
    <br />
  </span>
  <div style="align-items:left;text-align:left;">
    <span style="margin-left:0px;">
      <User></User>
      {#if isCommentAuthor}
        <button
          on:click={Delete}
          class="hyperlinkbutton"
          style="font-size:10px;color:red;">Delete</button
        >
        <span style="font-size:10px;"> | </span>
      {/if}
      <span style="font-size:10px;">Sent on: {date}</span>
      <br />
      <div>
        <span style="text-align:left;word-wrap: break-word;width:50%;">
          {comment}
        </span>
      </div>
    </span>
  </div>

  <br />
</div>
<p></p>
