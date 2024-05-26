<svelte:options accessors/>

<script>
  import { SetData } from "../library/datatransfer";
  import { POST } from "../library/networking";


  let name;
  let pfp;
  let comment;
  let date = "";
  let accountid;
  let isCommentAuthor = false
  export let onDelete;
  let commentid;
  let pageid;
  let color;

  export function InitCommentNode(_comment, _pfp, _name, _accountid, _commentid, _localid, _pageid, _color)
  {
    name = _name;
    color = _color;
    pfp = _pfp;
    comment = _comment;
    accountid = _accountid;
    pageid = _pageid;
    commentid = _commentid;
    isCommentAuthor = _localid == _accountid;
    let timestamp = parseInt(_commentid)
    let d = new Date(timestamp);
    date = d.toLocaleString();
  }

  function OpenProfile(){
      SetData("profile_id", accountid);
      window.open("#/profilepage", "_self");
  } 

  async function Delete(){
      await POST("deletecomment", {commentid: commentid, pageid: pageid})
      onDelete()
  }
</script>

<div style="width:50%;height:100px;background-color:#2e2e2e;border-radius:10px;display:flex;padding:3px;filter: drop-shadow(1px 1px 3px black);margin:auto;">
    <span style="margin-top:auto;margin-bottom:auto;text-align:center;margin-left:10px;">
        <img src={pfp} alt="" style="width:60px;height:60px;border-radius:100%;filter: drop-shadow(1px 1px 3px black);">
        <br>
        <button on:click={OpenProfile} style="color:{color}" class="hyperlinkbutton">{name}</button>
    </span>
    <div style="align-items:left;text-align:left;">
        <span style="margin-left:20px;">
        {#if isCommentAuthor}
        <button on:click={Delete} class="hyperlinkbutton" style="font-size:10px;color:red;">Delete</button>
        <span style="font-size:10px;"> | </span>
        {/if}
        <span style="font-size:10px;">Sent on: {date}</span>
        <br>
        <div style="margin-left:20px;">
            <span style="text-align:left;word-wrap: break-word;width:50%;">
                {comment}
            </span>
        </div>
    </div>
    
    <br>
</div>
<p></p>
