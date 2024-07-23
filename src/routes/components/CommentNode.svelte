<svelte:options accessors />

<script lang="ts">
  import { onMount } from "svelte";
  import { POST, GetId, CommentData, GetToken } from "../library/networking";
  import User from "./User.svelte";
  let date: string;
  let isCommentAuthor: boolean;
  export let content: string;
  export let commentID: string;
  export let id: string;
  export let onDelete: any;
  let deleted = false;

  export function InitCommentNode() {
    let timestamp = parseInt(commentID);
    let d = new Date(timestamp);
    date = d.toLocaleString();
    CompareLocalID();
  }

  async function CompareLocalID() {
    let localID = await GetId();

    if (localID == id) {
      isCommentAuthor = true;
    }
  }

  async function Delete() {
    await POST(
      "comment/delete",
      {
        ID: commentID,
        Token: await GetToken(),
      },
      false,
    );
    onDelete();
    deleted = true;
  }

  onMount(() => {
    InitCommentNode();
  });
</script>

{#if !deleted}
  <div
    style="width:200px;min-height:40px;max-height:80px;background-color:#2e2e2e;display:flex;margin:auto;overflow-y: scroll;"
  >
    <span
      style="margin-top:auto;margin-bottom:auto;text-align:center;margin-left:3px;"
    >
      <br />
    </span>
    <div style="align-items:left;text-align:left;">
      <span style="margin-left:0px;word-wrap: break-word;">
        <User ID={id}></User>
        <br />
        <span
          style="word-wrap:normal;word-break:normal;text-wrap:pretty;overflow-x:hidden;font-size:10px;line-height:1.4em;display: block;padding-left: 1px;padding-right: 3px;"
        >
          {content}
        </span>
      </span>
    </div>
    <br />
  </div>

  <div
    style="width:100%;height:20px;background-color: black;background-color: rgb(35 35 35);"
  >
    {#if isCommentAuthor}
      <button
        on:click={Delete}
        class="hyperlinkbutton"
        style="font-size:10px;color:red;float:right;margin-left:5px;margin-top:4px;"
        >Delete</button
      >
    {/if}

    <button
      on:click={Delete}
      class="hyperlinkbutton"
      style="font-size:10px;float:right;margin-top: 3px;">Report</button
    >

    <span style="font-size:6px;">Sent on: {date}</span>
  </div>
{/if}
